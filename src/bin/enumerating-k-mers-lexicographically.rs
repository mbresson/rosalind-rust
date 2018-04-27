#![feature(test)]

extern crate rosalind;
extern crate test;

// solution to http://rosalind.info/problems/lexf/

use rosalind::io;

#[cfg(test)]
mod tests {
    #[test]
    fn rec_compute_ordered_permutations_length_1() {
        // let's test with beautiful symbols from Thai, Chinese's Bopomofo system and Burmese script
        let ordered_symbols = &['A', 'ญ', 'ㄤ', 'ဪ'];

        let ordered_permutations = ::rec_compute_ordered_permutations_length_1(ordered_symbols);

        let expected_ordered_permutations = &[&['A'], &['ญ'], &['ㄤ'], &['ဪ']];

        assert_eq!(ordered_permutations, expected_ordered_permutations);
    }

    #[test]
    fn rec_compute_ordered_permutations() {
        let ordered_symbols = &['A', 'C', 'G', 'T'];

        let ordered_permutations = ::rec_compute_ordered_permutations(ordered_symbols, 3);

        let expected_ordered_permutations = vec![
            vec!['A', 'A', 'A'],
            vec!['A', 'A', 'C'],
            vec!['A', 'A', 'G'],
            vec!['A', 'A', 'T'],
            vec!['A', 'C', 'A'],
            vec!['A', 'C', 'C'],
            vec!['A', 'C', 'G'],
            vec!['A', 'C', 'T'],
            vec!['A', 'G', 'A'],
            vec!['A', 'G', 'C'],
            vec!['A', 'G', 'G'],
            vec!['A', 'G', 'T'],
            vec!['A', 'T', 'A'],
            vec!['A', 'T', 'C'],
            vec!['A', 'T', 'G'],
            vec!['A', 'T', 'T'],
            vec!['C', 'A', 'A'],
            vec!['C', 'A', 'C'],
            vec!['C', 'A', 'G'],
            vec!['C', 'A', 'T'],
            vec!['C', 'C', 'A'],
            vec!['C', 'C', 'C'],
            vec!['C', 'C', 'G'],
            vec!['C', 'C', 'T'],
            vec!['C', 'G', 'A'],
            vec!['C', 'G', 'C'],
            vec!['C', 'G', 'G'],
            vec!['C', 'G', 'T'],
            vec!['C', 'T', 'A'],
            vec!['C', 'T', 'C'],
            vec!['C', 'T', 'G'],
            vec!['C', 'T', 'T'],
            vec!['G', 'A', 'A'],
            vec!['G', 'A', 'C'],
            vec!['G', 'A', 'G'],
            vec!['G', 'A', 'T'],
            vec!['G', 'C', 'A'],
            vec!['G', 'C', 'C'],
            vec!['G', 'C', 'G'],
            vec!['G', 'C', 'T'],
            vec!['G', 'G', 'A'],
            vec!['G', 'G', 'C'],
            vec!['G', 'G', 'G'],
            vec!['G', 'G', 'T'],
            vec!['G', 'T', 'A'],
            vec!['G', 'T', 'C'],
            vec!['G', 'T', 'G'],
            vec!['G', 'T', 'T'],
            vec!['T', 'A', 'A'],
            vec!['T', 'A', 'C'],
            vec!['T', 'A', 'G'],
            vec!['T', 'A', 'T'],
            vec!['T', 'C', 'A'],
            vec!['T', 'C', 'C'],
            vec!['T', 'C', 'G'],
            vec!['T', 'C', 'T'],
            vec!['T', 'G', 'A'],
            vec!['T', 'G', 'C'],
            vec!['T', 'G', 'G'],
            vec!['T', 'G', 'T'],
            vec!['T', 'T', 'A'],
            vec!['T', 'T', 'C'],
            vec!['T', 'T', 'G'],
            vec!['T', 'T', 'T'],
        ];

        assert_eq!(ordered_permutations, expected_ordered_permutations);
    }

    #[bench]
    fn bench_rec_compute_ordered_permutations(b: &mut ::test::Bencher) {
        let ordered_symbols = &['A', 'C', 'G', 'T'];
        b.iter(|| ::rec_compute_ordered_permutations(ordered_symbols, 10));
    }
}

// let's dream that we can use guard clauses for permutations_length...
fn rec_compute_ordered_permutations_length_1(ordered_symbols: &[char]) -> Vec<Vec<char>> {
    ordered_symbols
        .iter()
        .cloned()
        .map(|symbol| vec![symbol])
        .collect()
}

fn rec_compute_ordered_permutations(
    ordered_symbols: &[char],
    permutations_length: u32,
) -> Vec<Vec<char>> {
    if permutations_length == 1 {
        return rec_compute_ordered_permutations_length_1(ordered_symbols);
    }

    let number_permutations = ordered_symbols.len().pow(permutations_length);
    let mut permutations = Vec::with_capacity(number_permutations);

    let permutations_tail =
        rec_compute_ordered_permutations(ordered_symbols, permutations_length - 1);

    for symbol in ordered_symbols {
        for tail in &permutations_tail {
            let mut permutation = Vec::with_capacity(permutations_length as usize);
            permutation.push(*symbol);

            permutation.extend(tail.iter().cloned());

            permutations.push(permutation);
        }
    }

    permutations
}

fn main() {
    let data = rosalind::io::load_data(file!()).expect("Couldn't open the file");

    let lines = data.lines().collect::<Vec<_>>();

    if lines.len() != 2 {
        panic!("Expected two lines of input, got {} lines", lines.len());
    }

    let ordered_symbols = io::parse_separated_values::<char>(lines[0], " ")
        .expect("Couldn't parse space-separated symbols");

    let permutations_length = lines[1]
        .parse::<u32>()
        .expect("Couldn't parse string length on second line");

    let ordered_permutations =
        rec_compute_ordered_permutations(&ordered_symbols, permutations_length);

    let print_permutation = |permutation: &Vec<char>| {
        for ch in permutation {
            print!("{}", ch);
        }

        println!()
    };

    for permutation in &ordered_permutations {
        print_permutation(permutation);
    }
}
