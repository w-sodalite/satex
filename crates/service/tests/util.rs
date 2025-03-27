use std::path::PathBuf;

pub fn test_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/")
}

pub fn test_file_dir() -> PathBuf {
    test_dir().join("file")
}
