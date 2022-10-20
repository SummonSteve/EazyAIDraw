use std::{sync::{LazyLock, Mutex}, collections::HashMap};
use sea_orm::{DatabaseConnection, Database};
use std::sync::Arc;
use tracing::info;
use dotenv_codegen::dotenv;
use pollster::FutureExt;


pub static DB: LazyLock<Arc<DatabaseConnection>> = LazyLock::new(|| {
    info!("Connecting to database");
    Arc::new(Database::connect(dotenv!("DATABASE_URL")).block_on().unwrap())
});

