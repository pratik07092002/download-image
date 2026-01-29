use std::{ path::PathBuf};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use clap::Parser;
use reqwest::Client;
use url::Url;

#[derive(Parser, Debug)]
struct Args {
    image_url: String,
}
async fn download_image(download_url: String) -> Result<(), Box<dyn std::error::Error>> {
    let parsed_url = Url::parse(&download_url)?;

    let filename = parsed_url
        .path_segments()
        .and_then(|s| s.last())
        .filter(|s| !s.is_empty())
        .unwrap_or("download_image");

    let mut file_path = PathBuf::from(std::env::temp_dir());
    file_path.push(filename);
    

    let client = Client::new();
    let response = client.get(parsed_url).send().await?;

    if !response.status().is_success() {
        return Err(format!("Failed with status {}",response.status()).into());
    }


    let bytes = response.bytes().await?;

    let mut file = File::create(&file_path).await?;
    file.write_all(&bytes).await?;
   println!("Image saved at {:?}", file_path);

    Ok(())
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("ARGS ARE THIS {}", args.image_url);

    download_image(args.image_url).await?;

    Ok(())
}
