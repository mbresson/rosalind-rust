extern crate rosalind;

// solution to http://rosalind.info/problems/prot/

#[cfg(test)]
mod tests {
    extern crate rosalind;

    #[test]
    fn amino_acids_from_rna() {
        let rna = "AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGGUGA";

        let aas = rosalind::amino_acids::amino_acids_from_rna(&rna)
            .expect("Couldn't translate RNA to AA!");

        let aas_string = aas.iter().map(|acid| acid.to_string()).collect::<String>();

        assert_eq!(aas_string, "MAMAPRTEINSTRING");
    }
}

fn main() {
    let messenger_rna = rosalind::io::load_data(file!()).expect("Couldn't open the file");

    println!("RNA string: {}", messenger_rna);

    let amino_acids = rosalind::amino_acids::amino_acids_from_rna(&messenger_rna)
        .expect("Error translating mRNA into amino acids!");

    println!(
        "AA string: {}",
        rosalind::amino_acids::AminoAcidString(&amino_acids)
    );
}
