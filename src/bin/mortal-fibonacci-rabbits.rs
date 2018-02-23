extern crate num_bigint;
extern crate rosalind;

use std::str::FromStr;
use num_bigint::{BigUint, ToBigUint};

// solution to http://rosalind.info/problems/fibd/

#[cfg(test)]
mod tests {
    use num_bigint::ToBigUint;

    #[test]
    fn rabbit_pairs_after_6_months() {
        let n_months = 6;
        let m_life_expectancy = 3;

        let rabbit_pairs = ::rabbit_pairs_after_n_months(n_months, m_life_expectancy);

        assert_eq!(rabbit_pairs, 4_i32.to_biguint().unwrap());
    }

    #[test]
    fn rabbit_pairs_after_8_months() {
        let n_months = 8;
        let m_life_expectancy = 3;

        let rabbit_pairs = ::rabbit_pairs_after_n_months(n_months, m_life_expectancy);

        assert_eq!(rabbit_pairs, 7_i32.to_biguint().unwrap());
    }

    #[test]
    fn rabbit_pairs_after_8_months_life_expectancy_2_months() {
        let n_months = 8;
        let m_life_expectancy = 2;

        let rabbit_pairs = ::rabbit_pairs_after_n_months(n_months, m_life_expectancy);

        assert_eq!(rabbit_pairs, 1_i32.to_biguint().unwrap());
    }
}

// parse an arbitrary number of values from `values_string` separated by `separator`
fn parse_separated_values<F: FromStr>(
    values_string: &str,
    separator: &str,
) -> Result<Vec<F>, F::Err> {
    values_string
        .split(separator)
        .map(|raw_value| raw_value.parse::<F>())
        .collect::<Result<Vec<F>, _>>()
}

fn step_rabbit_pairs_population(population: &Vec<BigUint>) -> Vec<BigUint> {
    let mut next_population: Vec<BigUint> = vec![0.to_biguint().unwrap(); population.len()];

    let mut generation_index = population.len() - 1;
    while generation_index > 0 {
        next_population[0] += &population[generation_index];

        next_population[generation_index] = population[generation_index - 1].clone();

        generation_index -= 1;
    }

    next_population
}

fn rabbit_pairs_after_n_months(n_months: u32, m_life_expectancy: u32) -> BigUint {
    let mut rabbit_population = vec![0.to_biguint().unwrap(); m_life_expectancy as usize];

    rabbit_population[0] = 1.to_biguint().unwrap();

    let mut month = 1;
    while month < n_months {
        rabbit_population = step_rabbit_pairs_population(&mut rabbit_population);
        month += 1;
    }

    let mut total_rabbit_pairs = 0.to_biguint().unwrap();
    for rabbit_pairs in rabbit_population {
        total_rabbit_pairs += rabbit_pairs;
    }

    total_rabbit_pairs
}

fn main() {
    let dataset = rosalind::io::load_data(file!()).expect("Couldn't open the file");

    let values = parse_separated_values(&dataset, " ").expect("Couldn't parse values");

    let (n_months, m_life_expectancy) = (values[0], values[1]);

    println!(
        "n_months = {}\nm_life_expectancy = {}",
        n_months, m_life_expectancy
    );

    let rabbit_pairs = rabbit_pairs_after_n_months(n_months, m_life_expectancy);

    println!("Total rabbit pairs: {}", rabbit_pairs);
}
