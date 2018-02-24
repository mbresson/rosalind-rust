extern crate rosalind;

// solution to http://rosalind.info/problems/iev/

use rosalind::io::parse_separated_values;

use rosalind::probabilities::{PROBABILITY_K_AND_K_PRODUCING_DOMINANT_ALLELE,
                              PROBABILITY_K_AND_M_PRODUCING_DOMINANT_ALLELE,
                              PROBABILITY_K_AND_N_PRODUCING_DOMINANT_ALLELE,
                              PROBABILITY_M_AND_M_PRODUCING_DOMINANT_ALLELE,
                              PROBABILITY_M_AND_N_PRODUCING_DOMINANT_ALLELE,
                              PROBABILITY_N_AND_N_PRODUCING_DOMINANT_ALLELE};

#[cfg(test)]
mod tests {
    #[test]
    fn average_offsprings_with_dominant_phenotype() {
        let average = ::average_offsprings_with_dominant_phenotype(1, 0, 0, 1, 0, 1);

        assert_eq!(average, 3.5);
    }
}

fn average_offsprings_with_dominant_phenotype(
    couples_k_k: u32,
    couples_k_m: u32,
    couples_k_n: u32,
    couples_m_m: u32,
    couples_m_n: u32,
    couples_n_n: u32,
) -> f64 {
    let couple_offsprings = 2.0;

    (couples_k_k as f64 * PROBABILITY_K_AND_K_PRODUCING_DOMINANT_ALLELE * couple_offsprings)
        + (couples_k_m as f64 * PROBABILITY_K_AND_M_PRODUCING_DOMINANT_ALLELE * couple_offsprings)
        + (couples_k_n as f64 * PROBABILITY_K_AND_N_PRODUCING_DOMINANT_ALLELE * couple_offsprings)
        + (couples_m_m as f64 * PROBABILITY_M_AND_M_PRODUCING_DOMINANT_ALLELE * couple_offsprings)
        + (couples_m_n as f64 * PROBABILITY_M_AND_N_PRODUCING_DOMINANT_ALLELE * couple_offsprings)
        + (couples_n_n as f64 * PROBABILITY_N_AND_N_PRODUCING_DOMINANT_ALLELE * couple_offsprings)
}

fn main() {
    let dataset = rosalind::io::load_data(file!()).expect("Couldn't open the file");

    let values = parse_separated_values::<u32>(&dataset, " ").expect("Couldn't parse values");

    // number of couples that are AA-AA, AA-Aa, AA-aa, Aa-Aa, Aa-aa, aa-aa
    let (couples_k_k, couples_k_m, couples_k_n, couples_m_m, couples_m_n, couples_n_n) = (
        values[0],
        values[1],
        values[2],
        values[3],
        values[4],
        values[5],
    );

    let average_offsprings = average_offsprings_with_dominant_phenotype(
        couples_k_k,
        couples_k_m,
        couples_k_n,
        couples_m_m,
        couples_m_n,
        couples_n_n,
    );

    println!("{}", average_offsprings);
}
