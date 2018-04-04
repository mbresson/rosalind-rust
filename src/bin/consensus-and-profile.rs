extern crate rosalind;

use rosalind::fasta::{Label, Sequence};
use rosalind::{Nucleobase, ADENINE, CYTOSINE, GUANINE, THYMINE};

// solution to http://rosalind.info/problems/cons/

#[cfg(test)]
mod tests {
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

        let profile_matrix = ::build_profile_matrix(sequences.into_iter().collect())
            .expect("Error building profile matrix");

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
        row_to_string(&profile_matrix[nucleobase_index(ADENINE).unwrap()])
    );
    println!(
        "C: {}",
        row_to_string(&profile_matrix[nucleobase_index(CYTOSINE).unwrap()])
    );
    println!(
        "G: {}",
        row_to_string(&profile_matrix[nucleobase_index(GUANINE).unwrap()])
    );
    println!(
        "T: {}",
        row_to_string(&profile_matrix[nucleobase_index(THYMINE).unwrap()])
    );
}

fn nucleobase_index(nucleobase: Nucleobase) -> Result<usize, String> {
    match nucleobase {
        ADENINE => Ok(0),
        CYTOSINE => Ok(1),
        GUANINE => Ok(2),
        THYMINE => Ok(3),
        _ => Err(format!("Unallowed nucleobase: {}", nucleobase)),
    }
}

fn nucleobase_from_index(index: usize) -> Result<Nucleobase, String> {
    match index {
        0 => Ok(ADENINE),
        1 => Ok(CYTOSINE),
        2 => Ok(GUANINE),
        3 => Ok(THYMINE),
        _ => Err(format!("Unallowed nucleobase index: {}", index)),
    }
}

fn build_profile_matrix(sequences: Vec<(Label, Sequence)>) -> Result<ProfileMatrix, String> {
    if sequences.len() == 0 {
        return Err("Cannot create profile matrix for 0 sequences!".to_string());
    }

    let &(_, ref first_sequence) = &sequences[0];
    let sequences_length = first_sequence.len();

    let mut profile_matrix = [
        vec![0; sequences_length], // Adenine
        vec![0; sequences_length], // Cytosize
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

        for (index_in_sequence, nucleobase) in sequence.char_indices() {
            match nucleobase_index(nucleobase) {
                Ok(index) => {
                    profile_matrix[index][index_in_sequence] += 1;
                }
                Err(error) => {
                    return Err(format!(
                        "Couldn't fill profile matrix with data from sequence {}: {}",
                        label, error
                    ))
                }
            }
        }
    }

    Ok(profile_matrix)
}

fn compute_consensus_string(profile_matrix: &ProfileMatrix) -> String {
    //let mut consensus_string_buffer = [' '; profile_matrix[0].len()];
    let mut consensus_string_buffer: Vec<String> = Vec::with_capacity(profile_matrix[0].len());

    for (index, _) in profile_matrix[0].iter().enumerate() {
        let nucleobases = [
            profile_matrix[nucleobase_index(ADENINE).unwrap()][index],
            profile_matrix[nucleobase_index(CYTOSINE).unwrap()][index],
            profile_matrix[nucleobase_index(GUANINE).unwrap()][index],
            profile_matrix[nucleobase_index(THYMINE).unwrap()][index],
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

    let profile_matrix = build_profile_matrix(sequences.into_iter().collect())
        .expect("Error building profile matrix");

    let consensus_string = compute_consensus_string(&profile_matrix);

    println!("{}", consensus_string);

    display_profile_matrix(&profile_matrix);
}
