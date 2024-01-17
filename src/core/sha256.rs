use crate::HashLine;
use sha256::try_digest;
use std::path::Path;

pub fn hash_file_sha256(path: &Path) -> Option<HashLine> {
    match try_digest(path) {
        Ok(d) => Some(HashLine {
            hash: d,
            file: format!("{}", path.display()),
        }),
        Err(_) => None,
    }
}
