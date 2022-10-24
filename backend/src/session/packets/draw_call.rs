use crossbeam::channel::Sender;
use sea_orm::ActiveModelTrait;
use tokio_tungstenite::tungstenite::Message;
use tracing::info;

use crate::backapis::{DrawTask, TaskMessage};
use crate::glob::DB;
use crate::session::packets::DrawCallType;
use crate::entities::{
    public_data::generate_stream::{self, Entity as GenerateStreamEntity},
};


pub async fn handle_draw_call(draw_call: DrawCallType, message_tx: Sender<Message>, task_tx: Sender<TaskMessage>) -> color_eyre::Result<()>{

    match draw_call {
        DrawCallType::NovelAi(call) => {
            info!("samples: {}", call.n_samples);
            for _ in 0..call.n_samples {
                info!("generated task");
                let mut _call = call.clone();
                let new_record = <generate_stream::ActiveModel as sea_orm::ActiveModelTrait>::default();
                let record = new_record.insert(DB.as_ref()).await?;
                let tx = message_tx.clone();
                _call.id = record.id;
                _call.n_samples = 1;

                let task = DrawTask::new(record.id, tx, _call.steps, Box::new(_call));
                task_tx.send(TaskMessage::Task(task)).unwrap()
            }
        },
        DrawCallType::Sd(call) => {}
    }
    
    Ok(())
}