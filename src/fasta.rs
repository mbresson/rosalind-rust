use std::collections::HashMap;

pub type Label = String;
pub type Sequence = String;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn parse_fasta_format_to_map() {
        let raw_data = r"
>Rosalind_6404
CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCC
TCCCACTAATAATTCTGAGG
>Rosalind_5959
CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCT
ATATCCATTTGTCAGCAGACACGC
>Rosalind_0808
CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGAC
TGGGAACCTGCGGGCAGTAGGTGGAAT
";

        let fasta_data =
            ::fasta::parse_fasta_format_to_map(&raw_data).expect("Error parsing FASTA data!");

        let expected_data: HashMap<::fasta::Label, ::fasta::Sequence> = [
            ("Rosalind_6404".to_string(), "CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCCTCCCACTAATAATTCTGAGG".to_string()),
            ("Rosalind_5959".to_string(), "CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCTATATCCATTTGTCAGCAGACACGC".to_string()),
            ("Rosalind_0808".to_string(), "CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGACTGGGAACCTGCGGGCAGTAGGTGGAAT".to_string()),
        ].iter().cloned().collect();

        assert_eq!(expected_data, fasta_data);
    }

    #[test]
    fn parse_fasta_format_to_map_with_empty_data() {
        let fasta_data = ::fasta::parse_fasta_format_to_map("").expect("Error parsing FASTA data!");

        let expected_data = HashMap::new();

        assert_eq!(expected_data, fasta_data);
    }

    #[test]
    fn parse_fasta_format_to_map_errors_with_missing_label() {
        let raw_data = r"

; missing sequence label!
CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCC
TCCCACTAATAATTCTGAGG

>Rosalind_5959
CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCT
ATATCCATTTGTCAGCAGACACGC

>Rosalind_0808
CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGAC
TGGGAACCTGCGGGCAGTAGGTGGAAT
";

        assert!(::fasta::parse_fasta_format_to_map(&raw_data).is_err());
    }
}

pub fn parse_fasta_format_to_map(fasta_content: &str) -> Result<HashMap<Label, Sequence>, String> {
    let mut data = HashMap::new();

    let meaningful_lines = fasta_content
        .lines()
        .map(extract_meaningful_line)
        .filter_map(|line| line); // skip None-s (<=> non-meaningful lines such as comments)

    let mut current_label_and_sequence: Option<(Label, Sequence)> = None;

    for line in meaningful_lines {
        let is_new_sequence = line.starts_with(">");

        if is_new_sequence {
            if let Some((label, sequence)) = current_label_and_sequence {
                data.insert(label, sequence);
            }

            current_label_and_sequence = Some((line[1..].to_string(), String::new()));
        } else {
            match current_label_and_sequence {
                Some((label, mut sequence)) => {
                    sequence.push_str(line);

                    current_label_and_sequence = Some((label, sequence));
                }
                None => {
                    return Err("Malformatted FASTA data, got a sequence without preceding label ('>' character missing)".to_string());
                }
            }
        }
    }

    if let Some((label, sequence)) = current_label_and_sequence {
        data.insert(label, sequence);
    }

    Ok(data)
}

// returns None if the line is not meaningful (is a comment or an empty line)
// else, returns the line, stripped of any leading and trailing space
fn extract_meaningful_line(fasta_line: &str) -> Option<&str> {
    let stripped_line = fasta_line.trim();

    let first_char = stripped_line.chars().next();

    first_char.and_then(|first_char| match first_char {
        ';' => None,
        _ => Some(stripped_line),
    }) // if line was empty, first_char was None so None is returned
}
