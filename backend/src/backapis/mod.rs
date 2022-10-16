pub mod structures;
pub mod novelai;

use std::sync::Arc;

use crossbeam::channel::{unbounded, Sender, Receiver};
use tokio_tungstenite::tungstenite::Message;
use color_eyre::Result;

use self::structures::DrawCall;

enum BackendType{
    NovelAi,
    // stable diffusion web ui
    Sd
}

pub struct DrawTask {
    id: u32,
    api_type: BackendType,
    api_url: String,
    owner: Sender<Message>,
    completed: bool,
    body: Box<dyn DrawCall + Send + Sync>,
    result: Option<String>,
    
}

impl DrawTask {
    pub fn new(id: u32, api_url: String, owner: Sender<Message>, body: Box<dyn DrawCall + Send + Sync>) -> Self {
        Self {
            id,
            api_type: BackendType::NovelAi,
            api_url,
            owner,
            completed: false,
            body,
            result: None,
        }
    }
}

pub struct DrawCaller {
    task_port: (Sender<DrawTask>, Receiver<DrawTask>),
    http_client: Arc<reqwest::Client>,
}

impl DrawCaller {
    pub fn new() -> Self {
        Self {
            task_port: unbounded(),
            http_client: Arc::new(reqwest::Client::new()),
        }
    }

    pub fn add_task(&mut self, task: DrawTask) {
        self.task_port.0.send(task).unwrap();
    }

    pub async fn start(&self) -> Result<()>{
        let rx = self.task_port.1.clone();
        let client = self.http_client.clone();
        tokio::spawn(async move {
            while let Ok(task) = rx.recv() {
                let client = client.clone();
                tokio::spawn(async move {
                    let res = task.body.into_http_request(&client, task.api_url).send().await.unwrap();

                });
                
            }
        });
        Ok(())
    }
}