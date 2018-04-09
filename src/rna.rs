pub use self::nucleobase::Nucleobase;
pub use self::sequence::{Sequence, StrictCodonIterator};

pub mod nucleobase {
    #[cfg(test)]
    mod tests {
        use dna::Nucleobase as DnaNucleobase;

        #[test]
        fn from_dna_nucleobase() {
            use super::{Nucleobase, Nucleobase::*};

            assert_eq!(Nucleobase::from(DnaNucleobase::Adenine), Adenine);
            assert_eq!(Nucleobase::from(DnaNucleobase::Thymine), Uracil);
            assert_eq!(Nucleobase::from(DnaNucleobase::Cytosine), Cytosine);
            assert_eq!(Nucleobase::from(DnaNucleobase::Guanine), Guanine);
        }
    }

    use std::{convert, error, fmt};
    use dna::Nucleobase as DnaNucleobase;

    #[derive(Clone, Debug, PartialEq)]
    pub enum Nucleobase {
        Adenine,
        Uracil,
        Cytosine,
        Guanine,
    }

    impl fmt::Display for Nucleobase {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use self::Nucleobase::*;

            match self {
                Adenine => write!(f, "A"),
                Uracil => write!(f, "U"),
                Cytosine => write!(f, "C"),
                Guanine => write!(f, "G"),
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum ParseError {
        IllegalChar { ch: char },
    }

    impl error::Error for ParseError {
        fn description(&self) -> &str {
            match self {
                ParseError::IllegalChar { .. } => {
                    "there is no such RNA nucleobase as represented by this character"
                }
            }
        }
    }

    impl fmt::Display for ParseError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ParseError::IllegalChar { ch } => write!(
                    f,
                    "there is no such RNA nucleobase as represented by character {}",
                    ch
                ),
            }
        }
    }

    impl convert::TryFrom<char> for Nucleobase {
        type Error = ParseError;

        /// Tries to parse a single char to its corresponding RNA nucleobase.
        ///
        /// # Examples
        ///
        /// ```
        /// use std::convert::TryFrom;
        ///
        /// match rosalind::rna::Nucleobase::try_from('C') {
        ///     Ok(nucleobase) => println!("{}", nucleobase),
        ///     Err(error) => println!("{:?}", error),
        /// }
        /// ```
        fn try_from(ch: char) -> Result<Self, Self::Error> {
            use self::Nucleobase::{Adenine, Cytosine, Guanine, Uracil};

            let nucleobase = match ch {
                'A' => Adenine,
                'U' => Uracil,
                'C' => Cytosine,
                'G' => Guanine,
                _ => {
                    return Err(ParseError::IllegalChar { ch: ch });
                }
            };

            Ok(nucleobase)
        }
    }

    impl From<DnaNucleobase> for Nucleobase {
        /// Converts a DNA nucleobase to its equivalent RNA nucleobase.
        ///
        /// # Examples
        ///
        /// ```
        /// use rosalind::dna::Nucleobase as DnaNucleobase;
        /// use rosalind::rna::Nucleobase as RnaNucleobase;
        ///
        /// let dna_thymine = DnaNucleobase::Thymine;
        /// let rna_uracil = RnaNucleobase::from(dna_thymine);
        /// ```
        fn from(dna_nucleobase: DnaNucleobase) -> Self {
            match dna_nucleobase {
                DnaNucleobase::Adenine => Nucleobase::Adenine,
                DnaNucleobase::Thymine => Nucleobase::Uracil,
                DnaNucleobase::Guanine => Nucleobase::Guanine,
                DnaNucleobase::Cytosine => Nucleobase::Cytosine,
            }
        }
    }
}

pub mod sequence {
    #[cfg(test)]
    mod tests {
        use super::Sequence;
        use super::super::nucleobase;
        use std::convert::TryFrom;

        #[test]
        fn from_dna_sequence() {
            let dna_sequence = ::dna::Sequence::try_from("AATGGCCAT").unwrap();

            let rna_sequence = Sequence::from(&dna_sequence);

            assert_eq!(rna_sequence.to_string(), "AAUGGCCAU");
        }

        #[test]
        fn try_from_erroneous_str() {
            assert_eq!(
                Sequence::try_from("AUCXCG").unwrap_err(),
                super::ParseError::NucleobaseError {
                    index: 3,
                    error: nucleobase::ParseError::IllegalChar { ch: 'X' },
                },
            );
        }

