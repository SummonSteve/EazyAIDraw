#![feature(once_cell)]

use color_eyre::Result;
use dotenv_codegen::dotenv;

mod apis;
mod backapis;
mod glob;
mod session;
mod entities;
mod errors;

use glob::{DB};
use session::websocket::Handler;
use backapis::{scheduler::TaskScheduler, TaskMessage};
use tracing::Level;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let _db = DB.clone();

    let (task_tx, task_rx) = crossbeam::channel::unbounded::<TaskMessage>();

    TaskScheduler::new(task_rx).start()?;

    Handler::new(task_tx)
        .start_service(dotenv!("PORT").parse()?)
        .await?;

    Ok(())
}
