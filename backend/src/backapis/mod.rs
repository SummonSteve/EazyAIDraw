pub mod structures;
pub mod scheduler;
pub mod backend;
pub mod request_executor;
use serde::{Serialize, Deserialize};


use crossbeam::channel::Sender;
use tokio_tungstenite::tungstenite::Message;


use crate::session::packets::BackendProgress;

use self::structures::DrawCall;
#[derive(PartialEq, Eq)]
pub enum BackendType{
    NovelAi,
    // stable diffusion web ui
    Sd
}

pub enum TaskMessage {
    TaskSyncStatus(BackendProgress),
    Task(DrawTask),
    RequestSchedulerStatus,
}

pub struct DrawTask {
    id: i64,
    api_type: BackendType,
    owner: Sender<Message>,
    total_step: u16,
    current_step: u16,
    started: bool,
    completed: bool,
    body: Box<dyn DrawCall + Send + Sync>,
    result: Option<String>,
    
}

impl DrawTask {
    pub fn new(id: i64, owner: Sender<Message>, total_step: u16, body: Box<dyn DrawCall + Send + Sync>) -> Self {
        Self {
            id,
            api_type: BackendType::NovelAi,
            owner,
            current_step: 0,
            started: false,
            total_step,
            completed: false,
            body,
            result: None,
        }
    }
}

#[derive(Serialize, Deserialize ,Debug)]
pub struct  TaskSchedulerStatus {
    pub task_in_queue: u32,
    pub current_tasks: Vec<i64>,
}