        #[test]
        fn try_from_str() {
            use super::Nucleobase::*;

            let sequence = "AAUGCGA";

            let expected_sequence = Sequence(vec![
                Adenine, Adenine, Uracil, Guanine, Cytosine, Guanine, Adenine
            ]);

            assert_eq!(Sequence::try_from(sequence).unwrap(), expected_sequence);
        }

        #[test]
        fn strict_codon_iterator() {
            use super::Nucleobase::*;

            let rna_sequence = Sequence::try_from("AAUGGCCAU").unwrap();

            let expected_codons = vec![
                &[Adenine, Adenine, Uracil],
                &[Guanine, Guanine, Cytosine],
                &[Cytosine, Adenine, Uracil],
            ];

            let codons = super::StrictCodonIterator::new(&rna_sequence).collect::<Vec<_>>();

            assert_eq!(codons, expected_codons);
        }

        #[test]
        fn strict_codon_iterator_skips_remaining_nucleobases_in_non_multiple_of_three_sequence() {
            use super::Nucleobase::*;

            let rna_sequence = Sequence::try_from("AAUGGCCAUAA").unwrap();
            //                                              ^^ skipped because there is no 3rd nucleobase to form a codon

            let expected_codons = vec![
                &[Adenine, Adenine, Uracil],
                &[Guanine, Guanine, Cytosine],
                &[Cytosine, Adenine, Uracil],
            ];

            let codons = super::StrictCodonIterator::new(&rna_sequence).collect::<Vec<_>>();

            assert_eq!(codons, expected_codons);
        }

        #[test]
        fn strict_codon_iterator_starting_from() {
            use super::Nucleobase::*;

            let rna_sequence = Sequence::try_from("UAUGGCCAU").unwrap();

            let expected_codons = vec![&[Guanine, Guanine, Cytosine], &[Cytosine, Adenine, Uracil]];

            let codons =
                super::StrictCodonIterator::new_starting_from(&rna_sequence, 3).collect::<Vec<_>>();

            assert_eq!(codons, expected_codons);
        }
    }

    use std::{convert, fmt};
    use super::Nucleobase;

    #[derive(Debug, PartialEq)]
    pub struct Sequence(Vec<Nucleobase>);

    impl Sequence {
        pub fn new(nucleobases: Vec<Nucleobase>) -> Self {
            Sequence(nucleobases)
        }
    }

    impl fmt::Display for Sequence {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for nucleobase in &self.0 {
                write!(f, "{}", nucleobase)?;
            }

