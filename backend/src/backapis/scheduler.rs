use super::{
    backend::{Backend, Backends},
    request_executor::CallExecutor,
    DrawTask, TaskMessage, TaskSchedulerStatus,
};
use crate::glob::STATUS;
use crate::session::packets::draw_result::TaskStatus;
use crate::session::packets::draw_result::{DrawProgress, DrawResult};
use color_eyre::Result;
use crossbeam::channel::Receiver;
use dotenv_codegen::dotenv;
use indexmap::IndexMap;
use std::sync::Arc;
use tokio::time::{self, Duration};
use tokio_tungstenite::tungstenite::Message;
use tracing::{info, warn};
use crate::session::packets::{OutcomePacket, OutcomePacketType};

pub struct TaskScheduler {
    task_recv: Receiver<TaskMessage>,
    http_client: Arc<reqwest::Client>,
    tasks: IndexMap<i64, DrawTask>,
    backends: Backends,
    executor: CallExecutor,
    current_tasks: Vec<i64>,
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
            tasks: IndexMap::new(),
            backends: Backends::new(sd_backend, nai_backend),
            executor: CallExecutor::new(),
            current_tasks: Vec::new(),
        }
    }

    pub fn start(mut self) -> Result<()> {
        self.executor.start();
        let mut watch_interval = time::interval(Duration::from_millis(153));
        let mut task_interval = time::interval(Duration::from_millis(100));
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = task_interval.tick() => {
                        for (_, mut task) in self.tasks.iter_mut() {
                            if task.completed {
                                info!("Task {} completed", task.id);
                                for mut backend in &mut self.backends.inner {
                                    if backend.task == Some(task.id) {
                                        backend.task = None;
                                    }
                                }
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
                                    info!("Starting task {}", task.id);
                                    task.owner.clone().try_send(Message::Text(serde_json::to_string(&DrawResult::new(task.id, 0, TaskStatus::Pending)).unwrap())).unwrap();
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
                                    if let Some(mut task) = self.tasks.get_mut(&id) {
                                        if status.current == status.total {
                                            task.completed = true;
                                        }
                                        task.current_step = status.current;
                                    }

                                },
                                TaskMessage::Task(task) => {
                                    info!("Current task in stack: {}", self.tasks.len());
                                    info!("Received task {}", task.id);
                                    task.owner.clone().try_send(Message::Text(serde_json::to_string(&DrawResult::new(task.id, self.tasks.len(), TaskStatus::Pending)).unwrap())).unwrap();
                                    self.tasks.insert(task.id, task);
                                },
                                TaskMessage::RequestSchedulerStatus => {
                                    let now_status = TaskSchedulerStatus{
                                        task_in_queue: self.tasks.len() as u32,
                                        current_tasks: self.current_tasks.clone(),
                                    };

                                    *STATUS.lock().unwrap() = now_status;
                                }
                            }
                        }

                    }
                    _ = watch_interval.tick() => {
                        for (_, task) in self.tasks.iter(){
                            if !task.completed && task.started {
                                let struct_str = serde_json::to_string(&DrawProgress::new(
                                    task.id,
                                    task.current_step,
                                    task.total_step,
                                )).unwrap();
                                let res = OutcomePacket::new(OutcomePacketType::DrawProgress, serde_json::from_str(&struct_str).unwrap());
                                
        
                                task.owner
                                    .try_send(Message::Text(serde_json::to_string(&res).unwrap()))
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
