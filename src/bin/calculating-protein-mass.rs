extern crate reqwest;
extern crate rosalind;

// solution to http://rosalind.info/problems/prtm/

use std::convert::TryFrom;
use rosalind::amino_acids::Sequence;

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
    use rosalind::amino_acids::Sequence;

    #[test]
    fn compute_total_weight() {
        let sequence = Sequence::try_from("SKADYEK").unwrap();

        let total_weight = ::compute_total_weight(&sequence);

        let expected_total_weight = 821.392;

        assert!((total_weight - expected_total_weight).abs() < 0.01);
    }
}

fn compute_total_weight(sequence: &Sequence) -> f64 {
    let mut total_weight = 0.0;

    for aa in sequence {
        total_weight += aa.monoisotopic_mass();
    }

    total_weight
}

fn main() {
    let protein_str = rosalind::io::load_data(file!()).expect("Couldn't open the file");

    let sequence = Sequence::try_from(protein_str.as_str()).unwrap();

    println!("{}", compute_total_weight(&sequence));
}
