use std::fs::create_dir_all;
use std::{fs, path};
use std::path::Path;
use reqwest::Client;

pub enum ImageType {
    Jpg,
    Jpeg,
    Png,
    Webp,
    Bmp,
    Gif
}

impl ImageType {
    /// 从字节数组检测图片类型
    pub fn detect_from_bytes(bytes: &[u8]) -> Option<Self> {
        // 使用 image crate 检测格式
        match image::guess_format(bytes) {
            Ok(image::ImageFormat::Jpeg) => Some(ImageType::Jpeg),
            Ok(image::ImageFormat::Png) => Some(ImageType::Png),
            Ok(image::ImageFormat::WebP) => Some(ImageType::Webp),
            Ok(image::ImageFormat::Bmp) => Some(ImageType::Bmp),
            Ok(image::ImageFormat::Gif) => Some(ImageType::Gif),
            _ => None,
        }
    }

    /// 转换为 image crate 的 ImageFormat
    pub fn to_image_format(&self) -> image::ImageFormat {
        match self {
            ImageType::Jpg | ImageType::Jpeg => image::ImageFormat::Jpeg,
            ImageType::Png => image::ImageFormat::Png,
            ImageType::Webp => image::ImageFormat::WebP,
            ImageType::Bmp => image::ImageFormat::Bmp,
            ImageType::Gif => image::ImageFormat::Gif,
        }
    }
}

pub async fn download_image(url: String, output_path: String, output_format: ImageType) -> Result<(), Box<dyn std::error::Error>>{
    let client = Client::new();

    let response = client.get(url).send().await?;

    let image_bytes = response.bytes().await?;

    let image = image::load_from_memory(&image_bytes)?;

    let path = Path::new(output_path.as_str());
    if !path.exists() {
        if let Some(parent) = path.parent() {
            create_dir_all(parent).unwrap();
        }
    }

    image.save_with_format(output_path, output_format.to_image_format())?;

    Ok(())
}

#[tokio::test]
pub async fn test() {
    download_image(
        "https://mmbiz.qpic.cn/sz_mmbiz_jpg/ylbPx3dqvY9uNyNjsvicFaicmBrIxPGnEd58nYs9r7MbusMyOsbzkgichKvfkYyUbgwR08qDf9BJV8C4n28dBOthg/640?wx_fmt=jpeg".to_string(),
        "tmp.png".to_string(),
        ImageType::Png
    ).await.expect("TODO: panic message");
}