pub mod io;
pub mod fasta;
pub mod probabilities;
pub mod amino_acids;

pub type Nucleobase = char;

pub const ADENYNE: Nucleobase = 'A';
pub const THYMINE: Nucleobase = 'T';
pub const CYTOSINE: Nucleobase = 'C';
pub const GUANINE: Nucleobase = 'G';
pub const URACIL: Nucleobase = 'U';

#[cfg(test)]
mod tests {
    #[test]
    fn count_nucleotides() {
        let dna = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";

        let (nb_adenyne, nb_thymine, nb_cytosine, nb_guanine) =
            ::count_nucleotides(dna).expect("Couldn't count nucleotides!");

        assert_eq!(nb_adenyne, 20);
        assert_eq!(nb_cytosine, 12);
        assert_eq!(nb_guanine, 17);
        assert_eq!(nb_thymine, 21);
    }

    #[test]
    fn count_nucleotides_fails_on_unknown_nucleobase() {
        let dna = "AGCTTTTCATTCZGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
        //                    (?)

        assert!(
            ::count_nucleotides(dna).is_err(),
            "Unknown nucleobase should return an Error"
        );
    }

    #[test]
    fn reverse_complement_dna_strand() {
        let strand = "AAAACCCGGT";

        let complemented_strand = ::reverse_complement_dna_strand(&strand)
            .expect("Error reversing and complementing the strand!");

        assert_eq!(complemented_strand, "ACCGGGTTTT");
    }
}

/// Returns the number of (adenyne, thymine, cytosine, guanine) nucleotides in the `dna` string.
///
/// # Examples
///
/// ```
/// match rosalind::count_nucleotides("AATAGGCTA") {
///     Ok((a, t, c, g)) => {
///         println!(
///             "Adenine: {}\nThymine: {}\nCytosine: {}\nGuanine: {}",
///             a, t, c, g
///         );
///     }
///     Err(error) => println!("Couldn't count nucleotides: {}", error),
/// }
/// ```
pub fn count_nucleotides(dna: &str) -> Result<(u32, u32, u32, u32), String> {
    let (mut adenyne, mut thymine, mut cytosine, mut guanine) = (0u32, 0u32, 0u32, 0u32);

    for nucleotide in dna.chars() {
        match nucleotide {
            ADENYNE => adenyne += 1,
            THYMINE => thymine += 1,
            CYTOSINE => cytosine += 1,
            GUANINE => guanine += 1,
            _ => return Err(format!("Unexpected nucleobase: {}", nucleotide)),
        }
    }

    Ok((adenyne, thymine, cytosine, guanine))
}

fn base_complement(base: Nucleobase) -> Result<Nucleobase, String> {
    match base {
        ADENYNE => Ok(THYMINE),
        THYMINE => Ok(ADENYNE),
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
