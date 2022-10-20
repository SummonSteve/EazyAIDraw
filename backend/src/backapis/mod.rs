pub mod structures;
pub mod scheduler;
pub mod backend;


use crossbeam::channel::{unbounded, Sender, Receiver};
use tokio_tungstenite::tungstenite::Message;


use self::structures::DrawCall;
#[derive(PartialEq, Eq)]
pub enum BackendType{
    NovelAi,
    // stable diffusion web ui
    Sd
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

