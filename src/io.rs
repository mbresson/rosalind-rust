use std::io;
use std::fs;
use std::io::Read;
use std::str::FromStr;

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

    #[test]
    fn parse_separated_values() {
        let string = "1 and 42 and 6788";

        assert_eq!(
            ::io::parse_separated_values::<u32>(string, " and ").expect("Couldn't parse values!"),
            vec![1, 42, 6788]
        );
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

    let len_without_trailing_whitespace = buffer.trim_right().len();

    buffer.truncate(len_without_trailing_whitespace);

    Ok(buffer)
}

/// Returns the contents of the file located under the data/ directory
/// whose filename (excluding the .txt extension) is the same as `main_file` without extension.
///
/// # Examples
///
/// ```
/// match rosalind::io::load_data("test_file.rs") {
///     Ok(contents) => println!("The content of the data file is {}", contents),
///     Err(file_error) => panic!("The data file couldn't be read: {}", file_error),
/// }
/// ```
pub fn load_data(main_file: &str) -> io::Result<String> {
    load_file_to_string(&data_file_path(main_file))
}

/// Parses an arbitrary number of values from `values_string` separated by `separator`.
///
/// # Examples
///
/// ```
/// let values = rosalind::io::parse_separated_values::<u32>("1, 2, 3, 4, 5", ", ").expect("Couldn't parse values");
///
/// assert_eq!(values, vec![1, 2, 3, 4, 5]);
/// ```
pub fn parse_separated_values<F: FromStr>(
    values_string: &str,
    separator: &str,
) -> Result<Vec<F>, F::Err> {
    values_string
        .split(separator)
        .map(|raw_value| raw_value.parse::<F>())
        .collect::<Result<Vec<F>, _>>()
}

// returns the path to the data file that has the same name as `main_file`
// e.g. "complementing-a-strand-of-dna.rs" -> "data/complementing-a-strand-of-dna.txt"
fn data_file_path(main_file: &str) -> String {
    let file_without_extension = main_file.trim_right_matches(".rs");

    let file_name_only = match file_without_extension.rfind('/') {
        Some(index) => &file_without_extension[index + 1..],
        None => file_without_extension,
    };

    format!("data/{}.txt", file_name_only)
}
