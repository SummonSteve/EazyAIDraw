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

pub static API_POOL: LazyLock<Arc<Mutex<ApiPool>>> = LazyLock::new(|| {
    info!("Initializing API pool");
    Arc::new(Mutex::new(ApiPool::new()))
});

pub struct ApiPool {
    pub novelai_pool: HashMap<u8, Mutex<String>>,
    pub sd_pool: HashMap<u8, Mutex<String>>
}

impl ApiPool {
    pub fn new() -> Self {
        let mut novelai_pool = HashMap::new();
        let mut sd_pool = HashMap::new();
        if dotenv!("NOVEL_AI_API").len() > 0 {
            novelai_pool.insert(0, Mutex::new(dotenv!("NOVEL_AI_API").to_string()));
        }
        if dotenv!("SD_API").len() > 0 {
            sd_pool.insert(0, Mutex::new(dotenv!("SD_API").to_string()));
        }
        
        Self {
            novelai_pool,
            sd_pool
        }
    }

}