extern crate rosalind;

// solution to http://rosalind.info/problems/orf/

use std::convert::TryFrom;
use std::iter::FromIterator;

use rosalind::dna::Sequence as DnaSequence;
use rosalind::rna::{frequent_codons, Nucleobase as RnaNucleobase, Sequence as RnaSequence,
                    StrictCodonIterator};
use rosalind::amino_acids::Sequence as AaSequence;

#[cfg(test)]
mod tests {
    use rosalind::amino_acids::Sequence as AaSequence;
    use std::convert::TryFrom;
    use rosalind::dna::Sequence as DnaSequence;

    #[test]
    fn find_all_candidate_protein_strings() {
        let sequence = DnaSequence::try_from("AGCCATGTAGCTAACTCAGGTTACATGGGGATGACCCCGCGACTTGGATTAGAGTCTCTTTTGGAATAAGCCTGAATGATCCGAGTAGCATCTCAG").unwrap();

        let mut expected_candidates = vec![
            AaSequence::try_from("M").unwrap(),
            AaSequence::try_from("MLLGSFRLIPKETLIQVAGSSPCNLS").unwrap(),
            AaSequence::try_from("MGMTPRLGLESLLE").unwrap(),
            AaSequence::try_from("MTPRLGLESLLE").unwrap(),
        ];

        expected_candidates.sort();

        let mut candidates = ::find_all_candidate_protein_strings(&sequence)
            .expect("Error searching for candidates");

        candidates.sort();

        assert_eq!(candidates, expected_candidates);
    }
}

fn is_stop_codon(codon: &[RnaNucleobase]) -> bool {
    use frequent_codons::*;

    match codon {
        UAA | UAG | UGA => true,
        _ => false,
    }
}

const RNA_START_CODON: &[RnaNucleobase] = frequent_codons::AUG;

fn find_candidate_protein_strings_in_codons_sequence(
    codons: Vec<&[RnaNucleobase]>,
) -> Result<Vec<AaSequence>, String> {
    let mut candidates = Vec::new();

    let mut index = 0;

    while index < codons.len() {
        if codons[index] == RNA_START_CODON {
            let mut candidate = Vec::from_iter(RNA_START_CODON.iter().cloned());

            let mut index_end = index + 1;
            while index_end < codons.len() {
                if is_stop_codon(codons[index_end]) {
                    let sequence = RnaSequence::new(candidate);

                    candidates.push(AaSequence::from(&sequence));

                    break;
                } else {
                    candidate.extend(codons[index_end].iter().cloned());
                }

                index_end += 1;
            }
        }

        index += 1;
    }

    Ok(candidates)
}

fn find_all_candidate_protein_strings(
    dna_sequence: &DnaSequence,
) -> Result<Vec<AaSequence>, String> {
    let rna_reverse_complement = RnaSequence::from(&dna_sequence.reverse_complement());

    let rna = RnaSequence::from(dna_sequence);

    let codons_sequences = vec![
        StrictCodonIterator::new_starting_from(&rna, 0).collect(),
        StrictCodonIterator::new_starting_from(&rna, 1).collect(),
        StrictCodonIterator::new_starting_from(&rna, 2).collect(),
        StrictCodonIterator::new_starting_from(&rna_reverse_complement, 0).collect(),
        StrictCodonIterator::new_starting_from(&rna_reverse_complement, 1).collect(),
        StrictCodonIterator::new_starting_from(&rna_reverse_complement, 2).collect(),
        /*
        rosalind::CodonIterator::new(&rna).collect(),
        rosalind::CodonIterator::new(&rna[1..]).collect(),
        rosalind::CodonIterator::new(&rna[2..]).collect(),
        rosalind::CodonIterator::new(&rna_reverse_complement).collect(),
        rosalind::CodonIterator::new(&rna_reverse_complement[1..]).collect(),
        rosalind::CodonIterator::new(&rna_reverse_complement[2..]).collect(),
*/
    ];

    /*
    let codons_sequences = vec![
        rosalind::CodonIterator::new(&rna).collect(),
        rosalind::CodonIterator::new(&rna[1..]).collect(),
        rosalind::CodonIterator::new(&rna[2..]).collect(),
        rosalind::CodonIterator::new(&rna_reverse_complement).collect(),
        rosalind::CodonIterator::new(&rna_reverse_complement[1..]).collect(),
        rosalind::CodonIterator::new(&rna_reverse_complement[2..]).collect(),
    ];
     */

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

        let sequence =
            DnaSequence::try_from(sequence_string.as_str()).expect("Couldn't parse the sequence");

        let candidate_protein_strings =
            find_all_candidate_protein_strings(&sequence).expect("Should work");

        for candidate in candidate_protein_strings {
            println!("{}", candidate);
        }
    }
}
