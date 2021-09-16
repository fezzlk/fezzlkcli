extern crate dotenv;

#[macro_use]
extern crate dotenv_codegen;
use std::collections::HashMap;
use dotenv::dotenv;
use structopt::StructOpt;
use anyhow::{Context, Result};
use reqwest::{Client, header};

// #[derive(StructOpt)]
// struct Args {
//     command: String,
//     target: String,
// }

#[tokio::main]
async fn main() -> Result<()> {
    // 引数の取得
    // let args = Args::from_args();
    let url: &str = "https://github.com/fezzlk";
    let api_token: &str = dotenv!("WEBEV_API_TOKEN");
    let server_url: &str = dotenv!("WEBEV_SERVER_URL");
    println!("api_token: {}", api_token);

    let mut map = HashMap::new();
    map.insert("url", url);
    map.insert("apiTokenForExtension", api_token);

    let client = Client::new();
    let response = client.post(server_url)
        .body("the exact body that is sent")
        // .form(&params)
        .header(header::CONTENT_TYPE, "application_json")
        .json(&map)
        // .body("{\"url\": \"https://github.com/fezzlk\", \"apiTokenForExtension\": \"U2FsdGVkX188kbM7uQ0Y1IIaELhYBcEIWpQojHmwvvCtQ4ODaDa/owCwHmM52pwC\"")
        .send()
        .await?;

    println!("response: {:?}", response);
    // println!("command: {}", &args.command);
    // println!("target: {}", &args.target);
    Ok(())
}
