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

type FuncHashFile = fn(&Path) -> Option<HashLine>;

pub fn run(hash_file: FuncHashFile) {
    let args = Args::parse();

    if args.check {
        fn_check(hash_file, args.item)
    } else {
        fn_hash(hash_file, args.item)
    }
}

fn fn_check(hash_file: FuncHashFile, items: Vec<String>) {
    for item in items {
        let Some(lines) = parse(&item) else {
            continue;
        };
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

fn fn_hash(hash_file: FuncHashFile, items: Vec<String>) {
    for item in items {
        let Ok(paths) = glob(&item) else {
            continue;
        };
        for path in paths {
            let Ok(path) = path else {
                continue;
            };
            if let Some(line) = hash_file(&path) {
                println!("{}", line)
            }
        }
    }
}
