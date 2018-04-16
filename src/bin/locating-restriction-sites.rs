extern crate rosalind;

// solution to http://rosalind.info/problems/revp/

use std::convert::TryFrom;
use rosalind::dna::Sequence as DnaSequence;

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
    use std::cmp::Ordering::Equal;
    use rosalind::dna::Sequence as DnaSequence;

    fn sort_positions_and_lengths(
        mut positions_and_lengths: Vec<::PositionAndLength>,
    ) -> Vec<::PositionAndLength> {
        positions_and_lengths.sort_by(
            |&(ref position_1, ref length_1), &(ref position_2, ref length_2)| {
                let first_cmp = position_1.cmp(&position_2);

                match first_cmp {
                    Equal => length_1.cmp(length_2),
                    _ => first_cmp,
                }
            },
        );

        positions_and_lengths
    }

    #[test]
    fn find_reverse_palindromes_of_length_between_4_and_12() {
        let sequence = DnaSequence::try_from("TCAATGCATGCGGGTCTATATGCAT").unwrap();

        let positions_and_lengths = sort_positions_and_lengths(
            ::find_reverse_palindromes_of_length_between_4_and_12(&sequence),
        );

        let expected_positions_and_lengths = sort_positions_and_lengths(vec![
            (3, 6),
            (4, 4),
            (5, 6),
            (6, 4),
            (16, 4),
            (17, 4),
            (19, 6),
            (20, 4),
        ]);

        assert_eq!(positions_and_lengths, expected_positions_and_lengths);
    }
}

type PositionAndLength = (usize, usize);

fn find_reverse_palindromes_of_length_between_4_and_12(
    sequence: &DnaSequence,
) -> Vec<PositionAndLength> {
    let mut positions_and_lengths = Vec::new();

    let reverse_complement = sequence.reverse_complement();

    for (index, _nucleobase) in sequence.into_iter().enumerate() {
        for subsequence_length in 4..13 {
            if index + subsequence_length > sequence.len() {
                break;
            }

            let subsequence = &sequence[index..index + subsequence_length];

            let reverse_complement_subsequence = &reverse_complement
                [sequence.len() - (index + subsequence_length)..sequence.len() - index];

            if subsequence == reverse_complement_subsequence {
                positions_and_lengths.push((index, subsequence_length));
            }
        }
    }

    positions_and_lengths
}

fn main() {
    let fasta_content = rosalind::io::load_data(file!()).expect("Couldn't open the file");

    let sequences = rosalind::fasta::parse_fasta_format_to_map(&fasta_content)
        .expect("Couldn't parse FASTA data");

    for (_label, sequence_str) in sequences {
        let sequence = DnaSequence::try_from(sequence_str.as_str()).unwrap();

        for (position, length) in find_reverse_palindromes_of_length_between_4_and_12(&sequence) {
            println!("{} {}", position + 1, length); // position + 1 because Rosalind expects 1-indexed strings
        }
    }
}
