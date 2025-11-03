Below is a professional and detailed [README.md](./README.md) file for your Rust crate. It includes an overview of the functionality, installation instructions, usage examples, and additional notes.

---

# Search Crate

A flexible and efficient Rust library for searching text within files. Supports multiple search modes (substring and regex), optional case-insensitive searches, and writing results to an output file or printing them to the console.

---

## Features

- **Substring Search**: Perform simple substring searches in a file.
- **Regex Search**: Use regular expressions for advanced pattern matching.
- **Case-Insensitive Searches**: Optionally ignore case during searches.
- **Output Flexibility**: Write results to an output file or print them to the console.
- **Efficient File Handling**: Reads large files line by line using buffered I/O for optimal performance.

---

## Installation

### Prerequisites
- Rust 1.60 or higher.
- Cargo package manager.

### Add as a Dependency
If you want to use this crate in your project, add it to your [Cargo.toml](./Cargo.toml):

```toml
[dependencies]
search = { path = "path_to_search_crate" }
```

Alternatively, if the crate is published to [crates.io](https://crates.io), you can include it as:

```toml
[dependencies]
search = "0.2.0"
```

---

## Usage

### Command-Line Interface
The crate provides a command-line interface for performing searches. Below are examples of how to use it.

#### Substring Search
Perform a case-sensitive substring search and print results to the console:
```bash
cargo run -- search "Error" "./large_log_file.txt"
```

#### Case-Insensitive Regex Search
Perform a case-insensitive regex search and write results to an output file:
```bash
cargo run -- regex "error\d+" "./large_log_file.txt" "./output.txt" --ignore-case
```

#### Help Message
If insufficient arguments are provided, the program will display a usage message:
```bash
Usage: cargo run -- <mode> <search_string> <file_path> [output_file] [--ignore-case]
```

---

### Programmatic Usage
You can also use the crate programmatically in your Rust code. Here's an example:

```rust
use search::search_in_file;

fn main() -> std::io::Result<()> {
    // Perform a case-insensitive regex search and print results to the console
    search_in_file(
        "./large_log_file.txt", // File path
        "error\\d+",           // Query (regex pattern)
        None,                  // Output file (None means print to console)
        true,                  // Ignore case
        true                   // Treat query as regex
    )
}
```

---

## Arguments

The program accepts the following arguments:

| Argument         | Description                                                                 |
|-------------------|-----------------------------------------------------------------------------|
| `<mode>`         | The search mode (`search` for substring, `regex` for regular expressions).   |
| `<search_string>`| The string or regex pattern to search for in the file.                      |
| `<file_path>`    | The path to the file to search.                                             |
| `[output_file]`  | (Optional) The path to the file where results will be written.              |
| `[--ignore-case]`| (Optional) If provided, the search will ignore case.                        |

---

## Examples

### Example 1: Substring Search
Search for the word `"Error"` in `large_log_file.txt` and print results to the console:
```bash
cargo run -- search "Error" "./large_log_file.txt"
```

### Example 2: Case-Insensitive Regex Search
Search for lines matching the regex pattern `"error\d+"` in `large_log_file.txt`, ignoring case, and write results to `output.txt`:
```bash
cargo run -- regex "error\d+" "./large_log_file.txt" "./output.txt" --ignore-case
```

### Example 3: Invalid Regex Pattern
If the regex pattern is invalid, the program will notify you:
```bash
cargo run -- regex "[" "./large_log_file.txt"
Invalid regex pattern '['
```

---

## Error Handling

The program gracefully handles errors such as:
- Missing or invalid arguments.
- File opening or reading errors.
- Invalid regex patterns.

Example error messages:
```bash
Error: No such file or directory (os error 2)
Invalid regex pattern '['
```

---

## Notes

1. **Performance**: The crate reads files line by line using buffered I/O, making it suitable for large files.
2. **Dependencies**: The crate uses the [`regex`](https://docs.rs/regex/latest/regex/) crate for regex support.
3. **Extensibility**: Additional search modes or features can be added in the future.

---

## Contributing

Contributions are welcome! To contribute:
1. Fork the repository.
2. Create a new branch (`git checkout -b feature/your-feature`).
3. Commit your changes (`git commit -am 'Add feature'`).
4. Push to the branch (`git push origin feature/your-feature`).
5. Open a pull request.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## Authors

- zero

---

This [README.md](./README.md) provides a comprehensive overview of your crate, making it easy for users to understand its functionality and integrate it into their projects. Let me know if you'd like to customize it further!