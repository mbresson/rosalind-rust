extern crate rosalind;

// solution to http://rosalind.info/problems/orf/

use rosalind::amino_acids;
use rosalind::dna::Sequence;
use std::convert::TryFrom;

#[cfg(test)]
mod tests {
    use rosalind::amino_acids::AminoAcid;
    use std::convert::TryFrom;
    use rosalind::dna::Sequence;

    fn string_to_amino_acids(sequence: &str) -> Vec<AminoAcid> {
        sequence
            .chars()
            .map(|ch| AminoAcid::from_char(ch).unwrap())
            .collect()
    }

    #[test]
    fn find_all_candidate_protein_strings() {
        let sequence = Sequence::try_from("AGCCATGTAGCTAACTCAGGTTACATGGGGATGACCCCGCGACTTGGATTAGAGTCTCTTTTGGAATAAGCCTGAATGATCCGAGTAGCATCTCAG").unwrap();

        let mut expected_candidates = vec![
            string_to_amino_acids("M"),
            string_to_amino_acids("MLLGSFRLIPKETLIQVAGSSPCNLS"),
            string_to_amino_acids("MGMTPRLGLESLLE"),
            string_to_amino_acids("MTPRLGLESLLE"),
        ];

        expected_candidates.sort();

        let mut candidates = ::find_all_candidate_protein_strings(&sequence)
            .expect("Error searching for candidates");

        candidates.sort();

        assert_eq!(candidates, expected_candidates);
    }
}

const RNA_START_CODON: &str = "AUG";
const RNA_END_CODON_1: &str = "UAA";
const RNA_END_CODON_2: &str = "UAG";
const RNA_END_CODON_3: &str = "UGA";

fn find_candidate_protein_strings_in_codons_sequence(
    codons: Vec<&str>,
) -> Result<Vec<Vec<amino_acids::AminoAcid>>, String> {
    let mut candidates = Vec::new();

    let mut index = 0;

    while index < codons.len() {
        if codons[index] == RNA_START_CODON {
            let mut candidate = RNA_START_CODON.to_string();

            let mut index_end = index + 1;
            while index_end < codons.len() {
                let has_end_codon = match codons[index_end] {
                    RNA_END_CODON_1 | RNA_END_CODON_2 | RNA_END_CODON_3 => true,
                    codon => {
                        candidate.push_str(codon);
                        false
                    }
                };

                if has_end_codon {
                    candidates.push(amino_acids::amino_acids_from_rna(&candidate)?);

                    break;
                }

                index_end += 1;
            }
        }

        index += 1;
    }

    Ok(candidates)
}

fn find_all_candidate_protein_strings(
    dna_sequence: &Sequence,
) -> Result<Vec<Vec<amino_acids::AminoAcid>>, String> {
    let rna = rosalind::dna_to_rna(&dna_sequence.to_string())?;

    let rna_reverse_complement =
        rosalind::dna_to_rna(&dna_sequence.reverse_complement().to_string())?;

    let codons_sequences = vec![
        rosalind::CodonIterator::new(&rna).collect(),
        rosalind::CodonIterator::new(&rna[1..]).collect(),
        rosalind::CodonIterator::new(&rna[2..]).collect(),
        rosalind::CodonIterator::new(&rna_reverse_complement).collect(),
        rosalind::CodonIterator::new(&rna_reverse_complement[1..]).collect(),
        rosalind::CodonIterator::new(&rna_reverse_complement[2..]).collect(),
    ];

    let mut candidates = Vec::new();

    for sequence in codons_sequences {
        let mut sequence_candidates = find_candidate_protein_strings_in_codons_sequence(sequence)?;

        for candidate in sequence_candidates {
            if !candidates.contains(&candidate) {
                candidates.push(candidate);
            }
        }
    }

    Ok(candidates)
}

fn main() {
    let fasta_content = rosalind::io::load_data(file!()).expect("Couldn't open the file");

    let sequences_strings = rosalind::fasta::parse_fasta_format_to_map(&fasta_content)
        .expect("Couldn't parse FASTA data");

    for (label, sequence_string) in sequences_strings.iter() {
        println!("{}", label);

        let sequence = Sequence::try_from(sequence_string.as_ref())
            .expect("Couldn't parse DNA sequence string");

        let candidate_protein_strings =
            find_all_candidate_protein_strings(&sequence).expect("Should work");

        for candidate in candidate_protein_strings {
            println!("{}", amino_acids::AminoAcidString(&candidate));
        }
    }
}
