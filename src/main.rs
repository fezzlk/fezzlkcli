use structopt::StructOpt;
use anyhow::{Context, Result};
use std::io::{stdout, Write};
use curl::easy::Easy;

#[derive(StructOpt)]
struct Args {
    command: String,
    target: String,
}

fn main() -> Result<()> {
    // 引数の取得
    let args = Args::from_args();

    let mut easy = Easy::new();
    
    easy.url("https://www.rust-lang.org/").unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();

    println!("{}", easy.response_code().unwrap());

    println!("command: {}", &args.command);
    println!("target: {}", &args.target);
    Ok(())
}
