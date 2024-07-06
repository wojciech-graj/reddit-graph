use std::str::FromStr;

use anyhow::Result;
use deadpool_postgres::{Manager, Pool, Runtime};
use tokio::{sync::mpsc::Receiver, task::JoinSet};
use tokio_postgres::{Config, NoTls};

use crate::args::Args;

pub struct InsertPostRef {
    pub is_submission: bool,
    pub author: String,
    pub created_utc: i32,
    pub subreddit_from: String,
    pub subreddits_to: Vec<String>,
}

pub fn start(
    args: &Args,
    handles: &mut JoinSet<Result<()>>,
    read: Receiver<InsertPostRef>,
) -> Result<()> {
    let postgres_cfg = Config::from_str(args.db.as_str())?;
    let manager = Manager::new(postgres_cfg, NoTls);
    let pool = Pool::builder(manager).runtime(Runtime::Tokio1).build()?;

    handles.spawn(async { db_task(pool, read).await });
    Ok(())
}

async fn db_task(pool: Pool, mut read: Receiver<InsertPostRef>) -> Result<()> {
    let client = pool.get().await?;
    while let Some(data) = read.recv().await {
        for subreddit_to in data.subreddits_to.iter() {
            crate::cornucopia::queries::post_refs::insert()
                .bind(
                    &client,
                    &data.author.as_str(),
                    &data.subreddit_from.as_str(),
                    &subreddit_to.as_str(),
                    &data.is_submission,
                    &data.created_utc,
                )
                .await?;
        }
    }
    Ok(())
}
