use std::{
    fs::{self, File},
    io::BufReader,
    path::{Path, PathBuf},
    sync::{Arc, OnceLock},
};

use anyhow::Result;
use regex::Regex;
use serde::{de::DeserializeOwned, Deserialize};
use tokio::{
    sync::{mpsc::Sender, Semaphore},
    task::JoinSet,
};
use zstd::stream::read::Decoder;

use crate::{args::Args, Error, InsertPostRef};

static RE: OnceLock<Regex> = OnceLock::new();

#[derive(Deserialize)]
struct Comment {
    created_utc: i32,
    subreddit: String,
    author: String,
    body: String,
}

impl From<Comment> for Option<InsertPostRef> {
    fn from(value: Comment) -> Self {
        let subreddits_to = refs(value.body.as_str());
        if subreddits_to.is_empty() {
            None
        } else {
            Some(InsertPostRef {
                is_submission: false,
                author: value.author,
                subreddit_from: qualify_subreddit(value.subreddit.as_str()),
                created_utc: value.created_utc,
                subreddits_to,
            })
        }
    }
}

#[derive(Deserialize)]
struct Submission {
    created_utc: i32,
    subreddit: String,
    author: String,
    selftext: Option<String>,
}

impl From<Submission> for Option<InsertPostRef> {
    fn from(value: Submission) -> Self {
        if let Some(selftext) = value.selftext {
            let subreddits_to = refs(selftext.as_str());
            if subreddits_to.is_empty() {
                None
            } else {
                Some(InsertPostRef {
                    is_submission: true,
                    author: value.author,
                    created_utc: value.created_utc,
                    subreddit_from: qualify_subreddit(value.subreddit.as_str()),
                    subreddits_to,
                })
            }
        } else {
            None
        }
    }
}

struct FileTask {
    is_submission: bool,
    path: PathBuf,
}

fn files_in_dir<P>(path: P) -> Result<Vec<PathBuf>>
where
    P: AsRef<Path>,
{
    let files = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(files)
}

fn refs(text: &str) -> Vec<String> {
    RE.get()
        .unwrap()
        .captures_iter(text)
        .map(|c| c.get(0).unwrap().as_str().to_string())
        .collect()
}

fn qualify_subreddit(subreddit: &str) -> String {
    if subreddit.starts_with("u_") {
        let mut buf = String::with_capacity(subreddit.len());
        buf.push_str("u/");
        buf.push_str(&subreddit[2..]);
        buf
    } else {
        let mut buf = String::with_capacity(subreddit.len() + 2);
        buf.push_str("r/");
        buf.push_str(subreddit);
        buf
    }
}

pub fn start(
    args: &Args,
    handles: &mut JoinSet<Result<()>>,
    sender: Sender<InsertPostRef>,
) -> Result<()> {
    RE.set(Regex::new(r"\b[ru]\/[A-Za-z0-9_\-]{1,20}\b")?)
        .unwrap();

    let path = Path::new(args.input.as_str());
    let mut files = files_in_dir(path.join("submissions/"))?
        .into_iter()
        .map(|path| FileTask {
            is_submission: true,
            path,
        })
        .collect::<Vec<_>>();
    files.extend(
        files_in_dir(path.join("comments/"))?
            .into_iter()
            .map(|path| FileTask {
                is_submission: false,
                path,
            })
            .collect::<Vec<_>>(),
    );

    let semaphore = Arc::new(Semaphore::new(10));

    for task in files {
        let semaphore = semaphore.clone();
        let sender = sender.clone();
        handles.spawn(async move {
            let _permit = semaphore.acquire().await?;
            if task.is_submission {
                file_task::<_, Submission>(&task.path, sender).await
            } else {
                file_task::<_, Comment>(&task.path, sender).await
            }?;
            Ok(())
        });
    }

    Ok(())
}

async fn file_task<P, D>(path: P, write: Sender<InsertPostRef>) -> Result<()>
where
    P: AsRef<Path>,
    D: DeserializeOwned + Into<Option<InsertPostRef>>,
{
    let file = File::open(path.as_ref())?;
    let reader = BufReader::new(file);
    let mut decoder = Decoder::new(reader)?;
    decoder.window_log_max(31)?;
    let stream = serde_json::Deserializer::from_reader(decoder).into_iter::<D>();

    for entry in stream {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                log::error!("{}: {}", &path.as_ref().as_os_str().to_str().unwrap(), e);
                continue;
            }
        };
        if let Some(insert_data) = entry.into() {
            write
                .send(insert_data)
                .await
                .map_err(|_| Error::SendError)?;
        }
    }

    Ok(())
}
