use super::structures::novelai::GenerateStream;
use crate::glob::API_POOL;
use color_eyre::Result;





pub async fn call_draw(param: GenerateStream) -> Result<()> {
    let api = API_POOL
        .clone()
        .lock()
        .unwrap()
        .novelai_pool.get(&0)
        .unwrap()
        .try_lock()
        .unwrap()
        .to_string();

    let api = format!("{}{}", api, "/generate-stream");
    
    let client = reqwest::Client::new();
    let res = client.post(api)
        .json(&param)
        .send()
        .await?;

        println!("{:?}", res);

    Ok(())
}
