use std::fs::File;
use std::io::Read;

pub fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_existing_file() {
        let file_path = "Cargo.toml";
        match read_file(file_path) {
            Ok(contents) => {
                assert!(!contents.is_empty(), "File should not be empty");
            }
            Err(e) => assert!(false, "Failed to read file: {}", e),
        }
    }

    #[test]
    fn test_read_file_non_existing_file() {
        let file_path = "non_existing_file.txt";
        match read_file(file_path) {
            Ok(_) => assert!(false, "Expected an error for non-existing file"),
            Err(e) => assert_eq!(e.kind(), std::io::ErrorKind::NotFound),
        }
    }
}
