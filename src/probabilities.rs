// probabilities that two mating organisms will produce an individual possessing a dominant allele,
// as illustrated with Punnett squares where Y = a dominant allele and y = a recissive allele

// Punett square for two 'k' (homozygous dominant individuals, YY and YY)
//    | Y  | Y
// ---|----|----
// Y  | YY | YY
// ---|----|----
// Y  | YY | YY
//
// probability of producing a dominant allele: 1
pub const PROBABILITY_K_AND_K_PRODUCING_DOMINANT_ALLELE: f64 = 1.0;

// Punett square for 'k' (homozygous dominant, YY) and 'm' (heterozygous, Yy)
//    | Y  | y
// ---|----|----
// Y  | YY | Yy
// ---|----|----
// Y  | YY | Yy
//
// probability of producing a dominant allele: 1 (at least we'll have 1 Y)
pub const PROBABILITY_K_AND_M_PRODUCING_DOMINANT_ALLELE: f64 = 1.0;

// Punett square for 'k' (homozygous dominant, YY) and 'n' (homozygous recessive, yy)
//    | y  | y
// ---|----|----
// Y  | Yy | Yy
// ---|----|----
// Y  | Yy | Yy
//
// probability of producing a dominant allele: 1 (at least we'll have 1 Y)
pub const PROBABILITY_K_AND_N_PRODUCING_DOMINANT_ALLELE: f64 = 1.0;

// Punett square for two 'm' (heterozygous, Yy)
//    | Y  | y
// ---|----|----
// Y  | YY | Yy
// ---|----|----
// y  | yY | yy
//
// probability of producing a dominant allele: 0.75 (3 of 4 cases produce a dominant allele)
pub const PROBABILITY_M_AND_M_PRODUCING_DOMINANT_ALLELE: f64 = 0.75;

// Punett square for 'm' (heterozygous, Yy) and 'n' (homozygous recessive, yy)
//    | y  | y
// ---|----|----
// Y  | Yy | Yy
// ---|----|----
// y  | yy | yy
//
// probability of producing a dominant allele: 0.5 (2 of 4 cases produce a dominant allele)
pub const PROBABILITY_M_AND_N_PRODUCING_DOMINANT_ALLELE: f64 = 0.5;

// Punett square for two 'n' (homozygous recessive, yy)
//    | y  | y
// ---|----|----
// y  | yy | yy
// ---|----|----
// y  | yy | yy
//
// probability of producing a dominant allele: 0 (there is no dominant allele from the start)
pub const PROBABILITY_N_AND_N_PRODUCING_DOMINANT_ALLELE: f64 = 0.0;