            Ok(())
        }
    }

    impl<'a> From<&'a ::dna::Sequence> for Sequence {
        /// Converts a DNA sequence to a RNA sequence (basically, replaces all thymine nucleobases with uracil).
        ///
        /// # Examples
        ///
        /// ```
        /// use std::convert::TryFrom;
        ///
        /// let dna_sequence = rosalind::dna::Sequence::try_from("TTACGGGCAT").unwrap();
        /// let rna_sequence = rosalind::rna::Sequence::from(&dna_sequence);
        /// ```
        fn from(dna_sequence: &'a ::dna::Sequence) -> Self {
            Sequence(
                dna_sequence
                    .into_iter()
                    .map(|dna_nucleobase| Nucleobase::from(dna_nucleobase))
                    .collect(),
            )
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum ParseError {
        NucleobaseError {
            index: usize,
            error: super::nucleobase::ParseError,
        },
    }

    impl<'a> convert::TryFrom<&'a str> for Sequence {
        type Error = ParseError;

        /// Tries to parse a &str to a sequence of RNA nucleobases.
        ///
        /// # Examples
        ///
        /// ```
        /// use std::convert::TryFrom;
        ///
        /// match rosalind::rna::Sequence::try_from("UUACGGGCAU") {
        ///     Ok(sequence) => println!("{}", sequence),
        ///     Err(error) => println!("{:?}", error),
        /// }
        /// ```
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            let mut sequence = Vec::new();

            for (index, nucleobase_char) in value.chars().enumerate() {
                let nucleobase = match Nucleobase::try_from(nucleobase_char) {
                    Ok(nucleobase) => nucleobase,
                    Err(error) => {
                        return Err(ParseError::NucleobaseError {
                            index: index,
                            error: error,
                        });
                    }
                };

                sequence.push(nucleobase);
            }

            Ok(Sequence(sequence))
        }
    }

    /// Cloning iterator over a RNA sequence's nucleobases.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::convert::TryFrom;
    ///
    /// let sequence = rosalind::rna::Sequence::try_from("AAUUAGCCG").unwrap();
    ///
    /// for nucleobase in &sequence {
    ///     println!("{}", nucleobase);
    /// }
    /// ```
    impl<'a> IntoIterator for &'a Sequence {
        type Item = Nucleobase;
        type IntoIter = ::std::iter::Cloned<::std::slice::Iter<'a, Nucleobase>>;

        fn into_iter(self) -> Self::IntoIter {
            self.0.iter().cloned()
        }
    }

    /// Iterates over a RNA sequence, grouping nucleobases by codon (a codon is a group of 3 nucleobases).
    /// If the length of the RNA sequence is not a multiple of 3, the remaining nucleobases are skipped (hence the name _Strict_CodonIterator).
    pub struct StrictCodonIterator<'a> {
        rna: &'a [Nucleobase],
        index: usize,
    }

    impl<'a> StrictCodonIterator<'a> {
        /// # Examples
        ///
        /// ```
        /// use std::convert::TryFrom;
        /// use rosalind::rna::{Sequence, Nucleobase, sequence::StrictCodonIterator};
        ///
        /// let rna_sequence = Sequence::try_from("UUACGGGCAU").unwrap();
        ///
        /// let codons = StrictCodonIterator::new(&rna_sequence).collect::<Vec<_>>();
        /// ```
        pub fn new(rna_sequence: &'a Sequence) -> Self {
            StrictCodonIterator {
                rna: &rna_sequence.0,
                index: 0,
            }
        }

        /// Starts from a given position within the `rna_sequence`, skipping all nucleobases that come before `start_index`.
        /// It is safe to use even if `start_index` >= the length of the `rna_sequence` (in that case it will yield None).
        ///
        /// # Examples
        ///
        /// ```
        /// use std::convert::TryFrom;
        /// use rosalind::rna::{Sequence, Nucleobase, sequence::StrictCodonIterator};
        ///
        /// let rna_sequence = Sequence::try_from("UUACGGGCAU").unwrap();
        ///
        /// let codons = StrictCodonIterator::new_starting_from(&rna_sequence, 4).collect::<Vec<_>>();
        /// ```
        pub fn new_starting_from(rna_sequence: &'a Sequence, start_index: usize) -> Self {
            StrictCodonIterator {
                rna: &rna_sequence.0,
                index: start_index,
            }
        }
    }

    impl<'a> Iterator for StrictCodonIterator<'a> {
        type Item = &'a [Nucleobase];

        fn next(&mut self) -> Option<Self::Item> {
            self.index += 3;

            if self.index > self.rna.len() {
                None
            } else {
                Some(&self.rna[self.index - 3..self.index])
            }
        }
    }
}

pub mod frequent_codons {
    use super::{Nucleobase, Nucleobase::*};

    // it would be more correct to define Codon as [Nucleobase; 3] as a RNA codon is always 3-nucleobase long
    // however, defining it as [Nucleobase] allows us to use slices and avoid making copies:
    // when extracting a Codon from a RNA Sequence, we can just do &rna[i..i+3], so we avoid copies
    pub type Codon = [Nucleobase];

