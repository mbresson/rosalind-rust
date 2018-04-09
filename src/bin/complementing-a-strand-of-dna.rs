extern crate rosalind;

// solution to http://rosalind.info/problems/revc/

use rosalind::dna::Sequence;
use std::convert::TryFrom;

fn main() {
    let dna_str = rosalind::io::load_data(file!()).expect("Couldn't open the file");

    let dna = Sequence::try_from(dna_str.as_str()).expect("Couldn't parse the sequence");

    println!("DNA strand: {}", dna);

    let complemented_strand = dna.reverse_complement();

    println!("Reverse complement: {}", complemented_strand);
}
