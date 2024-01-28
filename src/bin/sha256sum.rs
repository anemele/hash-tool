use hash::cli;
use hash::sha256::hash_file;

fn main() {
    cli::run(hash_file)
}
