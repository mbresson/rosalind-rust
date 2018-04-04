extern crate rosalind;

use std::fmt;

// solution to http://rosalind.info/problems/perm/

#[cfg(test)]
mod tests {
    #[test]
    fn compute_permutations_3() {
        let mut expected_permutations = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];
        expected_permutations.sort();

        let mut permutations = ::compute_permutations(3);
        permutations.sort();

        assert_eq!(permutations, expected_permutations);
    }

    #[test]
    fn compute_permutations_4() {
        let mut expected_permutations = vec![
            [1, 2, 3, 4],
            [1, 2, 4, 3],
            [1, 3, 2, 4],
            [1, 3, 4, 2],
            [1, 4, 2, 3],
            [1, 4, 3, 2],
            [2, 1, 3, 4],
            [2, 1, 4, 3],
            [2, 3, 1, 4],
            [2, 3, 4, 1],
            [2, 4, 1, 3],
            [2, 4, 3, 1],
            [3, 1, 2, 4],
            [3, 1, 4, 2],
            [3, 2, 1, 4],
            [3, 2, 4, 1],
            [3, 4, 1, 2],
            [3, 4, 2, 1],
            [4, 1, 2, 3],
            [4, 1, 3, 2],
            [4, 2, 1, 3],
            [4, 2, 3, 1],
            [4, 3, 1, 2],
            [4, 3, 2, 1],
        ];

        expected_permutations.sort();

        let mut permutations = ::compute_permutations(4);
        permutations.sort();

        println!("{:?}", permutations);

        assert_eq!(permutations, expected_permutations);
    }
}

fn compute_permutations(n: u32) -> Vec<Vec<u32>> {
    match n {
        0 => vec![],
        1 => vec![vec![1]],
        2 => vec![vec![1, 2], vec![2, 1]],
        n => {
            let mut permutations = vec![];

            let permutations_n_minus_1 = compute_permutations(n - 1);

            for i in 1..(n + 1) {
                for permutation in &permutations_n_minus_1 {
                    let mut permutation_i = vec![i];

                    let p = permutation
                        .into_iter()
                        .map(|value| if *value == i { n } else { *value })
                        .collect::<Vec<u32>>();

                    permutation_i.extend_from_slice(&p);

                    permutations.push(permutation_i);
                }
            }

            permutations
        }
    }
}

struct Array<'a, T: 'a + fmt::Display>(pub &'a Vec<T>);

impl<'a, T> fmt::Display for Array<'a, T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (index, item) in self.0.iter().enumerate() {
            if index == 0 {
                try!(write!(f, "{}", item));
            } else {
                try!(write!(f, " {}", item));
            }
        }

        Ok(())
    }
}

fn main() {
    let n = rosalind::io::load_data(file!())
        .expect("Couldn't open the file")
        .parse::<u32>()
        .expect("Couldn't parse data string to number");

    let permutations = compute_permutations(n);

    println!("{}", permutations.len());

    for permutation in permutations {
        println!("{}", Array(&permutation));
    }
}
