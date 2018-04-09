extern crate rosalind;

// solution to http://rosalind.info/problems/rna/

use std::convert::TryFrom;
use rosalind::{dna::Sequence as DnaSequence, rna::Sequence as RnaSequence};

fn main() {
    let dna_string = rosalind::io::load_data(file!()).expect("Couldn't open the file");

    let dna = DnaSequence::try_from(dna_string.as_str()).expect("Couldn't parse the sequence");

    println!("DNA string: {}", dna);

    let rna = RnaSequence::from(&dna);

    println!("RNA string: {}", rna);
}
