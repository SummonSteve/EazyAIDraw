use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DrawResult {
    task_id: i64,
    position_in_queue: i32,
    status: TaskStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TaskStatus {
    Pending,
    Finished,
    Failed,
}

impl DrawResult {
    pub fn new(task_id: i64, position_in_queue: i32, status: TaskStatus) -> Self {
        Self {
            task_id,
            position_in_queue,
            status,
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DrawProgress {
    task_id: i64,
    current_step: u16,
    total_step: u16,
}

impl DrawProgress {
    pub fn new(task_id: i64, current_step: u16, total_step: u16) -> Self {
        Self {
            task_id,
            current_step,
            total_step,
        }
    }
}
