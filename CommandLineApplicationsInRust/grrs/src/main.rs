#![allow(unused)]

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    // 0 번째는 실행 되는 커맨드 라인 명령어다
    // clap 라이브러리를 쓰지 않으면 아래처럼 복잡함
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");

    // let args = Cli {
    //     pattern: pattern,
    //     path: std::path::PathBuf::from(path)
    // };

    let args = Cli::parse();

    let content = std::fs::read_to_string("test.txt")
        .with_context(|| format!("could not read file `{}`", "text.txt"))?;
    println!("file content: {}", content);
    Ok(())

    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }
}
