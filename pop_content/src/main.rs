mod args;
mod cornucopia;
mod db_task;
pub mod error;
mod file_task;

use anyhow::Result;
use args::Args;
use clap::Parser;
use db_task::InsertPostRef;
pub use error::Error;
use log::LevelFilter;
use tokio::{sync::mpsc, task::JoinSet};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .init();

    let args = Args::parse();

    let (sender, receiver) = mpsc::channel::<InsertPostRef>(2048);

    let mut handles: JoinSet<Result<()>> = JoinSet::new();

    db_task::start(&args, &mut handles, receiver)?;
    file_task::start(&args, &mut handles, sender)?;

    let total_tasks = handles.len();
    while let Some(res) = handles.join_next().await {
        log::info!(
            "Progress: [{}/{}]",
            total_tasks - handles.len(),
            total_tasks,
        );
        res??;
    }

    Ok(())
}
