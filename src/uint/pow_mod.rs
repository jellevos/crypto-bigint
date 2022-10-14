// use crate::{Concat, Split, UInt};

// use super::modular::{Residue, ResidueParams};

// impl<const LIMBS: usize, const DLIMBS: usize> UInt<LIMBS>
// where
//     UInt<LIMBS>: Concat<Output = UInt<DLIMBS>>,
//     UInt<DLIMBS>: Split<Output = UInt<LIMBS>>,
// {
//     /// Computes `self^exponent mod modulus` using Montgomery's ladder. If you are also performing other modular operations, consider using `Modular` and the associated `pow` function.
//     pub fn pow_mod(&self, exponent: &Self, modulus: &Self) -> Self {
//         let modulus_params = ResidueParams::new(*modulus);
//         let base_mod = Residue::new(*self, modulus_params);

//         base_mod.pow(exponent).retrieve()
//     }

//     /// Computes `self^exponent mod modulus` using Montgomery's ladder, but only considering the first `exponent_bits` bits of the exponent. This number is revealed from the timing pattern. If you are also performing other modular operations, consider using `Modular` and the associated `pow` function.
//     pub fn pow_mod_specific(&self, exponent: &Self, modulus: &Self, exponent_bits: usize) -> Self {
//         let modulus_params = ResidueParams::new(*modulus);
//         let base_mod = Residue::new(*self, modulus_params);

//         base_mod.pow_specific(exponent, exponent_bits).retrieve()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::U64;

//     #[test]
//     fn test_powmod_mini() {
//         let b = U64::from(3u64);
//         let e = U64::from(7u64);
//         let m = U64::from(11u64);

//         let res = b.pow_mod(&e, &m);

//         let expected = U64::from(9u64);
//         assert_eq!(res, expected);
//     }
// }
