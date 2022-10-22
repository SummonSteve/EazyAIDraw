use dotenv_codegen::dotenv;
use serde_json::Value;
use crossbeam::channel::{unbounded, Receiver, Sender};
use reqwest::{RequestBuilder, Response};
use tokio_tungstenite::tungstenite::Message;
use miniz_oxide::deflate::compress_to_vec_zlib;

pub struct CallExecutor {
    response_tx: Sender<(Response, Sender<Message>)>,
    response_rx: Receiver<(Response, Sender<Message>)>,
}

impl CallExecutor {
    pub fn new() -> Self {
        let (response_tx, response_rx) = unbounded::<(Response, Sender<Message>)>();
        Self {
            response_tx,
            response_rx,
        }
    }

    pub fn start(&self) {
        let rx = self.response_rx.clone();
        tokio::spawn(async move {
            while let Ok((response, sender)) = rx.recv() {
                handle_and_send(response, sender).await;
            }
        });
    }

    pub fn add_request(&self, request: RequestBuilder, sender: Sender<Message>) {
        let tx = self.response_tx.clone();
        tokio::spawn(async move {
            let http_response = request.send().await.unwrap();
            tx.send((http_response, sender)).unwrap();
        });
    }
}

async fn handle_and_send(response: Response, sender: Sender<Message>) {
    let body = response.text().await.unwrap();
    if let Ok::<Value, _>(nai_response) = serde_json::from_str(&body) {
    } else {
        let imgs_b64 = body
            .split("data:")
            .map(|_str| {
                _str.split("\n\n")
                    .filter(|part| !part.contains("event") && part.len() > 1)
                    .collect()
            })
            .filter(|part: &Vec<&str>| part.len() > 0)
            .collect::<Vec<Vec<&str>>>()
            .iter_mut()
            .map(|x| x.pop().unwrap())
            .collect::<Vec<&str>>();

            for img_b64 in imgs_b64 {
                let raw_img = base64::decode(img_b64).unwrap();
                sender.send(Message::Binary(compress_to_vec_zlib(&raw_img, 10))).unwrap();
            }

    }
}
