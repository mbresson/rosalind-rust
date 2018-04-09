extern crate rosalind;

// solution to http://rosalind.info/problems/prot/

use std::convert::TryFrom;
use rosalind::{amino_acids::Sequence as AaSequence, rna::Sequence as RnaSequence};

fn main() {
    let rna_string = rosalind::io::load_data(file!()).expect("Couldn't open the file");

    let rna: RnaSequence =
        RnaSequence::try_from(rna_string.as_str()).expect("Couldn't parse the sequence");

    println!("RNA string: {}", rna);

    let amino_acids = AaSequence::from(&rna);

    println!("AA string: {}", amino_acids);
}
