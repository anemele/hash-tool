use hash::{cli, sha256};

fn main() {
    cli(sha256::check, sha256::hash_file)
}
