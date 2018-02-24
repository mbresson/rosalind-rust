extern crate rosalind;

use std::collections::HashMap;
use rosalind::fasta::{Label, Sequence};

// solution to http://rosalind.info/problems/grph/

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use rosalind::fasta::{Label, Sequence};
    use std::cmp::Ordering::Equal;

    fn sort_adjacencies(mut adjacencies: Vec<(Label, Label)>) -> Vec<(Label, Label)> {
        adjacencies.sort_by(|&(ref l_1_1, ref l_1_2), &(ref l_2_1, ref l_2_2)| {
            let first_cmp = l_1_1.cmp(&l_2_1);

            match first_cmp {
                Equal => l_1_2.cmp(l_2_2),
                _ => first_cmp,
            }
        });

        adjacencies
    }

    #[test]
    fn compute_adjacency_list() {
        let sequences: HashMap<Label, Sequence> = [
            ("Rosalind_0498".to_string(), "AAATAAA".to_string()),
            ("Rosalind_2391".to_string(), "AAATTTT".to_string()),
            ("Rosalind_2323".to_string(), "TTTTCCC".to_string()),
            ("Rosalind_0442".to_string(), "AAATCCC".to_string()),
            ("Rosalind_5013".to_string(), "GGGTGGG".to_string()),
        ].iter()
            .cloned()
            .collect();

        let expected_adjacencies = sort_adjacencies(vec![
            ("Rosalind_2391".to_string(), "Rosalind_2323".to_string()),
            ("Rosalind_0498".to_string(), "Rosalind_2391".to_string()),
            ("Rosalind_0498".to_string(), "Rosalind_0442".to_string()),
        ]);

        let adjacencies = sort_adjacencies(::compute_adjacency_list(&sequences, 3));

        assert_eq!(adjacencies, expected_adjacencies);
    }
}

fn compute_adjacency_list(
    sequences: &HashMap<Label, Sequence>,
    subsequence_length: usize,
) -> Vec<(Label, Label)> {
    let mut adjacency_list = Vec::new();

    for (label_1, sequence_1) in sequences {
        let sequence_1_length = sequence_1.len();

        if sequence_1_length < subsequence_length {
            continue;
        }

        let suffix = &sequence_1[sequence_1_length - subsequence_length..];

        for (label_2, sequence_2) in sequences {
            if label_2 == label_1 || sequence_2.len() < subsequence_length {
                continue;
            }

            let prefix = &sequence_2[..subsequence_length];

            if suffix == prefix {
                adjacency_list.push((label_1.clone(), label_2.clone()));
            }
        }
    }

    adjacency_list
}

fn main() {
    let fasta_content = rosalind::io::load_data(file!()).expect("Couldn't open the file");

    let sequences = rosalind::fasta::parse_fasta_format_to_map(&fasta_content)
        .expect("Couldn't parse FASTA data");

    let adjacency_list = compute_adjacency_list(&sequences, 3);

    for &(ref label_sequence_1, ref label_sequence_2) in &adjacency_list {
        println!("{} {}", label_sequence_1, label_sequence_2);
    }
}
