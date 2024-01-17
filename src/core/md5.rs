use crate::HashLine;
use md5::{Digest, Md5};
use std::{fs::File, io::Read, path::Path};

const BUF_SIZE: usize = 4096;

pub fn hash_file_md5(path: &Path) -> Option<HashLine> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return None,
    };

    let mut hasher = Md5::new();
    let mut buf = [0u8; BUF_SIZE];
    loop {
        if let Ok(size) = file.read(&mut buf) {
            hasher.update(buf.as_slice());
            if size < BUF_SIZE {
                break;
            }
        } else {
            return None;
        };
    }

    let result = hasher.finalize();
    let mut hash = String::new();
    for byte in result[..].to_vec() {
        hash += &format!("{:02x}", byte);
    }
    Some(HashLine {
        hash,
        file: format!("{}", path.display()),
    })
}
