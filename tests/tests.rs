use std::path::Path;

const THIS_DIR: &str = "tests";

#[test]
fn test_md5sum() {
    use hash::md5::hash_file;

    let empty = Path::new(THIS_DIR).join("empty");
    assert!(empty.exists());

    let ret = hash_file(&empty).unwrap();
    assert_eq!(ret.hash, "d41d8cd98f00b204e9800998ecf8427e".to_string())
}

#[test]
fn test_sha256sum() {
    use hash::sha256::hash_file;

    let empty = Path::new(THIS_DIR).join("empty");
    assert!(empty.exists());

    let ret = hash_file(&empty).unwrap();
    assert_eq!(
        ret.hash,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string()
    )
}
