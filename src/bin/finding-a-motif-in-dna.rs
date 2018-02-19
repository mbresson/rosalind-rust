extern crate rosalind;

// solution to http://rosalind.info/problems/subs/

#[cfg(test)]
mod tests {
    #[test]
    fn find_all_substring_locations() {
        let string = "GATATATGCATATACTT";
        let substring = "ATAT";

        let locations = ::find_all_substring_locations(string, substring);

        assert_eq!(locations, vec![1, 3, 9]);
    }

    #[test]
    fn find_all_substring_locations_2() {
        let string = "ABAAAAEFGAAAAA";
        let substring = "AAA";

        let locations = ::find_all_substring_locations(string, substring);

        assert_eq!(locations, vec![2, 3, 9, 10, 11]);
    }
}

fn find_all_substring_locations(string: &str, substring: &str) -> Vec<usize> {
    let mut locations = Vec::new();

    let mut search_index = 0;

    while let Some(index) = string[search_index..].find(substring) {
        locations.push(index + search_index);

        search_index += index + 1;
    }

    locations
}

fn main() {
    let content = rosalind::io::load_data(file!()).expect("Couldn't open the file");

    let strings = content.lines().collect::<Vec<&str>>();

    if strings.len() != 2 {
        panic!("Expected two strings, got {}", strings.len());
    }

    let locations = find_all_substring_locations(strings[0], strings[1]);

    for index in &locations {
        // in Rosalind, indexes start at 1 instead of 0
        print!("{} ", index + 1);
    }

    println!(""); // because it looks better...
}
