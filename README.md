# File IO Utils Library 📁✨

Welcome to `file_io_utils`! This Rust library simplifies file input/output operations, making it easy to read from, write to, and append content to files. Streamline your file handling with `file_io_utils`! 🚀

## Features 🌟

- **Read File**: Read a file and return its contents as a string.
- **Write File**: Write a string to a file.
- **Append File**: Append content to an existing file.

## Installation 📦

Add `file_io_utils` to your `Cargo.toml`:

```toml
[dependencies]
file_io_utils = "0.1.0"
```

## Usage 🛠️
Here's a quick example to get you started:

```
use file_io_utils::{read_file, write_file, append_file};

fn main() {
    let file_path = "example.txt";
    
    // Write to a file
    write_file(file_path, "Hello, File IO Utils!").expect("Failed to write to file");
    
    // Append to a file
    append_file(file_path, "\nAppending some content.").expect("Failed to append to file");
    
    // Read from a file
    let content = read_file(file_path).expect("Failed to read from file");
    println!("File Content:\n{}", content);
}
```

## Documentation 📚
For detailed documentation, run:

```
cargo doc --open
```

## Contributing 🤝
Contributions are welcome! If you have ideas for new features or improvements, feel free to open an issue or submit a pull request.

## License 📜
This project is licensed under the MIT License. See the LICENSE file for more details.

Enjoy using file_io_utils! If you like it, give it a ⭐ on GitHub.