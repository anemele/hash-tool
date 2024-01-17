use hash::cli;
use hash::hash_file_sha256;
use std::{env, path::MAIN_SEPARATOR_STR};

fn get_self_name() -> String {
    let self_path = env::args().into_iter().next().unwrap();
    let self_name = match self_path.rsplit_once(MAIN_SEPARATOR_STR) {
        Some((_, name)) => name.to_string(),
        None => self_path,
    };
    if cfg!(windows) {
        match self_name.strip_suffix(".exe") {
            Some(s) => s.to_string(),
            None => self_name,
        }
    } else {
        self_name
    }
}

fn main() {
    match get_self_name().as_str() {
        "sha256sum" => cli(hash_file_sha256),
        s => {
            eprintln!("Not support: {}", s);
        }
    }
}
