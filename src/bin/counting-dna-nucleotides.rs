
extern crate rosalind;

// solution to http://rosalind.info/problems/dna/

const FILENAME: &'static str = "data/counting-dna-nucleotides.txt";

#[cfg(test)]
mod tests {
    #[test]
    fn count_nucleotides() {
        let dna = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";

        let (nb_adenyne, nb_thymine, nb_cytosine, nb_guanine) =
            ::count_nucleotides(&dna).expect("Couldn't count nucleotides!");

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

// returns the number of (adenyne, thymine, cytosine, guanine) nucleotides in the DNA string
fn count_nucleotides(dna: &str) -> Result<(u32, u32, u32, u32), String> {
    use rosalind::{ADENYNE, THYMINE, CYTOSINE, GUANINE};

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

fn main() {
    let dna = rosalind::io::load_file_to_string(FILENAME).expect("Couldn't open the file");

    println!("DNA string: {}", dna);

    let (nb_adenyne, nb_thymine, nb_cytosine, nb_guanine) =
        count_nucleotides(&dna).expect("Error counting nucleotides!");

    println!(
        "Adenyne: {}\nThymine: {}\nCytosine: {}\nGuanine: {}",
        nb_adenyne,
        nb_thymine,
        nb_cytosine,
        nb_guanine
    );
}
