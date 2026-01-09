use chrono::Local;
use log::error;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::config::get_config_clone;
use crate::utils::download::{download_image, ImageType};

#[derive(Deserialize, Serialize, Debug)]
struct ResponseBody<T> {
    code: i16,
    message: String,
    data: T
}

#[derive(Deserialize, Serialize, Debug)]
struct News {
    date: String,
    news: Vec<String>,
    image: String,
    tip: String,
    cover: String
}

pub async fn get_news_response() -> Result<String, String>{
    let client = Client::new();
    let formatted = Local::now().format("%Y-%m-%d").to_string();
    let config = get_config_clone().await;
    let res = client
        .get(config.features.news.url)
        .query(&[("date", formatted.clone())])
        .send().await;
    match res {
        Ok(res) => {
            let text = res.json::<ResponseBody<News>>().await.unwrap();
            let output_path = format!("resources/{}.png", formatted);
            match download_image(text.data.image, output_path.clone(), ImageType::Png).await {
                Ok(_) => {
                    Ok(format!("https://rbot.yunnet.top/{}", output_path))
                }
                Err(err) => {
                    error!("Failed to download image: {}", err);
                    Err("".to_string())
                }
            }
        }
        Err(_) => {
            Err("Failed to get news response".to_string())
        }
    }
}

#[tokio::test]
pub async fn test() {
    let formatted = Local::now().format("%Y-%m-%d").to_string();
    println!("{}", formatted);
    println!("{}", get_news_response().await.unwrap());
}