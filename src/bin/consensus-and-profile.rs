extern crate rosalind;

use std::convert::TryFrom;
use rosalind::fasta::Label;
use rosalind::dna::{Nucleobase as DnaNucleobase, Nucleobase::*, Sequence as DnaSequence};

// solution to http://rosalind.info/problems/cons/

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
    use rosalind::dna::Sequence as DnaSequence;

    #[test]
    fn build_profile_matrix() {
        let fasta_content = r"
>Rosalind_1
ATCCAGCT
>Rosalind_2
GGGCAACT
>Rosalind_3
ATGGATCT
>Rosalind_4
AAGCAACC
>Rosalind_5
TTGGAACT
>Rosalind_6
ATGCCATT
>Rosalind_7
ATGGCACT
";

        let sequences = ::rosalind::fasta::parse_fasta_format_to_map(&fasta_content)
            .expect("Couldn't parse FASTA data");

        let profile_matrix = ::build_profile_matrix(
            sequences
                .into_iter()
                .map(|(label, sequence_str)| {
                    // convert all ::fasta::Sequence to DnaSequence with TryFrom<&str> trait
                    // if the sequence is not correct (e.g. someone slipped a fancy character such as '錯' in it),
                    // die in agonizing pain
                    (label, DnaSequence::try_from(sequence_str.as_str()).unwrap())
                })
                .collect(),
        ).expect("Error building profile matrix");

        let expected_profile_matrix = [
            vec![5, 1, 0, 0, 5, 5, 0, 0],
            vec![0, 0, 1, 4, 2, 0, 6, 1],
            vec![1, 1, 6, 3, 0, 1, 0, 0],
            vec![1, 5, 0, 0, 0, 1, 1, 6],
        ];

        assert_eq!(profile_matrix, expected_profile_matrix);
    }

    #[test]
    fn compute_consensus_string() {
        let profile_matrix = [
            vec![5, 1, 0, 0, 5, 5, 0, 0],
            vec![0, 0, 1, 4, 2, 0, 6, 1],
            vec![1, 1, 6, 3, 0, 1, 0, 0],
            vec![1, 5, 0, 0, 0, 1, 1, 6],
        ];

        let consensus_string = ::compute_consensus_string(&profile_matrix);

        assert_eq!(consensus_string, "ATGCAACT");
    }
}

// in ProfileMatrix, each row contains the number of occurrences of a nucleobase in all sequences, letter by letter
//
// example of profile matrix:
//
// A   5 1 0 0 5 5 0 0
// C   0 0 1 4 2 0 6 1
// G   1 1 6 3 0 1 0 0
// T   1 5 0 0 0 1 1 6
type ProfileMatrix = [Vec<u32>; 4];

fn row_to_string(row: &Vec<u32>) -> String {
    row.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn display_profile_matrix(profile_matrix: &ProfileMatrix) {
    println!(
        "A: {}",
        row_to_string(&profile_matrix[nucleobase_index(Adenine)])
    );
    println!(
        "C: {}",
        row_to_string(&profile_matrix[nucleobase_index(Cytosine)])
    );
    println!(
        "G: {}",
        row_to_string(&profile_matrix[nucleobase_index(Guanine)])
    );
    println!(
        "T: {}",
        row_to_string(&profile_matrix[nucleobase_index(Thymine)])
    );
}

fn nucleobase_index(nucleobase: DnaNucleobase) -> usize {
    match nucleobase {
        Adenine => 0,
        Cytosine => 1,
        Guanine => 2,
        Thymine => 3,
    }
}

fn nucleobase_from_index(index: usize) -> Result<DnaNucleobase, String> {
    match index {
        0 => Ok(Adenine),
        1 => Ok(Cytosine),
        2 => Ok(Guanine),
        3 => Ok(Thymine),
        _ => Err(format!("Unallowed nucleobase index: {}", index)),
    }
}

fn build_profile_matrix(sequences: Vec<(Label, DnaSequence)>) -> Result<ProfileMatrix, String> {
    if sequences.len() == 0 {
        return Err("Cannot create profile matrix for 0 sequences!".to_string());
    }

    let &(_, ref first_sequence) = &sequences[0];
    let sequences_length = first_sequence.len();

    let mut profile_matrix = [
        vec![0; sequences_length], // Adenine
        vec![0; sequences_length], // Cytosine
        vec![0; sequences_length], // Guanine
        vec![0; sequences_length], // Thymine
    ];

    for &(ref label, ref sequence) in &sequences {
        if sequence.len() != sequences_length {
            return Err(format!(
                "Sequences should have a length of {} but sequence {} has length of {}",
                sequences_length,
                label,
                sequence.len()
            ));
        }

        for (index_in_sequence, nucleobase) in sequence.into_iter().enumerate() {
            profile_matrix[nucleobase_index(nucleobase)][index_in_sequence] += 1;
        }
    }

    Ok(profile_matrix)
}

fn compute_consensus_string(profile_matrix: &ProfileMatrix) -> String {
    let mut consensus_string_buffer: Vec<String> = Vec::with_capacity(profile_matrix[0].len());

    for (index, _) in profile_matrix[0].iter().enumerate() {
        let nucleobases = [
            profile_matrix[nucleobase_index(Adenine)][index],
            profile_matrix[nucleobase_index(Cytosine)][index],
            profile_matrix[nucleobase_index(Guanine)][index],
            profile_matrix[nucleobase_index(Thymine)][index],
        ];

        let (_, dominant_nucleobase_index) = nucleobases
            .iter()
            .enumerate()
            .map(|(index, value)| (value, index))
            .max()
            .unwrap();

        let dominant_nucleobase = nucleobase_from_index(dominant_nucleobase_index).unwrap();

        consensus_string_buffer.push(dominant_nucleobase.to_string());
    }

    // find max of each letter
    consensus_string_buffer.join("")
}

fn main() {
    let fasta_content = rosalind::io::load_data(file!()).expect("Couldn't open the file");

    let sequences = rosalind::fasta::parse_fasta_format_to_map(&fasta_content)
        .expect("Couldn't parse FASTA data");

    let profile_matrix = build_profile_matrix(
        // this very eloquent piece of gargling code's life purpose is to convert sequence
        // from:
        // HashMap<::fasta::Label, ::fasta::Sequence> (where Label and Sequence are just String)
        // to:
        // Vec<(::fasta::Label, DnaSequence)>
        sequences
            .into_iter()
            .map(|(label, sequence_str)| {
                // convert all ::fasta::Sequence to DnaSequence with TryFrom<&str> trait
                // if the sequence is not correct (e.g. someone slipped a fancy character such as '錯' in it),
                // die in agonizing pain
                (label, DnaSequence::try_from(sequence_str.as_str()).unwrap())
            })
            .collect(),
    ).expect("Error building profile matrix");

    let consensus_string = compute_consensus_string(&profile_matrix);

    println!("{}", consensus_string);

    display_profile_matrix(&profile_matrix);
}
