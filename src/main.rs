use structopt::StructOpt;
use anyhow::{Context, Result};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
#[derive(Debug)]

struct CustomError(String);

struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    // /// structoptを使って -o, --output の後の引数の型を指定する
    // #[structopt(short = "o", long = "output")]
    // outputOpt: String,
}

// fn main() -> Result<(), Box<dyn std::error::Error>> {
fn main() -> Result<(), CustomError> {
    // // 引数の取得（from_args()はstructoptが用意した関数?でmain()関数ないでしか使えない）
    // let args = Cli::from_args();

    // // ファイルの指定
    // let content = std::fs::read_to_string(&args.path)
    // .expect("could not read file");

    // // ファイルのロード
    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }
    // let result = std::fs::read_to_string("test.txt");
    // // let content = std::fs::read_to_string("test.txt").unwrap(); // shortcut
    // // let content = std::fs::read_to_string("test.txt")?; // more shortcut

    // match result { // nicer error handling
    //     Ok(content) => { content },
    //     Err(error) => { panic!("Can't deal with {}, just exit here", error); }
    //     // Err(error) => { return Err(error.into()); }
    // };
    let content = std::fs::read_to_string(path)
    // .map_err(|err| CustomError(format!("Error reading `{}`: {}", path, err)))?;
    .with_context(|| format!("could not read file `{}`", path))?;
    
    println!("file content: {}", content);
    Ok(())
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");
    // let args = Cli {
    //     pattern: pattern,
    //     path: std::path::PathBuf::from(path),
    // };
}

}