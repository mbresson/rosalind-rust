extern crate rosalind;

// solution to http://rosalind.info/problems/gc/

use rosalind::dna;
use std::convert::TryFrom;

#[cfg(test)]
mod tests {
    #[test]
    fn compute_gc_content() {
        let sequence = "CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGACTGGGAACCTGCGGGCAGTAGGTGGAAT";

        let gc_content = ::compute_gc_content(sequence);

        let expected_gc_content = 0.60919540;

        assert!((gc_content - expected_gc_content).abs() < 0.01);
    }
}

// returns the GC content as a float between 0 and 1
fn compute_gc_content(sequence: &str) -> f64 {
    let count = dna::Sequence::try_from(sequence)
        .unwrap()
        .count_nucleobases();

    let nb_gc = (count.cytosines + count.guanines) as f64;
    let nb_bases = (count.adenines + count.thymines + count.cytosines + count.guanines) as f64;

    nb_gc / nb_bases
}

fn main() {
    let fasta_content = rosalind::io::load_data(file!()).expect("Couldn't open the file");

    let sequences = rosalind::fasta::parse_fasta_format_to_map(&fasta_content)
        .expect("Couldn't parse FASTA data");

    for (label, sequence) in sequences {
        let gc_content = compute_gc_content(&sequence);

        println!("{}\n{}", label, gc_content * 100.0);
    }
}
