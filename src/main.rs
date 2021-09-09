extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen;

use dotenv::dotenv;
use structopt::StructOpt;
use anyhow::{Context, Result};
use reqwest::Client;

#[derive(StructOpt)]
struct Args {
    command: String,
    target: String,
}

fn main() -> Result<()> {
    // 引数の取得
    let args = Args::from_args();
    let url: &str = "https://github.com/fezzlk";
    let api_token: &str = dotenv!("WEBEV_API_TOKEN");
    let server_url: &str = dotenv!("WEBEV_SERVER_URL");

    let params = [("url", url), ("apiTokenForExtension", api_token)];
    let client = reqwest::Client::new();
    let res = client.post(server_url)
        .form(&params)
        .send();

    println!("command: {}", &args.command);
    println!("target: {}", &args.target);
    Ok(())
}
