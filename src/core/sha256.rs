use super::HashLine;
use sha256::try_digest;
use std::path::Path;

pub fn hash_file(path: &Path) -> Option<HashLine> {
    if let Ok(digest) = try_digest(path) {
        Some(HashLine {
            hash: digest,
            file: path.to_str().unwrap().to_string(),
        })
    } else {
        None
    }
}
