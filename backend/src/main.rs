#![feature(once_cell)]

use color_eyre::Result;
use dotenv_codegen::dotenv;
use poem::{endpoint::StaticFilesEndpoint, listener::TcpListener, Route, Server};

mod apis;
mod backapis;
mod glob;
mod session;
mod entities;
mod errors;

use glob::{DB};
use session::websocket::Handler;
use backapis::{scheduler::TaskScheduler, DrawTask, TaskMessage};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let _db = DB.clone();
    tokio::spawn(async move {
        let app = Route::new().nest("/", StaticFilesEndpoint::new("./www"));
        Server::new(TcpListener::bind("127.0.0.1:3000"))
            .run(app)
            .await.unwrap();
    });

    let (task_tx, task_rx) = crossbeam::channel::unbounded::<TaskMessage>();

    TaskScheduler::new(task_rx).start()?;


    Handler::new(task_tx)
        .start_service(dotenv!("PORT").parse()?)
        .await?;

    Ok(())
}
