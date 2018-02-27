extern crate rosalind;

use std::collections::HashMap;
use rosalind::fasta::{Label, Sequence};

// solution to http://rosalind.info/problems/lcsm/

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use rosalind::fasta::{Label, Sequence};

    #[test]
    fn find_longest_common_substring() {
        let sequences: HashMap<Label, Sequence> = [
            ("Rosalind_1".to_string(), "GATTACA".to_string()),
            ("Rosalind_2".to_string(), "TAGACCA".to_string()),
            ("Rosalind_3".to_string(), "ATACA".to_string()),
        ].iter()
            .cloned()
            .collect();

        let longest_common_substring: &str = ::find_longest_common_substring(&sequences);

        // there are 2 possible solutions here and we have no guarantee on which one will come out,
        // due to the fact that HashMap is unordered and the outcome depends on which sequence is compared first
        let expected_possible_longest_common_substrings: Vec<&str> = vec!["AC", "TA"];

        assert!(expected_possible_longest_common_substrings.contains(&longest_common_substring));
    }
}

// in order to find the longest common substring in all sequences,
// we start by searching a common substring of length 1
// if we find it, then we'll only look for a common substring of length 2,
// if we find it, then we'll only look for a common substring of length (last common substring's length) + 1
// and so on...
// this will allow us to skip useless comparisons: as long as we already have a common substring,
// we don't need to search for other common substrings that are not longer than the previously found one
fn find_longest_common_substring<'a>(sequences_map: &'a HashMap<Label, Sequence>) -> &'a str {
    let mut longest_common_substring = "";

    if sequences_map.len() == 0 {
        return longest_common_substring;
    }

    let mut sequences_iter = sequences_map.iter().map(|(_, sequence)| &sequence[..]);

    let first_sequence = sequences_iter.next().unwrap();
    let sequences = sequences_iter.collect::<Vec<_>>();

    let mut ch_index = 0;

    while ch_index + longest_common_substring.len() < first_sequence.len() {
        let needle = &first_sequence[ch_index..ch_index + longest_common_substring.len() + 1];

        if sequences.iter().all(|sequence| sequence.contains(needle)) {
            longest_common_substring = needle;
        } else {
            ch_index += 1;
        }
    }

    longest_common_substring
}

fn main() {
    let fasta_content = rosalind::io::load_data(file!()).expect("Couldn't open the file");

    let sequences = rosalind::fasta::parse_fasta_format_to_map(&fasta_content)
        .expect("Couldn't parse FASTA data");

    let longest_common_substring = find_longest_common_substring(&sequences);

    println!("{}", longest_common_substring);
}
