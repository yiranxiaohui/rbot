use log::error;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::json;
use uuid::Uuid;
use crate::qq::token::{get_access_token_clone};
use crate::utils::request::get_client;

#[derive(Debug, Clone, Copy)]
pub enum  FileType {
    Image = 1,
    Video = 2,
    Silk = 3
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct File {
    pub file_uuid: String,
    pub file_info: String,
    pub ttl: i16,
    pub id: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Req {
    pub file_type: i16,
    pub url: String,
    pub srv_send_msg: bool
}

impl Serialize for FileType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}

pub async fn upload_group_file(group_openid: String, file_url: String, file_type: FileType) -> Result<File, String> {
    let client = get_client().await;
    let url = format!("https://api.sgroup.qq.com/v2/groups/{group_openid}/files");
    println!("url: {}", url);
    println!("file_url: {}", file_url);
    let body = json!({
            "file_type": 1,
            "url": file_url,
            "srv_send_msg": false
        });
    let res = client.post(url)
        .header("Authorization", format!("Bearer {}", get_access_token_clone().await))
        .body(body.to_string())
        .send().await;
    match res {
        Ok(res) => {
            let res = res.text().await.unwrap();
            error!("{:#?}", res);
            match serde_json::from_str(&res) {
                Ok(ok) => {
                    Ok(ok)
                }
                Err(err) => {
                    error!("{:?}", err);
                    Err(err.to_string())
                }
            }
        }
        Err(err) => {
            error!("{:?}", err);
            Err(err.to_string())
        }
    }
}

#[tokio::test]
pub async fn test() {
    println!("{}", &json!({
        "file_type": FileType::Image
    }))
}