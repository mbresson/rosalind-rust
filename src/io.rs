use std::io;
use std::fs;
use std::io::Read;

#[cfg(test)]
mod tests {
    #[test]
    fn data_file_path() {
        let main_file = "hello-world.rs";

        assert_eq!(::io::data_file_path(main_file), "data/hello-world.txt");
    }

    #[test]
    fn data_file_path_with_slash() {
        let main_file = "src/bin/hello-world.rs";

        assert_eq!(::io::data_file_path(main_file), "data/hello-world.txt");
    }
}

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

pub fn load_data(main_file: &str) -> io::Result<String> {
    load_file_to_string(&data_file_path(main_file))
}

// removes trailing white spaces and newline characters
fn trim_string(s: &mut String) {
    let len_without_trailing_whitespace = s.trim_right().len();

    s.truncate(len_without_trailing_whitespace);
}

// returns the path to the data file that has the same name as `main_file`
// e.g. "complementing-a-strand-of-dna.rs" -> "data/complementing-a-strand-of-dna.txt"
fn data_file_path(main_file: &str) -> String {
    let file_without_extension = main_file.trim_right_matches(".rs");

    let file_name_only = match file_without_extension.rfind("/") {
        Some(index) => &file_without_extension[index + 1..],
        None => file_without_extension,
    };

    format!("data/{}.txt", file_name_only)
}
