use dotenv_codegen::dotenv;
use pollster::FutureExt;
use sea_orm::{Database, DatabaseConnection};
use std::sync::Arc;
use std::sync::{LazyLock, Mutex};
use tracing::info;

use crate::backapis::TaskSchedulerStatus;

pub static DB: LazyLock<Arc<DatabaseConnection>> = LazyLock::new(|| {
    info!("Connecting to database");
    Arc::new(
        Database::connect(dotenv!("DATABASE_URL"))
            .block_on()
            .unwrap(),
    )
});

pub static STATUS: LazyLock<Arc<Mutex<TaskSchedulerStatus>>> = LazyLock::new(|| {
    Arc::new(Mutex::new(TaskSchedulerStatus {
        task_in_queue: 0,
        current_tasks: Vec::new(),
    }))
});
