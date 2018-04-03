use std::fmt;
use std::error;

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

#[derive(Debug, Clone)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid sequence")
    }
}

// quick & dirty impl for the moment
impl error::Error for ParseError {
    fn description(&self) -> &str {
        "invalid sequence"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl AminoAcid {
    // TODO when std::convert::TryFrom trait moves into stable Rust, use it (https://doc.rust-lang.org/std/convert/trait.TryFrom.html)
    pub fn from_char(aa_char: char) -> Result<AminoAcid, ParseError> {
        use self::AminoAcid::*;

        match aa_char {
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
            _ => Err(ParseError),
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

pub fn amino_acids_from_rna(rna: &str) -> Result<Vec<AminoAcid>, String> {
    use self::AminoAcid::*;

    let mut amino_acids = Vec::new();

    for codon in super::CodonIterator::new(rna) {
        let amino_acid = match codon {
            "UUU" | "UUC" => Phenyalalanine,
            "UUA" | "UUG" | "CUU" | "CUC" | "CUA" | "CUG" => Leucine,
            "AUU" | "AUC" | "AUA" => Isoleucine,
            "AUG" => Methionine,
            "GUU" | "GUC" | "GUA" | "GUG" => Valine,
            "UCU" | "UCC" | "UCA" | "UCG" => Serine,
            "CCU" | "CCC" | "CCA" | "CCG" => Proline,
            "ACU" | "ACC" | "ACA" | "ACG" => Threonine,
            "GCU" | "GCC" | "GCA" | "GCG" => Alanine,
            "UAU" | "UAC" => Tyrosine,
            "CAU" | "CAC" => Histidine,
            "CAA" | "CAG" => Glutamine,
            "AAU" | "AAC" => Asparagine,
            "AAA" | "AAG" => Lysine,
            "GAU" | "GAC" => AsparticAcid,
            "GAA" | "GAG" => GlutamicAcid,
            "UGU" | "UGC" => Cysteine,
            "UGG" => Tryptophan,
            "CGU" | "CGC" | "CGA" | "CGG" => Arginine,
            "AGU" | "AGC" => Serine,
            "AGA" | "AGG" => Arginine,
            "GGU" | "GGC" | "GGA" | "GGG" => Glycine,
            "UAA" | "UAG" | "UGA" => {
                // STOP codons, they don't code for any amino acid
                continue;
            }
            _ => {
                return Err(format!("No amino acid matching codon {}!", codon));
            }
        };

        amino_acids.push(amino_acid);
    }

    Ok(amino_acids)
}

pub struct AminoAcidString<'a>(pub &'a Vec<AminoAcid>);

// to be able to easily display a Vec<AminoAcid>
impl<'a> fmt::Display for AminoAcidString<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for amino_acid in self.0 {
            try!(write!(f, "{}", amino_acid));
        }

        Ok(())
    }
}
