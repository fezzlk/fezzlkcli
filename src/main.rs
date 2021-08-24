use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]

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

fn main() {
    // 引数の取得（from_args()はmain()関数ないでしか使えない）
    let args = Cli::from_args();

    // ファイルの指定
    let content = std::fs::read_to_string(&args.path)
    .expect("could not read file");

    // ファイルのロード
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");
    // let args = Cli {
    //     pattern: pattern,
    //     path: std::path::PathBuf::from(path),
    // };
}

