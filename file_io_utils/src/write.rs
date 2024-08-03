use std::fs::File;
use std::io::{self, Write};

/// Writes a string to a file, overwriting the file if it already exists.
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path of the file to be written to.
/// * `contents` - The string slice that will be written to the file.
///
/// # Examples
///
/// ```rust
/// use file_io_utils::write_file;
///
/// write_file("example.txt", "Hello, File IO Utils!").expect("Failed to write to file");
/// ```
///
/// # Errors
///
/// This function will return an `io::Error` if the file cannot be created or written to.
pub fn write_file(file_path: &str, contents: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(contents.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_write_file() {
        // Ensure the test directory exists
        let test_dir = "test_files";
        fs::create_dir_all(test_dir).unwrap();

        let test_file_path = format!("{}/write_test.txt", test_dir);
        let result = write_file(&test_file_path, "Writing to a test file.");
        assert!(result.is_ok());

        let content = fs::read_to_string(&test_file_path).unwrap();
        assert_eq!(content, "Writing to a test file.");
    }
}
