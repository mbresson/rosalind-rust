use std::{convert, error, fmt};

pub use self::sequence::Sequence;

use rna::Sequence as RnaSequence;

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone, Ord)]
pub enum AminoAcid {
    Alanine,
    Arginine,
    Asparagine,
    AsparticAcid,
    Cysteine,
    GlutamicAcid,
    Glutamine,
    Glycine,
    Histidine,
    Isoleucine,
    Leucine,
    Lysine,
    Methionine,
    Phenyalalanine,
    Proline,
    Serine,
    Threonine,
    Tryptophan,
    Tyrosine,
    Valine,
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    IllegalChar { ch: char },
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        match self {
            ParseError::IllegalChar { .. } => {
                "there is no such amino acid as represented by this character"
            }
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::IllegalChar { ch } => write!(
                f,
                "there is no such amino acid as represented by character {}",
                ch
            ),
        }
    }
}

impl convert::TryFrom<char> for AminoAcid {
    type Error = ParseError;

    /// Tries to parse a single char to its corresponding amino acid.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::convert::TryFrom;
    ///
    /// match rosalind::amino_acids::AminoAcid::try_from('S') {
    ///     Ok(nucleobase) => println!("{}", nucleobase),
    ///     Err(error) => println!("{:?}", error),
    /// }
    /// ```
    fn try_from(ch: char) -> Result<Self, Self::Error> {
        use self::AminoAcid::*;

        match ch {
            'F' => Ok(Phenyalalanine),
            'L' => Ok(Leucine),
            'I' => Ok(Isoleucine),
            'M' => Ok(Methionine),
            'V' => Ok(Valine),
            'S' => Ok(Serine),
            'P' => Ok(Proline),
            'T' => Ok(Threonine),
            'A' => Ok(Alanine),
            'Y' => Ok(Tyrosine),
            'H' => Ok(Histidine),
            'Q' => Ok(Glutamine),
            'N' => Ok(Asparagine),
            'K' => Ok(Lysine),
            'D' => Ok(AsparticAcid),
            'E' => Ok(GlutamicAcid),
            'C' => Ok(Cysteine),
            'W' => Ok(Tryptophan),
            'R' => Ok(Arginine),
            'G' => Ok(Glycine),
            _ => Err(ParseError::IllegalChar { ch }),
        }
    }
}

impl fmt::Display for AminoAcid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::AminoAcid::*;

        let repr = match *self {
            Alanine => "A",
            Arginine => "R",
            Asparagine => "N",
            AsparticAcid => "D",
            Cysteine => "C",
            GlutamicAcid => "E",
            Glutamine => "Q",
            Glycine => "G",
            Histidine => "H",
            Isoleucine => "I",
            Leucine => "L",
            Lysine => "K",
            Methionine => "M",
            Phenyalalanine => "F",
            Proline => "P",
            Serine => "S",
            Threonine => "T",
            Tryptophan => "W",
            Tyrosine => "Y",
            Valine => "V",
        };

        write!(f, "{}", repr)
    }
}

pub mod sequence {

    #[cfg(test)]
    mod tests {

        #[test]
        fn amino_acids_from_rna_sequence() {
            use std::convert::TryFrom;

            let rna = ::rna::Sequence::try_from(
                "AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGGUGA",
            ).unwrap();

            let aas = super::Sequence::from(&rna);

            assert_eq!(aas.to_string(), "MAMAPRTEINSTRING");
        }
    }

    use std::{convert, fmt, ops};
    use super::AminoAcid;
    use super::RnaSequence;

    #[derive(Debug, Eq, PartialOrd, Ord, PartialEq)]
    pub struct Sequence(pub Vec<AminoAcid>);

    impl Sequence {
        pub fn len(&self) -> usize {
            self.0.len()
        }
    }

    // to be able to easily display a Vec<AminoAcid>
    impl fmt::Display for Sequence {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for amino_acid in &self.0 {
                write!(f, "{}", amino_acid)?;
            }

