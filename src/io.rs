
use std::io;
use std::fs;
use std::io::Read;

/// Returns the contents of the file as a String.
///
/// # Examples
///
/// ```
/// match rosalind::io::load_file_to_string("data/test_file.txt") {
///     Ok(contents) => println!("The content of the file is {}", contents),
///     Err(file_error) => panic!("The file couldn't be read: {}", file_error),
/// }
/// ```
pub fn load_file_to_string(filename: &str) -> io::Result<String> {
    let mut file = fs::File::open(filename)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    trim_string(&mut buffer);

    Ok(buffer)
}

// removes trailing white spaces and newline characters
fn trim_string(s: &mut String) {
    let len_without_trailing_whitespace = s.trim_right().len();

    s.truncate(len_without_trailing_whitespace);
}
