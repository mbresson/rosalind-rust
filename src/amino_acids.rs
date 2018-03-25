use std::fmt;
use std::error;

#[derive(PartialEq)]
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
