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

    const url: &str = "";
    const api_token: &str = "";
    const server_url: &str = "";

    // This will POST a body of `foo=bar&baz=quux`
    let params = [("url", url), ("apiTokenForExtension", api_token)];
    let client = reqwest::Client::new();
    let res = client.post(server_url)
        .form(&params)
        .send();

    // apiTokenForExtension
    println!("command: {}", &args.command);
    println!("target: {}", &args.target);
    Ok(())
}
