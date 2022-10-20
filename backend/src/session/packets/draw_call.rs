use crossbeam::channel::Sender;
use sea_orm::{ConnectionTrait, Statement, EntityTrait, ActiveModelTrait, DatabaseConnection};
use tokio_tungstenite::tungstenite::Message;
use tracing::log::warn;

use crate::backapis::DrawTask;
use crate::backapis::structures::DrawCall;
use crate::glob::DB;
use crate::session::packets::DrawCallType;
use crate::entities::{
    public_data::generate_stream::{self, Entity as GenerateStreamEntity},
};


pub async fn handle_draw_call(draw_call: DrawCallType, message_tx: Sender<Message>, task_tx: Sender<DrawTask>) -> color_eyre::Result<i64>{
    let new_record = <generate_stream::ActiveModel as sea_orm::ActiveModelTrait>::default();

    let record = new_record.insert(DB.as_ref()).await?;

    match draw_call {
        DrawCallType::NovelAi(call) => {
            let task = DrawTask::new(record.id, message_tx, call.steps, Box::new(call));
            task_tx.send(task).unwrap_or_else(|e|{warn!("{}",e)})
        },
        DrawCallType::Sd(call) => {}
    }
    
    Ok(record.id)
}