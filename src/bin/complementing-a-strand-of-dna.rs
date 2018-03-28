extern crate rosalind;

// solution to http://rosalind.info/problems/revc/

fn main() {
    let dna_strand = rosalind::io::load_data(file!()).expect("Couldn't open the file");

    println!("DNA strand: {}", dna_strand);

    let complemented_strand = rosalind::reverse_complement_dna_strand(&dna_strand)
        .expect("Error complementing DNA strand!");

    println!("Reverse complement: {}", complemented_strand);
}
