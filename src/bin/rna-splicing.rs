extern crate rosalind;

// solution to http://rosalind.info/problems/splc/

use std::convert::TryFrom;
use rosalind::dna::Sequence as DnaSequence;
use rosalind::rna::Sequence as RnaSequence;
use rosalind::amino_acids::Sequence as AaSequence;

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
    use rosalind::amino_acids::Sequence as AaSequence;

    #[test]
    fn dna_string_to_spliced_rna() {
        let dna  = "ATGGTCTACATAGCTGACAAACAGCACGTAGCAATCGGTCGAATCTCGAGAGGCATATGGTCACATGATCGGTCGAGCGTGTTTCAAAGTTTGCGCCTAG";

        let introns_strings = vec!["ATCGGTCGAA", "ATCGGTCGAGCGTGT"];

        let spliced_dna = ::dna_string_to_spliced_rna(&dna, &introns_strings);

        let amino_acids = AaSequence::from(&spliced_dna);

        let expected_amino_acids = AaSequence::try_from("MVYIADKQHVASREAYGHMFKVCA").unwrap();

        assert_eq!(amino_acids, expected_amino_acids);
    }
}

// removes each intron from `dna_string`
// each intron is only removed once, that is, if it occurs more than once in the DNA sequence,
// only the first occurrence is removed
fn dna_string_to_spliced_rna(dna_string: &str, introns_strings: &Vec<&str>) -> RnaSequence {
    let mut spliced_dna_string = dna_string.to_string();

    for intron_string in introns_strings {
        if let Some(intron_index) = spliced_dna_string.find(intron_string) {
            let mut dna_string_without_intron = spliced_dna_string[0..intron_index].to_string();

            dna_string_without_intron
                .push_str(&spliced_dna_string[intron_index + intron_string.len()..]);

            spliced_dna_string = dna_string_without_intron;
        }
    }

    RnaSequence::from(&DnaSequence::try_from(spliced_dna_string.as_str()).unwrap())
}

fn main() {
    let fasta_content = rosalind::io::load_data(file!()).expect("Couldn't open the file");

    let sequences_strings = rosalind::fasta::parse_fasta_format_to_map(&fasta_content)
        .expect("Couldn't parse FASTA data");

    // the first sequence given is the main DNA sequence, what remains are the introns
    let main_sequence_label = &fasta_content.lines().next().unwrap()[1..]; // skip the first character, '>'
    let main_sequence = &sequences_strings[main_sequence_label];

    let introns_strings = sequences_strings
        .iter()
        .filter(|(label, _sequence)| *label != main_sequence_label)
        .map(|(_label, sequence)| sequence.as_str())
        .collect::<Vec<_>>();

    let spliced_rna = dna_string_to_spliced_rna(&main_sequence, &introns_strings);

    let amino_acids = AaSequence::from(&spliced_rna);

    println!("{}", amino_acids);
}
