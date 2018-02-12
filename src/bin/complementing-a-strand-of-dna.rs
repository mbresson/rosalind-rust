
extern crate rosalind;

use rosalind::Nucleobase;

// solution to http://rosalind.info/problems/revc/

const FILENAME: &'static str = "data/complementing-a-strand-of-dna.txt";

#[cfg(test)]
mod tests {
    #[test]
    fn reverse_complement_dna_strand() {
        let strand = "AAAACCCGGT";

        let complemented_strand = ::reverse_complement_dna_strand(&strand).expect("Error reversing and complementing the strand!");

        assert_eq!(complemented_strand, "ACCGGGTTTT");
    }
}

fn base_complement(base: Nucleobase) -> Result<Nucleobase, String> {
    use rosalind::{ADENYNE, THYMINE, CYTOSINE, GUANINE};

    match base {
        ADENYNE => Ok(THYMINE),
        THYMINE => Ok(ADENYNE),
        CYTOSINE => Ok(GUANINE),
        GUANINE => Ok(CYTOSINE),
        _ => Err(format!("Unexpected nucleobase: {}", base)),
    }
}

fn reverse_complement_dna_strand(strand: &str) -> Result<String, String> {
    strand
        .chars()
        .rev()
        .map(base_complement)
        .collect()
}

fn main() {
    let dna_strand = rosalind::io::load_file_to_string(FILENAME).expect("Couldn't open the file");

    println!("DNA strand: {}", dna_strand);

    let complemented_strand = reverse_complement_dna_strand(&dna_strand).expect("Error complementing DNA strand!");

    println!("Reverse complement: {}", complemented_strand);
}
