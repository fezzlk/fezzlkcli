extern crate dotenv;

#[macro_use]
extern crate dotenv_codegen;
use std::collections::HashMap;
use dotenv::dotenv;
use structopt::StructOpt;
use anyhow::{Context, Result};
use reqwest::{Client, header};

#[derive(StructOpt)]
struct Args {
    command: String,
    target: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // 引数の取得
    let args = Args::from_args();
    let command: &str = &args.command;
    let url: &str = &args.target;

    let api_token: &str = dotenv!("WEBEV_API_TOKEN");
    let server_url: &str = dotenv!("WEBEV_SERVER_URL");

    let mut map = HashMap::new();
    map.insert("url", url);
    map.insert("apiTokenForExtension", api_token);

    let client = Client::new();
    let response = client.post(server_url)
        .body("the exact body that is sent")
        .header(header::CONTENT_TYPE, "application_json")
        .json(&map)
        .send()
        .await?;

    println!("response: {:?}", response);
    Ok(())
}
