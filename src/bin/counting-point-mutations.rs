extern crate rosalind;

// solution to http://rosalind.info/problems/hamm/

#[cfg(test)]
mod tests {
    #[test]
    fn count_point_mutations() {
        let strand_a = "GAGCCTACTAACGGGAT";
        let strand_b = "CATCGTAATGACGGCCT";

        let point_mutations = ::count_point_mutations(strand_a, strand_b);

        assert_eq!(point_mutations, 7);
    }

    #[test]
    fn count_point_mutations_unequal_length_strands() {
        let strand_a = "GAGCCTACTAACGGGAT";
        let strand_b = "GCGTAATGAAAG";

        let point_mutations = ::count_point_mutations(strand_a, strand_b);

        assert_eq!(point_mutations, 8);
    }
}

fn count_point_mutations(dna_strand_a: &str, dna_strand_b: &str) -> u32 {
    let mut strand_a_bases = dna_strand_a.chars();

    let mut point_mutations = 0;

    for base_strand_b in dna_strand_b.chars() {
        match strand_a_bases.next() {
            Some(base_strand_a) if base_strand_a != base_strand_b => {
                point_mutations += 1;
            }
            Some(_) => {}
            None => {
                break;
            }
        }
    }

    point_mutations
}

fn main() {
    let content = rosalind::io::load_data(file!()).expect("Couldn't open the file");

    let dna_strings = content.lines().collect::<Vec<&str>>();

    if dna_strings.len() != 2 {
        panic!("Expected two DNA strings, got {}", dna_strings.len());
    }

    let (strand_a, strand_b) = (dna_strings[0], dna_strings[1]);

    let point_mutations = count_point_mutations(strand_a, strand_b);

    println!(
        "Strand A: {}\nStrand B: {}\nPoint mutations: {}",
        strand_a, strand_b, point_mutations
    );
}
