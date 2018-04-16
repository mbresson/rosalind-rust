extern crate reqwest;
extern crate rosalind;

use std::convert::TryFrom;
use rosalind::amino_acids::{sequence, AminoAcid, Sequence};

// solution to http://rosalind.info/problems/mprt/

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
    use rosalind::amino_acids::Sequence;

    #[test]
    fn find_all_n_glycosylation_motif_positions_in_protein() {
        let protein_string = "MNTLQKGFTLIELMIVIAIVGILAAVALPAYQDYTARAQVSEAILLAEGQKSAVTEYYLN\
                              HGKWPENNTSAGVASPPSDIKGKYVKEVEVKNGVVTATMLSSGVNNEIKGKKLSLWARRE\
                              NGSVKWFCGQPVTRTDDDTVADAKDGKEIDTKHLPSTCRDNFDAK";

        let sequence = Sequence::try_from(protein_string).unwrap();

        let positions = ::find_all_n_glycosylation_motif_positions_in_protein(&sequence);

        assert_eq!(positions, vec![66, 67, 120]);
    }
}

// useless over-complication just for the sake of learning how to compose error types
#[derive(Debug)]
enum UniprotError {
    RequestError(reqwest::Error),
    ParseError(sequence::ParseError),
}

impl From<reqwest::Error> for UniprotError {
    fn from(err: reqwest::Error) -> UniprotError {
        UniprotError::RequestError(err)
    }
}

impl From<sequence::ParseError> for UniprotError {
    fn from(err: sequence::ParseError) -> UniprotError {
        UniprotError::ParseError(err)
    }
}

fn download_protein_from_uniprot(protein_id: &str) -> Result<Sequence, UniprotError> {
    let fasta_url = format!("https://www.uniprot.org/uniprot/{}.fasta", protein_id);

    let raw_data = reqwest::get(&fasta_url)?.text()?;

    let mut protein_lines = raw_data.lines();

    protein_lines.next(); // the first line is the sequence label, we don't need it

    let mut protein_str = String::new();

    for protein_line in protein_lines {
        protein_str.push_str(protein_line);
    }

    Ok(Sequence::try_from(protein_str.as_str())?)
}

fn find_all_n_glycosylation_motif_positions_in_protein(protein: &Sequence) -> Vec<usize> {
    use AminoAcid::{Asparagine, Proline, Serine, Threonine};

    let mut positions = Vec::new();

    // the shorthand protein motif notation for N-glycosylation: N{P}[ST]{P}
    // it means:
    // 1 Asparagine amino acid (N),             followed by
    // 1 "any amino acid except Proline (P)",   followed by
    // 1 "either Serine (S) or Threonine (T)",  followed by
    // 1 "any amino acid except Proline (P)"
    // the complete motif is 4-amino-acid long

    for (index, aa_1) in protein.into_iter().enumerate() {
        if index + 4 >= protein.len() {
            break;
        }

        let aa_2 = &protein[index + 1];
        let aa_3 = &protein[index + 2];
        let aa_4 = &protein[index + 3];

        if aa_1 == Asparagine && *aa_2 != Proline && (*aa_3 == Serine || *aa_3 == Threonine)
            && *aa_4 != Proline
        {
            positions.push(index);
        }
    }

    positions
}

fn main() {
    let dataset = rosalind::io::load_data(file!()).expect("Couldn't open the file");

    for uniprot_protein_id in dataset.lines() {
        let protein = match download_protein_from_uniprot(uniprot_protein_id) {
            Ok(data) => data,
            Err(error) => {
                eprintln!("Error downloading protein: {:?}", error);
                continue;
            }
        };

        let motif_positions = find_all_n_glycosylation_motif_positions_in_protein(&protein);

        if motif_positions.len() > 0 {
            println!("{}", uniprot_protein_id);
            for position in motif_positions {
                print!("{} ", position + 1); // rosalind expects 1-indexed positions
            }
            println!("")
        }
    }
}
