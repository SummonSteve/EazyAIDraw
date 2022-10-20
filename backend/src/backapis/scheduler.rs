use super::{
    backend::{Backend, Backends},
    DrawTask,
};
use crate::session::packets::draw_result::TaskStatus;
use crate::session::packets::draw_result::{DrawProgress, DrawResult};
use color_eyre::Result;
use crossbeam::channel::{unbounded, Receiver, Sender};
use dotenv_codegen::dotenv;
use poem::http::request;
use std::sync::{Arc, Mutex};
use tokio::time::{self, sleep, Duration};
use tokio_tungstenite::tungstenite::Message;
use tracing::{info, warn};

pub struct TaskScheduler {
    task_recv: Receiver<DrawTask>,
    http_client: Arc<reqwest::Client>,
    tasks: Vec<DrawTask>,
    backends: Backends,
}

impl TaskScheduler {
    pub fn new(task_recv: Receiver<DrawTask>) -> Self {
        let sd_backend = Backend::new(super::BackendType::Sd, dotenv!("SD_API").to_string());
        let nai_backend = Backend::new(
            super::BackendType::NovelAi,
            dotenv!("NOVEL_AI_API").to_string(),
        );

        Self {
            task_recv,
            http_client: Arc::new(reqwest::Client::new()),
            tasks: vec![],
            backends: Backends::new(sd_backend, nai_backend),
        }
    }

    pub fn start(mut self) -> Result<()> {
        let mut watch_interval = time::interval(Duration::from_secs(1));
        let mut task_interval = time::interval(Duration::from_millis(100));
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = task_interval.tick() => {
                        for task in self.tasks.iter_mut() {
                            if task.completed {
                                info!("Task {} completed", task.id);
                                task.owner
                                    .try_send(Message::Text(
                                        serde_json::to_string(&DrawResult::new(
                                            task.id,
                                            0,
                                            TaskStatus::Finished,
                                        ))
                                        .unwrap(),
                                    ))
                                    .unwrap_or_else(|e|{warn!("{}",e)});
                            } else if !task.completed && !task.started {
                                if let Ok(request) = task.body.into_http_request(self.http_client.as_ref(), &self.backends){
                                    tokio::spawn(async move {
                                        request.send().await.unwrap();
                                    });
                                    task.started = true;

                                }
                            }
                        }
                        self.tasks.retain(|task| !task.completed);

                        if let Ok(task) = self.task_recv.try_recv() {
                            info!("Received task {}", task.id);
                            self.tasks.push(task);
                        }

                    }
                    _ = watch_interval.tick() => {
                        for task in self.tasks.iter(){
                            if !task.completed && task.started {
                                task.owner
                                    .try_send(Message::Text(
                                        serde_json::to_string(&DrawProgress::new(
                                            task.id,
                                            task.current_step,
                                            task.total_step,
                                        ))
                                        .unwrap()
                                    ))
                                    .unwrap_or_else(|e|{warn!("{}",e)});
                                    warn!("sending")
                            }
                        }
                    }
                }
            }
        });
        Ok(())
    }
}
