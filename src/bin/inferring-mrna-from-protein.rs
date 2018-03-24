extern crate rosalind;

use rosalind::amino_acids::AminoAcid;

// solution to http://rosalind.info/problems/mrna/

#[cfg(test)]
mod tests {
    #[test]
    fn compute_number_possible_rna_mod_1_000_000() {
        let proteins_string = "MA";

        assert_eq!(
            ::compute_number_possible_rna_mod_1_000_000(proteins_string),
            12
        );
    }
}

fn possible_codons_for_aa(aa: AminoAcid) -> Vec<&'static str> {
    use AminoAcid::*;

    match aa {
        Alanine => vec!["GCU", "GCC", "GCA", "GCG"],
        Arginine => vec!["CGU", "CGC", "CGA", "CGG", "AGA", "AGG"],
        Asparagine => vec!["AAU", "AAC"],
        AsparticAcid => vec!["GAU", "GAC"],
        Cysteine => vec!["UGU", "UGC"],
        GlutamicAcid => vec!["GAA", "GAG"],
        Glutamine => vec!["CAA", "CAG"],
        Glycine => vec!["GGU", "GGC", "GGA", "GGG"],
        Histidine => vec!["CAU", "CAC"],
        Isoleucine => vec!["AUU", "AUC", "AUA"],
        Leucine => vec!["CUU", "CUC", "CUA", "CUG", "UUA", "UUG"],
        Lysine => vec!["AAA", "AAG"],
        Methionine => vec!["AUG"],
        Phenyalalanine => vec!["UUU", "UUC"],
        Proline => vec!["CCU", "CCC", "CCA", "CCG"],
        Serine => vec!["UCU", "UCC", "UCA", "UCG", "AGU", "AGC"],
        Threonine => vec!["ACU", "ACC", "ACA", "ACG"],
        Tryptophan => vec!["UGG"],
        Tyrosine => vec!["UAU", "UAC"],
        Valine => vec!["GUU", "GUC", "GUA", "GUG"],
    }
}

fn compute_number_possible_rna_mod_1_000_000(proteins_string: &str) -> u32 {
    // only 1 start codon possible, so 1 % 1_000_000 = 1
    let mut possible_rna_modulo = 1;

    for aa_str in proteins_string.chars() {
        let aa = AminoAcid::from_char(aa_str).expect("Incorrect AA string");

        let possible_codons = possible_codons_for_aa(aa);
        let num_possible_codons = possible_codons.len() as u32;

        // it seems that there is a risk of producing a multiplication overflow here
        // however we can guarantee that it can never happen:
        //
        // an amino acid can only have up to 6 corresponding RNA codons
        // and possible_rna_modulo can never be >= 1_000_000
        // so, at most, we'll be multiplicating 999_999 * 6, which can never overflow on u32
        possible_rna_modulo = (possible_rna_modulo * num_possible_codons) % 1_000_000;

        if possible_rna_modulo == 0 {
            return 0;
        }
    }

    // take into account the 3 possible end codons that must end any mRNA string
    (possible_rna_modulo * 3) % 1_000_000
}

fn main() {
    let proteins_string = rosalind::io::load_data(file!()).expect("Couldn't open the file");

    println!("Proteins string: {}", proteins_string);

    let possible_rna_mod = compute_number_possible_rna_mod_1_000_000(&proteins_string);

    println!(
        "Possible RNA strings modulo 1,000,000: {}",
        possible_rna_mod
    );
}