    pub const UUU: &Codon = &[Uracil, Uracil, Uracil];
    pub const UUC: &Codon = &[Uracil, Uracil, Cytosine];
    pub const UUA: &Codon = &[Uracil, Uracil, Adenine];
    pub const UUG: &Codon = &[Uracil, Uracil, Guanine];
    pub const CUU: &Codon = &[Cytosine, Uracil, Uracil];
    pub const CUC: &Codon = &[Cytosine, Uracil, Cytosine];
    pub const CUA: &Codon = &[Cytosine, Uracil, Adenine];
    pub const CUG: &Codon = &[Cytosine, Uracil, Guanine];
    pub const AUU: &Codon = &[Adenine, Uracil, Uracil];
    pub const AUC: &Codon = &[Adenine, Uracil, Cytosine];
    pub const AUG: &Codon = &[Adenine, Uracil, Guanine];
    pub const AUA: &Codon = &[Adenine, Uracil, Adenine];
    pub const GUU: &Codon = &[Guanine, Uracil, Uracil];
    pub const GUC: &Codon = &[Guanine, Uracil, Cytosine];
    pub const GUA: &Codon = &[Guanine, Uracil, Adenine];
    pub const GUG: &Codon = &[Guanine, Uracil, Guanine];
    pub const UCU: &Codon = &[Uracil, Cytosine, Uracil];
    pub const UCC: &Codon = &[Uracil, Cytosine, Cytosine];
    pub const UCA: &Codon = &[Uracil, Cytosine, Adenine];
    pub const UCG: &Codon = &[Uracil, Cytosine, Guanine];
    pub const CCU: &Codon = &[Cytosine, Cytosine, Uracil];
    pub const CCC: &Codon = &[Cytosine, Cytosine, Cytosine];
    pub const CCA: &Codon = &[Cytosine, Cytosine, Adenine];
    pub const CCG: &Codon = &[Cytosine, Cytosine, Guanine];
    pub const ACU: &Codon = &[Adenine, Cytosine, Uracil];
    pub const ACC: &Codon = &[Adenine, Cytosine, Cytosine];
    pub const ACA: &Codon = &[Adenine, Cytosine, Adenine];
    pub const ACG: &Codon = &[Adenine, Cytosine, Guanine];
    pub const GCU: &Codon = &[Guanine, Cytosine, Uracil];
    pub const GCC: &Codon = &[Guanine, Cytosine, Cytosine];
    pub const GCA: &Codon = &[Guanine, Cytosine, Adenine];
    pub const GCG: &Codon = &[Guanine, Cytosine, Guanine];
    pub const UAU: &Codon = &[Uracil, Adenine, Uracil];
    pub const UAC: &Codon = &[Uracil, Adenine, Cytosine];
    pub const CAU: &Codon = &[Cytosine, Adenine, Uracil];
    pub const CAC: &Codon = &[Cytosine, Adenine, Cytosine];
    pub const CAA: &Codon = &[Cytosine, Adenine, Adenine];
    pub const CAG: &Codon = &[Cytosine, Adenine, Guanine];
    pub const AAU: &Codon = &[Adenine, Adenine, Uracil];
    pub const AAC: &Codon = &[Adenine, Adenine, Cytosine];
    pub const AAA: &Codon = &[Adenine, Adenine, Adenine];
    pub const AAG: &Codon = &[Adenine, Adenine, Guanine];
    pub const GAU: &Codon = &[Guanine, Adenine, Uracil];
    pub const GAC: &Codon = &[Guanine, Adenine, Cytosine];
    pub const GAA: &Codon = &[Guanine, Adenine, Adenine];
    pub const GAG: &Codon = &[Guanine, Adenine, Guanine];
    pub const UGU: &Codon = &[Uracil, Guanine, Uracil];
    pub const UGC: &Codon = &[Uracil, Guanine, Cytosine];
    pub const UGG: &Codon = &[Uracil, Guanine, Guanine];
    pub const CGU: &Codon = &[Cytosine, Guanine, Uracil];
    pub const CGC: &Codon = &[Cytosine, Guanine, Cytosine];
    pub const CGA: &Codon = &[Cytosine, Guanine, Adenine];
    pub const CGG: &Codon = &[Cytosine, Guanine, Guanine];
    pub const AGU: &Codon = &[Adenine, Guanine, Uracil];
    pub const AGC: &Codon = &[Adenine, Guanine, Cytosine];
    pub const AGA: &Codon = &[Adenine, Guanine, Adenine];
    pub const AGG: &Codon = &[Adenine, Guanine, Guanine];
    pub const GGU: &Codon = &[Guanine, Guanine, Uracil];
    pub const GGC: &Codon = &[Guanine, Guanine, Cytosine];
    pub const GGA: &Codon = &[Guanine, Guanine, Adenine];
    pub const GGG: &Codon = &[Guanine, Guanine, Guanine];
    pub const UAA: &Codon = &[Uracil, Adenine, Adenine];
    pub const UAG: &Codon = &[Uracil, Adenine, Guanine];
    pub const UGA: &Codon = &[Uracil, Guanine, Adenine];
}
