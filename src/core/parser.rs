use super::types::{HashLine, Lines};
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    vec,
};

fn parse_line(line: &str) -> Option<HashLine> {
    let tmp: Vec<&str> = line.split_ascii_whitespace().collect();
    if tmp.len() == 2 {
        Some(HashLine {
            hash: tmp[0].to_string(),
            file: tmp[1].to_string(),
        })
    } else {
        None
    }
}

#[test]
fn test_parse_line() {
    assert_eq!(
        HashLine {
            hash: "123".to_string(),
            file: "abc".to_string()
        },
        parse_line("123 abc").unwrap()
    )
}

fn parse_text(info: &str) -> Lines {
    let lines: Vec<&str> = info.split("\n").collect();

    let mut ret = vec![];
    for line in lines {
        if let Some(hash_line) = parse_line(line) {
            ret.push(hash_line);
        }
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
    let path = path.as_ref();
    let Ok(file) = File::open(path) else {
        eprintln!("Unable to open file: {}", path.display());
        return None;
    };

    let mut ret = vec![];
    for result in io::BufReader::new(file).lines() {
        let Ok(line) = result else {
            continue;
        };
        if let Some(hash_line) = parse_line(&line) {
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
