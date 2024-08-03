use std::fs::File;
use std::io::{self, Read};

/// Reads the entire contents of a file and returns it as a `String`.
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path of the file to be read.
///
/// # Examples
///
/// ```rust
/// use file_io_utils::read_file;
///
/// let content = read_file("example.txt").expect("Failed to read file");
/// println!("{}", content);
/// ```
///
/// # Errors
///
/// This function will return an `io::Error` if the file cannot be opened or read.
pub fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    #[test]
    fn test_read_file() {
        // Ensure the test directory exists
        let test_dir = "test_files";
        fs::create_dir_all(test_dir).unwrap();

        let test_file_path = format!("{}/read_test.txt", test_dir);
        let mut test_file = File::create(&test_file_path).unwrap();
        writeln!(test_file, "This is a test file for reading.").unwrap();

        let result = read_file(&test_file_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "This is a test file for reading.\n");
    }
}
