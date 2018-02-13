
extern crate rosalind;

use std::fmt;

// solution to http://rosalind.info/problems/prot/

const FILENAME: &'static str = "data/translating-dna-into-protein.txt";

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

enum AminoAcid {
    Alanine,
    Arginine,
    Asparagine,
    AsparticAcid,
    Cysteine,
    GlutamicAcid,
    Glutamine,
    Glycine,
    Histidine,
    Isoleucine,
    Leucine,
    Lysine,
    Methionine,
    Phenyalalanine,
    Proline,
    Serine,
    Threonine,
    Tryptophan,
    Tyrosine,
    Valine,
}

impl fmt::Display for AminoAcid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AminoAcid::*;

        let repr = match *self {
            Alanine => "A",
            Arginine => "R",
            Asparagine => "N",
            AsparticAcid => "D",
            Cysteine => "C",
            GlutamicAcid => "E",
            Glutamine => "Q",
            Glycine => "G",
            Histidine => "H",
            Isoleucine => "I",
            Leucine => "L",
            Lysine => "K",
            Methionine => "M",
            Phenyalalanine => "F",
            Proline => "P",
            Serine => "S",
            Threonine => "T",
            Tryptophan => "W",
            Tyrosine => "Y",
            Valine => "V", 
        };

        write!(f, "{}", repr)
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

// CodonIterator iterates over all the codons of a RNA string
// a codon is a group of 3 nucleobases
// if the length of the RNA string is not a multiple of 3, the remaining bases are skipped
struct CodonIterator<'a> {
    rna: &'a str,
    index: usize,
}

impl<'a> CodonIterator<'a> {
    fn new(rna: &'a str) -> CodonIterator {
        CodonIterator { rna, index: 0 }
    }
}

impl<'a> Iterator for CodonIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 3;

        if self.index > self.rna.len() {
            None
        } else {
            Some(&self.rna[self.index - 3..self.index])
        }
    }
}

fn amino_acids_from_rna(rna: &str) -> Result<Vec<AminoAcid>, String> {
    use AminoAcid::*;

    let mut amino_acids = Vec::new();

    for codon in CodonIterator::new(rna) {
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
    let messenger_rna =
        rosalind::io::load_file_to_string(FILENAME).expect("Couldn't open the file");

    println!("RNA string: {}", messenger_rna);

    let amino_acids =
        amino_acids_from_rna(&messenger_rna).expect("Error translating mRNA into amino acids!");

    println!("AA string: {}", AminoAcidString(amino_acids));
}
