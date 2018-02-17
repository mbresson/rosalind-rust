extern crate rosalind;

// solution to http://rosalind.info/problems/dna/

const FILENAME: &'static str = "data/counting-dna-nucleotides.txt";

fn main() {
    let dna = rosalind::io::load_file_to_string(FILENAME).expect("Couldn't open the file");

    println!("DNA string: {}", dna);

    let (nb_adenyne, nb_thymine, nb_cytosine, nb_guanine) =
        rosalind::count_nucleotides(&dna).expect("Error counting nucleotides!");

    println!(
        "Adenyne: {}\nThymine: {}\nCytosine: {}\nGuanine: {}",
        nb_adenyne, nb_thymine, nb_cytosine, nb_guanine
    );
}
