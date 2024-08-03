use std::fs::OpenOptions;
use std::io::{self, Write};

/// Appends a string to the end of a file, creating the file if it does not exist.
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path of the file to be appended to.
/// * `contents` - The string slice that will be appended to the file.
///
/// # Examples
///
/// ```rust
/// use file_io_utils::append_file;
///
/// append_file("example.txt", "Appending some content.").expect("Failed to append to file");
/// ```
///
/// # Errors
///
/// This function will return an `io::Error` if the file cannot be opened or written to.
pub fn append_file(file_path: &str, contents: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)  // Create the file if it does not exist
        .open(file_path)?;
    file.write_all(contents.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_append_file() {
        // Ensure the test directory exists
        let test_dir = "test_files";
        fs::create_dir_all(test_dir).unwrap();

        let test_file_path = format!("{}/append_test.txt", test_dir);

        // First write to the file to make sure it exists
        use crate::write_file;
        write_file(&test_file_path, "Initial content.").unwrap();

        // Then append to it
        let result = append_file(&test_file_path, "\nAppending to a test file.");
        assert!(result.is_ok());

        let content = fs::read_to_string(&test_file_path).unwrap();
        assert_eq!(content, "Initial content.\nAppending to a test file.");
    }
}
