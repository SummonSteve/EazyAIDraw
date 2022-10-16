#![feature(once_cell)]

use color_eyre::Result;
use dotenv_codegen::dotenv;

mod session;
mod apis;
mod glob;
mod backapis;

use session::websocket::Handler;
use glob::{DB, API_POOL};

#[tokio::main]
async fn main() -> Result<()>{
    tracing_subscriber::fmt::init();
    let _db = DB.clone();
    let _api_pool = API_POOL.clone();

    Handler::new().start_service(dotenv!("PORT").parse()?).await?;

    Ok(())
}
