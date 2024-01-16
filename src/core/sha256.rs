use std::path::Path;

use sha256::try_digest;

use crate::types::HashLine;

pub fn hash_file(path: &Path) -> Option<HashLine> {
    match try_digest(path) {
        Ok(d) => Some(HashLine {
            hash: d,
            file: format!("{}", path.display()),
        }),
        Err(_) => None,
    }
}

pub fn check(line: &HashLine) -> bool {
    match hash_file(Path::new(&line.file)) {
        Some(d) => d.hash == line.hash,
        None => false,
    }
}
