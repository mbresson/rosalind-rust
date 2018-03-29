extern crate rosalind;

// solution to http://rosalind.info/problems/rna/

#[cfg(test)]
mod tests {
    extern crate rosalind;

    #[test]
    fn dna_to_rna() {
        let dna = "GATGGAACTTGACTACGTAAATT";

        let rna = rosalind::dna_to_rna(dna).expect("Couldn't transcribe DNA to RNA!");

        assert_eq!(rna, "GAUGGAACUUGACUACGUAAAUU");
    }

    #[test]
    fn dna_to_rna_fails_on_unknown_nucleobase() {
        let dna = "GATGGAACTJGACTACGTAAATT";
        //                 (?)

        assert!(
            rosalind::dna_to_rna(dna).is_err(),
            "Unknown nucleobase should return an Error"
        );
    }
}

fn main() {
    let dna = rosalind::io::load_data(file!()).expect("Couldn't open the file");

    println!("DNA string: {}", dna);

    let rna = rosalind::dna_to_rna(&dna).expect("Error transcribing DNA to RNA!");

    println!("RNA string: {}", rna);
}
