extern crate num_bigint;
extern crate num_traits;
extern crate rosalind;

use num_bigint::{BigUint, ToBigUint};
use num_traits::cast::ToPrimitive;
use rosalind::io::parse_separated_values;

// solution to http://rosalind.info/problems/lia/

#[cfg(test)]
mod tests {
    use num_bigint::ToBigUint;

    #[test]
    fn probability_at_least_n_successful_organisms_in_generation_k() {
        let generation_k = 2;
        let at_least_n = 1;

        let expected_probability = 0.684;
        let result =
            ::probability_at_least_n_successful_organisms_in_generation_k(at_least_n, generation_k);

        assert!((expected_probability - result).abs() < 0.001);
    }

    #[test]
    fn factorial() {
        assert_eq!(::factorial(0), 1.to_biguint().unwrap());
        assert_eq!(::factorial(1), 1.to_biguint().unwrap());
        assert_eq!(::factorial(2), 2.to_biguint().unwrap());
        assert_eq!(::factorial(3), 6.to_biguint().unwrap());
        assert_eq!(::factorial(10), 3628800.to_biguint().unwrap());
    }
}

const PROBABILITY_SUCCESSFUL: f64 = 0.25;
const PROBABILITY_OTHER: f64 = 0.75;

fn probability_at_least_n_successful_organisms_in_generation_k(
    at_least_n: u32,
    generation_k: u32,
) -> f64 {
    let total_offsprings = 2_u32.pow(generation_k);

    let mut probability = 0.0;

    for offsprings in at_least_n..total_offsprings + 1 {
        probability += PROBABILITY_SUCCESSFUL.powf(offsprings.into())
            * PROBABILITY_OTHER.powf((total_offsprings - offsprings).into())
            * compute_n_choose_k(offsprings, total_offsprings);
    }

    probability
}

fn compute_n_choose_k(n: u32, k: u32) -> f64 {
    let factorial_k = factorial(k).to_f64().unwrap();
    let factorial_n = factorial(n).to_f64().unwrap();
    let factorial_k_minus_n = factorial(k - n).to_f64().unwrap();

    factorial_k / (factorial_n * factorial_k_minus_n)
}

fn factorial(n: u32) -> BigUint {
    if n == 0 {
        return 1.to_biguint().unwrap();
    }

    let mut total = n.to_biguint().unwrap();
    let mut multiplier = n - 1;
    while multiplier > 1 {
        total *= multiplier;
        multiplier -= 1;
    }

    total
}

/*
 * For this exercise, I spent a lot of time trying to come up with a solution,
 * but I realized that I lacked the math concept of binomial distribution to solve it by myself.
 * Hence, I didn't find the solution alone but instead read the explanations of another Rosalind solver: https://adijo.github.io/2016/01/12/rosalind-independent-alleles/
 * Even though I took the time to redo the exercise and understand the solution, this is not my original solution,
 * so I decided to implement it in Rust only for the sake of completing the exercise.
 */

fn main() {
    let dataset = rosalind::io::load_data(file!()).expect("Couldn't open the file");

    let values = parse_separated_values::<u32>(&dataset, " ").expect("Couldn't parse values");

    let (generation_k, at_least_n) = (values[0], values[1]);

    let probability =
        probability_at_least_n_successful_organisms_in_generation_k(at_least_n, generation_k);

    println!("{}", probability);
}
