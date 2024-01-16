use crate::types::{HashLine, Lines};
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    vec,
};

fn parse_text(info: &str) -> Lines {
    let lines: Vec<&str> = info.split("\n").collect();

    let mut ret = vec![];
    for line in lines {
        let tmp: Vec<&str> = line.split_ascii_whitespace().collect();
        if tmp.len() != 2 {
            continue;
        }
        let hash_line = HashLine {
            hash: tmp[0].to_string(),
            file: tmp[1].to_string(),
        };
        ret.push(hash_line);
    }
    ret
}

#[test]
fn test_parse_info() {
    let sample = "\
123456 abc
ABCD  666";
    let expected = vec![
        HashLine {
            hash: "123456".to_string(),
            file: "abc".to_string(),
        },
        HashLine {
            hash: "ABCD".to_string(),
            file: "666".to_string(),
        },
    ];
    let output = parse_text(sample);
    assert_eq!(output, expected)
}

fn parse_file<P>(path: P) -> Option<Lines>
where
    P: AsRef<Path>,
{
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("{}", e);
            return None;
        }
    };
    let mut ret = vec![];
    for result in io::BufReader::new(file).lines() {
        if let Ok(line) = result {
            let tmp: Vec<&str> = line.split_ascii_whitespace().collect();
            if tmp.len() != 2 {
                continue;
            }
            let hash_line = HashLine {
                hash: tmp[0].to_string(),
                file: tmp[1].to_string(),
            };
            ret.push(hash_line);
        }
    }
    Some(ret)
}

pub fn parse(item: &str) -> Option<Lines> {
    if Path::new(item).exists() {
        parse_file(item)
    } else {
        Some(parse_text(item))
    }
}
