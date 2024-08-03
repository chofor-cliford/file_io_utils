use file_io_utils::{read_file, write_file, append_file};

fn main() {
    let test_file = "test_files/example.txt";

    // Ensure the test directory exists
    std::fs::create_dir_all("test_files").unwrap();

    // Write to the file
    if let Err(e) = write_file(test_file, "Hello, File IO Utils!") {
        eprintln!("Failed to write to file: {}", e);
        return;
    }

    // Append to the file
    if let Err(e) = append_file(test_file, "\nAppending some content.") {
        eprintln!("Failed to append to file: {}", e);
        return;
    }

    // Read the file
    match read_file(test_file) {
        Ok(content) => println!("File content:\n{}", content),
        Err(e) => eprintln!("Failed to read file: {}", e),
    }
}
