use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct HashLine {
    pub hash: String,
    pub file: String,
}

impl Display for HashLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}  {}", self.hash, self.file)
    }
}

pub type Lines = Vec<HashLine>;
