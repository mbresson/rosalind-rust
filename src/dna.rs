pub use self::nucleobase::Nucleobase;
pub use self::sequence::Sequence;

pub mod nucleobase {
    #[cfg(test)]
    mod tests {
        #[test]
        fn try_from_erroneous_char() {
            use std::convert::TryFrom;

            assert_eq!(
                super::Nucleobase::try_from('錯').unwrap_err(),
                super::ParseError::IllegalChar { ch: '錯' },
            );
        }

        #[test]
        fn try_from_char() {
            use std::convert::TryFrom;
            use super::Nucleobase::*;

            assert_eq!(super::Nucleobase::try_from('A').unwrap(), Adenine);
            assert_eq!(super::Nucleobase::try_from('T').unwrap(), Thymine);
            assert_eq!(super::Nucleobase::try_from('C').unwrap(), Cytosine);
            assert_eq!(super::Nucleobase::try_from('G').unwrap(), Guanine);
        }

        #[test]
        fn complement() {
            use super::Nucleobase::*;
            assert_eq!(Adenine.complement(), Thymine);
            assert_eq!(Thymine.complement(), Adenine);
            assert_eq!(Cytosine.complement(), Guanine);
            assert_eq!(Guanine.complement(), Cytosine);
        }
    }

    use std::{convert, error, fmt};

    #[derive(Clone, Debug, PartialEq)]
    pub enum Nucleobase {
        Adenine,
        Thymine,
        Cytosine,
        Guanine,
    }

    impl Nucleobase {
        /// Returns the complement of the nucleobase: A <=> T and C <=> G.
        ///
        /// # Examples
        ///
        /// ```
        ///
        /// let nucleobase = rosalind::dna::Nucleobase::Adenine;
        ///
        /// let complement = nucleobase.complement();
        /// ```
        pub fn complement(&self) -> Self {
            use self::Nucleobase::*;

            match self {
                Adenine => Thymine,
                Thymine => Adenine,
                Cytosine => Guanine,
                Guanine => Cytosine,
            }
        }
    }

    impl fmt::Display for Nucleobase {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use self::Nucleobase::*;

            match self {
                Adenine => write!(f, "A"),
                Thymine => write!(f, "T"),
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
                    "there is no such nucleobase as represented by this character"
                }
            }
        }
    }

    impl fmt::Display for ParseError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ParseError::IllegalChar { ch } => write!(
                    f,
                    "there is no such DNA nucleobase as represented by character {}",
                    ch
                ),
            }
        }
    }

    impl convert::TryFrom<char> for Nucleobase {
        type Error = ParseError;

        /// Tries to parse a single char to its corresponding DNA nucleobase.
        ///
        /// # Examples
        ///
        /// ```
        /// use std::convert::TryFrom;
        ///
        /// match rosalind::dna::Nucleobase::try_from('C') {
        ///     Ok(nucleobase) => println!("{}", nucleobase),
        ///     Err(error) => println!("{:?}", error),
        /// }
        /// ```
        fn try_from(ch: char) -> Result<Self, Self::Error> {
            use self::Nucleobase::{Adenine, Cytosine, Guanine, Thymine};

            let nucleobase = match ch {
                'A' => Adenine,
                'T' => Thymine,
                'C' => Cytosine,
                'G' => Guanine,
                _ => {
                    return Err(ParseError::IllegalChar { ch: ch });
                }
            };

            Ok(nucleobase)
        }
    }
}

pub mod sequence {
    #[cfg(test)]
    mod tests {
        use super::Sequence;
        use super::super::{nucleobase, Nucleobase};
        use std::convert::TryFrom;

        #[test]
        fn try_from_erroneous_str() {
            assert_eq!(
                Sequence::try_from("ATECCG").unwrap_err(),
                super::ParseError::NucleobaseError {
                    index: 2,
                    error: nucleobase::ParseError::IllegalChar { ch: 'E' },
                },
            );
        }

        #[test]
        fn try_from_str() {
            use self::Nucleobase::*;

            let sequence = "AATGCGA";

            let expected_sequence = Sequence(vec![
                Adenine, Adenine, Thymine, Guanine, Cytosine, Guanine, Adenine
            ]);

            assert_eq!(Sequence::try_from(sequence).unwrap(), expected_sequence);
        }

