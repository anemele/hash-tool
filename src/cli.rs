use crate::{parse, HashLine};
use clap::Parser;
use glob::glob;
use std::path::Path;

#[derive(Parser, Debug)]
struct Args {
    #[arg(help = "file(s) if not specify --check, else line(s)", required = true)]
    item: Vec<String>,
    #[arg(help = "check hash sum", short, long, default_value_t = false)]
    check: bool,
}

type TypeHashFile = fn(&Path) -> Option<HashLine>;

pub fn cli(hash_file: TypeHashFile) {
    let args = Args::parse();

    if args.check {
        for item in args.item {
            if let Some(lines) = parse(&item) {
                for line in lines {
                    let check = match hash_file(Path::new(&line.file)) {
                        Some(d) => d.hash == line.hash,
                        None => false,
                    };
                    if check {
                        println!("{}: OK", line.file)
                    } else {
                        println!("{}: do NOT match", line.file)
                    }
                }
            }
        }
    } else {
        for item in args.item {
            if let Ok(entries) = glob(&item) {
                for entry in entries {
                    if let Ok(file) = entry {
                        if let Some(line) = hash_file(&file) {
                            println!("{}", line)
                        }
                    }
                }
            };
        }
    }
}
