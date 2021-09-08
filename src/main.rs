use structopt::StructOpt;
use anyhow::{Context, Result};

#[derive(StructOpt)]
struct Args {
    command: String,
    target: String,
}

fn main() -> Result<()> {
    // 引数の取得
    let args = Args::from_args();

    println!("command: {}", &args.command);
    println!("target: {}", &args.target);
    Ok(())
}
