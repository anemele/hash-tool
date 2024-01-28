use super::HashLine;
use md5::{Digest, Md5};
use std::{fs::File, io::Read, path::Path};

const BUF_SIZE: usize = 4096;

pub fn hash_file(path: &Path) -> Option<HashLine> {
    let Ok(mut file) = File::open(path) else {
        eprintln!("Unable to open file: {}", path.display());
        return None;
    };

    let mut hasher = Md5::new();
    let mut buf = [0u8; BUF_SIZE];
    loop {
        let Ok(size) = file.read(&mut buf) else {
            return None;
        };
        if size < BUF_SIZE {
            hasher.update(&buf[..size]);
            break;
        } else {
            hasher.update(buf);
        }
    }

    let result = hasher.finalize();
    let mut hash = String::new();
    for byte in result[..].to_vec() {
        hash += &format!("{:02x}", byte);
    }

    Some(HashLine {
        hash,
        file: path.to_str().unwrap().to_string(),
    })
}
