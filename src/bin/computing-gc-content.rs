extern crate rosalind;

// solution to http://rosalind.info/problems/gc/

const FILENAME: &'static str = "data/computing-gc-content.txt";

#[cfg(test)]
mod tests {
    #[test]
    fn compute_gc_content() {
        let sequence = "CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGACTGGGAACCTGCGGGCAGTAGGTGGAAT";

        let gc_content = ::compute_gc_content(sequence).expect("Couldn't compute GC content");

        let expected_gc_content = 0.60919540;

        assert!((gc_content - expected_gc_content).abs() < 0.01);
    }
}

// returns the GC content as a float between 0 and 1
fn compute_gc_content(sequence: &str) -> Result<f64, String> {
    let (nb_adenyne, nb_thymine, nb_cytosine, nb_guanine) = rosalind::count_nucleotides(sequence)?;

    let nb_gc = (nb_cytosine + nb_guanine) as f64;
    let nb_bases = (nb_adenyne + nb_thymine + nb_cytosine + nb_guanine) as f64;

    Ok(nb_gc / nb_bases)
}

fn main() {
    let fasta_content =
        rosalind::io::load_file_to_string(FILENAME).expect("Couldn't open the file");

    let sequences = rosalind::fasta::parse_fasta_format_to_map(&fasta_content)
        .expect("Couldn't parse FASTA data");

    for (label, sequence) in sequences {
        let gc_content =
            compute_gc_content(&sequence).expect("Couldn't compute GC content of sequence!");

        println!("{}\n{}", label, gc_content * 100.0);
    }
}
