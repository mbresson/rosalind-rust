extern crate rosalind;

// solution to http://rosalind.info/problems/iprb/

use rosalind::probabilities::{PROBABILITY_K_AND_K_PRODUCING_DOMINANT_ALLELE,
                              PROBABILITY_K_AND_M_PRODUCING_DOMINANT_ALLELE,
                              PROBABILITY_K_AND_N_PRODUCING_DOMINANT_ALLELE,
                              PROBABILITY_M_AND_M_PRODUCING_DOMINANT_ALLELE,
                              PROBABILITY_M_AND_N_PRODUCING_DOMINANT_ALLELE,
                              PROBABILITY_N_AND_N_PRODUCING_DOMINANT_ALLELE};

#[cfg(test)]
mod tests {
    #[test]
    fn compute_probability_of_producing_dominant_allele() {
        let k = 2;
        let m = 2;
        let n = 2;

        let probability = ::compute_probability_of_producing_dominant_allele(k, m, n);
        let expected_probability = 0.78333;

        assert!((probability - expected_probability).abs() < 0.01);
    }

    #[test]
    fn compute_probability_of_producing_dominant_allele_2() {
        let k = 2;
        let m = 3;
        let n = 4;

        let probability = ::compute_probability_of_producing_dominant_allele(k, m, n);
        let expected_probability = 0.64583333;

        assert!((probability - expected_probability).abs() < 0.01);
    }
}

// given a dataset string formatted as three space-separated numbers "a b c",
// extract, parse and return the numbers
fn dataset_to_a_b_c_numbers(dataset: &str) -> Result<(u32, u32, u32), String> {
    let parsed_numbers = dataset
        .split_whitespace()
        .map(|number| number.parse::<u32>())
        .collect::<Result<Vec<_>, _>>();

    match parsed_numbers {
        Err(e) => Err(format!("Couldn't parse dataset: {}", e)),

        Ok(numbers) => {
            if numbers.len() != 3 {
                Err(format!(
                    "Expected 3 numbers, got {}: {:?}",
                    numbers.len(),
                    numbers
                ))
            } else {
                Ok((numbers[0], numbers[1], numbers[2]))
            }
        }
    }
}

// k, m, and n are numbers representing, for some factor (factor <=> pair of alleles):
// k = number of homozygous dominant individuals
// m = number of heterozygous individuals
// n = number of homozygous recessive individuals
fn compute_probability_of_producing_dominant_allele(k_i: u32, m_i: u32, n_i: u32) -> f64 {
    let (k, m, n) = (k_i as f64, m_i as f64, n_i as f64);
    let total_organisms = k + m + n;

    /*
     * probabilities with first parent = 'k'
     */
    let pr_first_parent_is_k = k / total_organisms;

    // 'k' with 'k'
    let pr_first_parent_is_k_second_parent_is_k =
        pr_first_parent_is_k * ((k - 1.0) / (total_organisms - 1.0));

    // 'k' with 'm'
    let pr_first_parent_is_k_second_parent_is_m =
        pr_first_parent_is_k * (m / (total_organisms - 1.0));

    // 'k' with 'n'
    let pr_first_parent_is_k_second_parent_is_n =
        pr_first_parent_is_k * (n / (total_organisms - 1.0));

    /*
     * probabilities with first parent = 'm'
     */
    let pr_first_parent_is_m = m / total_organisms;

    // 'm' with 'k' (= 'k' with 'm')
    let pr_first_parent_is_m_second_parent_is_k = pr_first_parent_is_k_second_parent_is_m;

    // 'm' with 'm'
    let pr_first_parent_is_m_second_parent_is_m =
        pr_first_parent_is_m * ((m - 1.0) / (total_organisms - 1.0));

    // 'm' with 'n'
    let pr_first_parent_is_m_second_parent_is_n =
        pr_first_parent_is_m * (n / (total_organisms - 1.0));

    /*
     * probabilities with first parent = 'n'
     */
    let pr_first_parent_is_n = n / total_organisms;

    // 'n' with 'k' (= 'k' with 'n')
    let pr_first_parent_is_n_second_parent_is_k = pr_first_parent_is_k_second_parent_is_n;

    // 'n' with 'm' (= 'm' with 'n')
    let pr_first_parent_is_n_second_parent_is_m = pr_first_parent_is_m_second_parent_is_n;

    // 'n' with 'n'
    let pr_first_parent_is_n_second_parent_is_n =
        pr_first_parent_is_n * ((n - 1.0) / (total_organisms - 1.0));

    (pr_first_parent_is_k_second_parent_is_k * PROBABILITY_K_AND_K_PRODUCING_DOMINANT_ALLELE)
        + (pr_first_parent_is_k_second_parent_is_m * PROBABILITY_K_AND_M_PRODUCING_DOMINANT_ALLELE)
        + (pr_first_parent_is_k_second_parent_is_n * PROBABILITY_K_AND_N_PRODUCING_DOMINANT_ALLELE)
        + (pr_first_parent_is_m_second_parent_is_k * PROBABILITY_K_AND_M_PRODUCING_DOMINANT_ALLELE)
        + (pr_first_parent_is_m_second_parent_is_m * PROBABILITY_M_AND_M_PRODUCING_DOMINANT_ALLELE)
        + (pr_first_parent_is_m_second_parent_is_n * PROBABILITY_M_AND_N_PRODUCING_DOMINANT_ALLELE)
        + (pr_first_parent_is_n_second_parent_is_k * PROBABILITY_K_AND_N_PRODUCING_DOMINANT_ALLELE)
        + (pr_first_parent_is_n_second_parent_is_m * PROBABILITY_M_AND_N_PRODUCING_DOMINANT_ALLELE)
        + (pr_first_parent_is_n_second_parent_is_n * PROBABILITY_N_AND_N_PRODUCING_DOMINANT_ALLELE)
}

fn main() {
    let dataset = rosalind::io::load_data(file!()).expect("Couldn't open the file");

    let (k_homozygous_dominant, m_heterozygous, n_homozygous_recessive) =
        dataset_to_a_b_c_numbers(&dataset).expect("Couldn't parse the file");

    println!(
        "k = {}, m = {}, n = {}",
        k_homozygous_dominant, m_heterozygous, n_homozygous_recessive
    );

    let probability = compute_probability_of_producing_dominant_allele(
        k_homozygous_dominant,
        m_heterozygous,
        n_homozygous_recessive,
    );

    println!("{}", probability);
}
