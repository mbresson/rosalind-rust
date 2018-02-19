extern crate rosalind;

// solution to http://rosalind.info/problems/dna/

fn main() {
    let dna = rosalind::io::load_data(file!()).expect("Couldn't open the file");

    println!("DNA string: {}", dna);

    let (nb_adenyne, nb_thymine, nb_cytosine, nb_guanine) =
        rosalind::count_nucleotides(&dna).expect("Error counting nucleotides!");

    println!(
        "Adenyne: {}\nThymine: {}\nCytosine: {}\nGuanine: {}",
        nb_adenyne, nb_thymine, nb_cytosine, nb_guanine
    );
}
