extern crate rosalind;

// solution to http://rosalind.info/problems/dna/

use rosalind::dna::sequence::Sequence;
use std::convert::TryFrom;

fn main() {
    let dna = rosalind::io::load_data(file!()).expect("Couldn't open the file");

    println!("DNA string: {}", dna);

    let count = Sequence::try_from(dna.as_ref())
        .unwrap()
        .count_nucleobases();

    println!(
        "Adenine: {}\nThymine: {}\nCytosine: {}\nGuanine: {}",
        count.adenines, count.thymines, count.cytosines, count.guanines
    );
}
