extern crate rosalind;

// solution to http://rosalind.info/problems/revc/

use rosalind::dna::Sequence;
use std::convert::TryFrom;

fn main() {
    let dna_strand = Sequence::try_from(
        rosalind::io::load_data(file!())
            .expect("Couldn't open the file")
            .as_ref(),
    ).unwrap();

    println!("DNA strand: {}", dna_strand);

    let complemented_strand = dna_strand.reverse_complement();

    println!("Reverse complement: {}", complemented_strand);
}
