extern crate rosalind;

// solution to http://rosalind.info/problems/fib/

#[cfg(test)]
mod tests {
    #[test]
    fn rabbit_pairs_after_n_months() {
        let k_new_pairs = 3;

        // we should have 4 pairs of rabbits after 3 months, 19 after 5 months, and so on...
        let months_and_expected_pairs = vec![(1, 1), (2, 1), (3, 4), (4, 7), (5, 19), (6, 40)];

        for (months, expected_pairs) in months_and_expected_pairs {
            let computed_pairs = ::rabbit_pairs_after_n_months(months, k_new_pairs);

            assert_eq!(computed_pairs, expected_pairs);
        }
    }
}

fn rabbit_pairs_after_n_months(mut n_months: u64, k_new_pairs: u64) -> u64 {
    if n_months < 3 {
        return 1;
    }

    let mut previous_month = 1;
    let mut previous_previous_month = 1;

    let mut rabbit_pairs = 0;

    while n_months > 2 {
        rabbit_pairs = previous_month + k_new_pairs * previous_previous_month;

        previous_previous_month = previous_month;
        previous_month = rabbit_pairs;

        n_months -= 1;
    }

    rabbit_pairs
}

// given a dataset string formatted as "N K", extract and parse the content
// as N = number of months and K = number of new rabbit pairs each month
fn dataset_to_n_months_and_k_new_pairs(dataset: &str) -> Result<(u64, u64), String> {
    let dataset_parts = dataset.split_whitespace().collect::<Vec<&str>>();

    if dataset_parts.len() != 2 {
        return Err(format!("Malformatted dataset: {}", dataset));
    }

    let n_months = match dataset_parts[0].parse::<u64>() {
        Err(error) => return Err(format!("Couldn't parse n_months: {}", error)),
        Ok(n_months) => n_months,
    };

    let k_new_pairs = match dataset_parts[1].parse::<u64>() {
        Err(error) => return Err(format!("Couldn't parse k_new_pairs: {}", error)),
        Ok(k_new_pairs) => k_new_pairs,
    };

    Ok((n_months, k_new_pairs))
}

fn main() {
    let dataset = rosalind::io::load_data(file!()).expect("Couldn't open the file");

    let (n_months, k_new_pairs) =
        dataset_to_n_months_and_k_new_pairs(&dataset).expect("Couldn't parse the file");

    println!("n_months = {}\nk_new_pairs = {}", n_months, k_new_pairs);

    let pairs = rabbit_pairs_after_n_months(n_months, k_new_pairs);

    println!("Total rabbit paris: {}", pairs);
}