            Ok(())
        }
    }

    impl ops::Index<usize> for Sequence {
        type Output = AminoAcid;

        fn index(&self, index: usize) -> &AminoAcid {
            &self.0[index]
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum ParseError {
        AminoAcidError {
            index: usize,
            error: super::ParseError,
        },
    }

    impl<'a> convert::TryFrom<&'a str> for Sequence {
        type Error = ParseError;

        /// Tries to parse a &str to a sequence of amino acids.
        ///
        /// # Examples
        ///
        /// ```
        /// use std::convert::TryFrom;
        ///
        /// let aas = rosalind::amino_acids::Sequence::try_from("MTPRLGLESLLE").unwrap();
        /// ```
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            let mut sequence = Vec::new();

            for (index, aa_char) in value.chars().enumerate() {
                let aa = match AminoAcid::try_from(aa_char) {
                    Ok(aa) => aa,
                    Err(error) => {
                        return Err(ParseError::AminoAcidError {
                            index: index,
                            error: error,
                        });
                    }
                };

                sequence.push(aa);
            }

            Ok(Sequence(sequence))
        }
    }

    impl<'a> convert::From<&'a RnaSequence> for Sequence {
        /// Converts a RNA sequence to an amino acid sequence.
        ///
        /// # Examples
        ///
        /// ```
        /// use std::convert::TryFrom;
        ///
        /// let rna_sequence = rosalind::rna::Sequence::try_from("UUAAGCGAU").unwrap();
        /// let amino_acids = rosalind::amino_acids::Sequence::from(&rna_sequence);
        ///
        /// println!("{}", amino_acids);
        /// ```
        fn from(rna: &RnaSequence) -> Self {
            use self::AminoAcid::*;
            use rna::codons::*;

            let mut amino_acids = Vec::new();

            for codon in ::rna::sequence::StrictCodonIterator::new(rna) {
                let amino_acid = match codon {
                    UUU | UUC => Phenyalalanine,
                    UUA | UUG | CUU | CUC | CUA | CUG => Leucine,
                    AUU | AUC | AUA => Isoleucine,
                    AUG => Methionine,
                    GUU | GUC | GUA | GUG => Valine,
                    UCU | UCC | UCA | UCG => Serine,
                    CCU | CCC | CCA | CCG => Proline,
                    ACU | ACC | ACA | ACG => Threonine,
                    GCU | GCC | GCA | GCG => Alanine,
                    UAU | UAC => Tyrosine,
                    CAU | CAC => Histidine,
                    CAA | CAG => Glutamine,
                    AAU | AAC => Asparagine,
                    AAA | AAG => Lysine,
                    GAU | GAC => AsparticAcid,
                    GAA | GAG => GlutamicAcid,
                    UGU | UGC => Cysteine,
                    UGG => Tryptophan,
                    CGU | CGC | CGA | CGG => Arginine,
                    AGU | AGC => Serine,
                    AGA | AGG => Arginine,
                    GGU | GGC | GGA | GGG => Glycine,
                    UAA | UAG | UGA => {
                        // STOP codons, they don't code for any amino acid
                        continue;
                    }
                    _ => {
                        // this can never happen as we cover every possible case
                        panic!("No matching codon! {:?}", codon);
                    }
                };

                amino_acids.push(amino_acid);
            }

            Sequence(amino_acids)
        }
    }

    /// Cloning iterator over a sequence's amino acids.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::convert::TryFrom;
    ///
    /// let sequence = rosalind::amino_acids::Sequence::try_from("MTPRLGLESLLE").unwrap();
    ///
    /// for aa in &sequence {
    ///     println!("{}", aa);
    /// }
    /// ```
    impl<'a> IntoIterator for &'a Sequence {
        type Item = AminoAcid;
        type IntoIter = ::std::iter::Cloned<::std::slice::Iter<'a, AminoAcid>>;

        fn into_iter(self) -> Self::IntoIter {
            self.0.iter().cloned()
        }
    }

}
