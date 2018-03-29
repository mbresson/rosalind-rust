extern crate rosalind;

use std::fmt;
use rosalind::amino_acids::AminoAcid;

// solution to http://rosalind.info/problems/prot/

#[cfg(test)]
mod tests {
    #[test]
    fn amino_acids_from_rna() {
        let rna = "AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGGUGA";

        let aas = ::amino_acids_from_rna(&rna).expect("Couldn't translate RNA to AA!");

        let aas_string = aas.iter().map(|acid| acid.to_string()).collect::<String>();

        assert_eq!(aas_string, "MAMAPRTEINSTRING");
    }
}

struct AminoAcidString(Vec<AminoAcid>);

impl fmt::Display for AminoAcidString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for amino_acid in &self.0 {
            try!(write!(f, "{}", amino_acid));
        }

        Ok(())
    }
}

fn amino_acids_from_rna(rna: &str) -> Result<Vec<AminoAcid>, String> {
    use AminoAcid::*;

    let mut amino_acids = Vec::new();

    for codon in rosalind::CodonIterator::new(rna) {
        let amino_acid = match codon {
            "UUU" | "UUC" => Phenyalalanine,
            "UUA" | "UUG" | "CUU" | "CUC" | "CUA" | "CUG" => Leucine,
            "AUU" | "AUC" | "AUA" => Isoleucine,
            "AUG" => Methionine,
            "GUU" | "GUC" | "GUA" | "GUG" => Valine,
            "UCU" | "UCC" | "UCA" | "UCG" => Serine,
            "CCU" | "CCC" | "CCA" | "CCG" => Proline,
            "ACU" | "ACC" | "ACA" | "ACG" => Threonine,
            "GCU" | "GCC" | "GCA" | "GCG" => Alanine,
            "UAU" | "UAC" => Tyrosine,
            "CAU" | "CAC" => Histidine,
            "CAA" | "CAG" => Glutamine,
            "AAU" | "AAC" => Asparagine,
            "AAA" | "AAG" => Lysine,
            "GAU" | "GAC" => AsparticAcid,
            "GAA" | "GAG" => GlutamicAcid,
            "UGU" | "UGC" => Cysteine,
            "UGG" => Tryptophan,
            "CGU" | "CGC" | "CGA" | "CGG" => Arginine,
            "AGU" | "AGC" => Serine,
            "AGA" | "AGG" => Arginine,
            "GGU" | "GGC" | "GGA" | "GGG" => Glycine,
            "UAA" | "UAG" | "UGA" => {
                // STOP codons, they don't code for any amino acid
                continue;
            }
            _ => {
                return Err(format!("No amino acid matching codon {}!", codon));
            }
        };

        amino_acids.push(amino_acid);
    }

    Ok(amino_acids)
}

fn main() {
    let messenger_rna = rosalind::io::load_data(file!()).expect("Couldn't open the file");

    println!("RNA string: {}", messenger_rna);

    let amino_acids =
        amino_acids_from_rna(&messenger_rna).expect("Error translating mRNA into amino acids!");

    println!("AA string: {}", AminoAcidString(amino_acids));
}
