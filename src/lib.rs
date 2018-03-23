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