        #[test]
        fn count_nucleobases() {
            let sequence = Sequence::try_from("AATAGGCTA").unwrap();

            let expected_count = super::NucleobaseCount {
                adenines: 4,
                thymines: 2,
                cytosines: 1,
                guanines: 2,
            };

            assert_eq!(sequence.count_nucleobases(), expected_count);
        }

        #[test]
        fn reverse_complement() {
            let sequence = Sequence::try_from("AATAGGCTA").unwrap();

            let expected_reverse_complement = Sequence::try_from("TAGCCTATT").unwrap();

            assert_eq!(sequence.reverse_complement(), expected_reverse_complement);
        }
    }

    use std::{convert, fmt, ops};
    use super::Nucleobase;

    #[derive(Debug, PartialEq)]
    pub struct Sequence(Vec<Nucleobase>);

    // as of 2018, the largest genome ever sequenced is the Axolotl genome: approx. 32 billion base pairs
    // if we ever happen to count the nucleobases of such a big genome,
    // the u64 type should be more than enough to hold values without risk of overflowing
    #[derive(Debug, PartialEq)]
    pub struct NucleobaseCount {
        pub adenines: u64,
        pub thymines: u64,
        pub cytosines: u64,
        pub guanines: u64,
    }

    impl Sequence {
        pub fn len(&self) -> usize {
            self.0.len()
        }

        /// Returns the reverse complement of the DNA sequence.
        ///
        /// # Examples
        ///
        /// ```
        /// use std::convert::TryFrom;
        ///
        /// let sequence = rosalind::dna::Sequence::try_from("AATAGGCTA").expect("Couldn't parse sequence");
        /// println!("{}", sequence.reverse_complement());
        /// ```
        pub fn reverse_complement(&self) -> Self {
            Sequence(
                self.0
                    .iter()
                    .rev()
                    .map(|nucleobase| nucleobase.complement())
                    .collect(),
            )
        }

        /// Returns the number of each A, T, C, G nucleobase in the `dna` sequence.
        ///
        /// # Examples
        ///
        /// ```
        /// use std::convert::TryFrom;
        ///
        /// let sequence = rosalind::dna::Sequence::try_from("AATAGGCTA").expect("Couldn't parse sequence");
        /// let count = sequence.count_nucleobases();
        ///
        /// println!(
        ///     "Adenine: {}\nThymine: {}\nCytosine: {}\nGuanine: {}",
        ///     count.adenines, count.thymines, count.cytosines, count.guanines
        /// );
        /// ```
        pub fn count_nucleobases(&self) -> NucleobaseCount {
            use self::Nucleobase::*;

            let mut count = NucleobaseCount {
                adenines: 0,
                thymines: 0,
                cytosines: 0,
                guanines: 0,
            };

            for nucleobase in self.0.iter() {
                match *nucleobase {
                    Adenine => count.adenines += 1,
                    Thymine => count.thymines += 1,
                    Cytosine => count.cytosines += 1,
                    Guanine => count.guanines += 1,
                }
            }

            count
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

    /// ```
    /// use std::convert::TryFrom;
    ///
    /// let sequence = rosalind::dna::Sequence::try_from("TTACGGGCAT").unwrap();
    ///
    /// let subsequence = &sequence[0..3];
    /// println!("{:?}", subsequence);
    /// ```
    impl ops::Index<ops::Range<usize>> for Sequence {
        type Output = [Nucleobase];

        #[inline]
        fn index(&self, index: ops::Range<usize>) -> &[Nucleobase] {
            &self.0[index]
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

        /// Tries to parse a &str to a sequence of DNA nucleobases.
        ///
        /// # Examples
        ///
        /// ```
        /// use std::convert::TryFrom;
        ///
        /// match rosalind::dna::Sequence::try_from("TTACGGGCAT") {
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

    /// Cloning iterator over a DNA sequence's nucleobases.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::convert::TryFrom;
    ///
    /// let sequence = rosalind::dna::Sequence::try_from("AATTAGCCG").unwrap();
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
}
