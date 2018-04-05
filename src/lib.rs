pub mod io;
pub mod fasta;
pub mod probabilities;
pub mod amino_acids;
pub mod dna;

pub type Nucleobase = char;

pub const ADENINE: Nucleobase = 'A';
pub const THYMINE: Nucleobase = 'T';
pub const CYTOSINE: Nucleobase = 'C';
pub const GUANINE: Nucleobase = 'G';
pub const URACIL: Nucleobase = 'U';

#[cfg(test)]
mod tests {
    #[test]
    fn reverse_complement_dna_strand() {
        let strand = "AAAACCCGGT";

        let complemented_strand = ::reverse_complement_dna_strand(&strand)
            .expect("Error reversing and complementing the strand!");

        assert_eq!(complemented_strand, "ACCGGGTTTT");
    }
}

fn base_complement(base: Nucleobase) -> Result<Nucleobase, String> {
    match base {
        ADENINE => Ok(THYMINE),
        THYMINE => Ok(ADENINE),
        CYTOSINE => Ok(GUANINE),
        GUANINE => Ok(CYTOSINE),
        _ => Err(format!("Unexpected nucleobase: {}", base)),
    }
}

pub fn reverse_complement_dna_strand(strand: &str) -> Result<String, String> {
    strand.chars().rev().map(base_complement).collect()
}

// CodonIterator iterates over all the codons of a RNA string
// a codon is a group of 3 nucleobases
// if the length of the RNA string is not a multiple of 3, the remaining bases are skipped
pub struct CodonIterator<'a> {
    rna: &'a str,
    index: usize,
}

impl<'a> CodonIterator<'a> {
    pub fn new(rna: &'a str) -> CodonIterator {
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

fn dna_base_to_rna_base(base: char) -> Result<char, String> {
    match base {
        THYMINE => Ok(URACIL),
        ADENINE | CYTOSINE | GUANINE => Ok(base),
        _ => Err(format!("Unexpected nucleobase: {}", base)),
    }
}

pub fn dna_to_rna(dna: &str) -> Result<String, String> {
    dna.chars()
        .map(|base: char| dna_base_to_rna_base(base))
        .collect()
}
