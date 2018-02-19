extern crate rosalind;

// solution to http://rosalind.info/problems/rna/

#[cfg(test)]
mod tests {
    #[test]
    fn dna_to_rna() {
        let dna = "GATGGAACTTGACTACGTAAATT";

        let rna = ::dna_to_rna(dna).expect("Couldn't transcribe DNA to RNA!");

        assert_eq!(rna, "GAUGGAACUUGACUACGUAAAUU");
    }

    #[test]
    fn dna_to_rna_fails_on_unknown_nucleobase() {
        let dna = "GATGGAACTJGACTACGTAAATT";
        //                 (?)

        assert!(
            ::dna_to_rna(dna).is_err(),
            "Unknown nucleobase should return an Error"
        );
    }
}

fn dna_base_to_rna_base(base: char) -> Result<char, String> {
    use rosalind::{ADENYNE, CYTOSINE, GUANINE, THYMINE, URACIL};

    match base {
        THYMINE => Ok(URACIL),
        ADENYNE | CYTOSINE | GUANINE => Ok(base),
        _ => Err(format!("Unexpected nucleobase: {}", base)),
    }
}

fn dna_to_rna(dna: &str) -> Result<String, String> {
    dna.chars()
        .map(|base: char| dna_base_to_rna_base(base))
        .collect()
}

fn main() {
    let dna = rosalind::io::load_data(file!()).expect("Couldn't open the file");

    println!("DNA string: {}", dna);

    let rna = dna_to_rna(&dna).expect("Error transcribing DNA to RNA!");

    println!("RNA string: {}", rna);
}
