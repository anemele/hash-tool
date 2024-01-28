use hash::md5;
use hash::sha256;
use std::path::Path;

const EMPTY: &str = "tests/empty";

#[test]
fn test_empty() {
    let empty = Path::new(EMPTY);
    assert!(empty.exists());

    let ret = md5::hash_file(&empty).unwrap();
    assert_eq!(ret.hash, "d41d8cd98f00b204e9800998ecf8427e");

    let ret = sha256::hash_file(&empty).unwrap();
    assert_eq!(
        ret.hash,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}
