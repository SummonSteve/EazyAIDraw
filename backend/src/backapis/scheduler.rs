use super::{
    backend::{Backend, Backends},
    request_executor::CallExecutor,
    DrawTask, TaskMessage,
};
use crate::session::packets::draw_result::TaskStatus;
use crate::session::packets::draw_result::{DrawProgress, DrawResult};
use color_eyre::Result;
use crossbeam::channel::Receiver;
use dashmap::DashMap;
use dotenv_codegen::dotenv;
use std::sync::Arc;
use tokio::time::{self, Duration};
use tokio_tungstenite::tungstenite::Message;
use tracing::{info, warn};

pub struct TaskScheduler {
    task_recv: Receiver<TaskMessage>,
    http_client: Arc<reqwest::Client>,
    tasks: DashMap<i64, DrawTask>,
    backends: Backends,
    executor: CallExecutor,
}

impl TaskScheduler {
    pub fn new(task_recv: Receiver<TaskMessage>) -> Self {
        let sd_backend = Backend::new(super::BackendType::Sd, dotenv!("SD_API").to_string());
        let nai_backend = Backend::new(
            super::BackendType::NovelAi,
            dotenv!("NOVEL_AI_API").to_string(),
        );

        Self {
            task_recv,
            http_client: Arc::new(reqwest::Client::new()),
            tasks: DashMap::new(),
            backends: Backends::new(sd_backend, nai_backend),
            executor: CallExecutor::new(),
        }
    }

    pub fn start(mut self) -> Result<()> {
        self.executor.start();
        let mut watch_interval = time::interval(Duration::from_secs(1));
        let mut task_interval = time::interval(Duration::from_millis(500));
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = task_interval.tick() => {
                        for mut task in self.tasks.iter_mut() {
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
                                if let Ok(request) = task.body.into_http_request(self.http_client.as_ref(), &mut self.backends){
                                    self.executor.add_request(request, task.owner.clone());
                                    task.started = true;

                                }
                            }
                        }
                        self.tasks.retain(|_, task| !task.completed);

                        if let Ok(task_msg) = self.task_recv.try_recv() {
                            match task_msg {
                                TaskMessage::TaskSyncStatus(status) => {
                                    let id = status.id;
                                    let mut task = self.tasks.get_mut(&id).unwrap();
                                    if status.current == status.total {
                                        task.completed = true;
                                    }
                                    task.current_step = status.current;
                                },
                                TaskMessage::Task(task) => {
                                    info!("Received task {}", task.id);
                                    self.tasks.insert(task.id, task);
                                }
                            }
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
                            }
                        }
                    }
                }
            }
        });
        Ok(())
    }
}
