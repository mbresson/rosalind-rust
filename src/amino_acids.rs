use std::fmt;

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
