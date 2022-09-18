// Volvelle Website
// Written in 2022 by
//     Andrew Poelstra <apoelstra@wpsoftware.net>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! Field Arithmetic
//!
//! Functionality to compute the codex32 error-correcting code, do field arithmetic, etc
//!

use serde::{Deserialize, Serialize};
use std::{iter, ops};

/// Needed for indexing as we need a static-lifetime zero object
const ZERO: Fe = Fe(0);
/// The bech32 alphabet, in binary order
const BECH32_ALPHABET: &[u8] = b"QPZRY9X8GF2TVDW0S3JNS4KHCE6MUA7L";
/// The codex32 generator polynomial
const CODEX32_POLYMOD: &[Fe] = &[
    Fe(25),
    Fe(27),
    Fe(17),
    Fe(8),
    Fe(0),
    Fe(25),
    Fe(25),
    Fe(25),
    Fe(31),
    Fe(27),
    Fe(24),
    Fe(16),
    Fe(16),
];
/// The bech32 generator polynomial
const BECH32_POLYMOD: &[Fe] = &[Fe(29), Fe(22), Fe(20), Fe(21), Fe(29), Fe(18)];

/// A single field element in the bech32 field
#[derive(Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct Fe(u8);

impl Fe {
    /// Construct the additive identity of the field
    pub fn zero() -> Self {
        Fe(0)
    }
    /// Construct the multiplicative identity of the field
    pub fn one() -> Self {
        Fe(1)
    }

    /// Construct a field element from its binary expression
    pub fn from_bin(n: u8) -> Self {
        Fe(n)
    }
}

impl From<Fe> for char {
    fn from(fe: Fe) -> Self {
        BECH32_ALPHABET[fe.0 as usize].into()
    }
}

impl TryFrom<char> for Fe {
    type Error = String;

    fn try_from(ch: char) -> Result<Self, String> {
        match ch {
            'Q' => Ok(Fe(0x00)),
            'P' => Ok(Fe(0x01)),
            'Z' => Ok(Fe(0x02)),
            'R' => Ok(Fe(0x03)),
            'Y' => Ok(Fe(0x04)),
            '9' => Ok(Fe(0x05)),
            'X' => Ok(Fe(0x06)),
            '8' => Ok(Fe(0x07)),
            'G' => Ok(Fe(0x08)),
            'F' => Ok(Fe(0x09)),
            '2' => Ok(Fe(0x0a)),
            'T' => Ok(Fe(0x0b)),
            'V' => Ok(Fe(0x0c)),
            'D' => Ok(Fe(0x0d)),
            'W' => Ok(Fe(0x0e)),
            '0' => Ok(Fe(0x0f)),
            'S' => Ok(Fe(0x10)),
            '3' => Ok(Fe(0x11)),
            'J' => Ok(Fe(0x12)),
            'N' => Ok(Fe(0x13)),
            '5' => Ok(Fe(0x14)),
            '4' => Ok(Fe(0x15)),
            'K' => Ok(Fe(0x16)),
            'H' => Ok(Fe(0x17)),
            'C' => Ok(Fe(0x18)),
            'E' => Ok(Fe(0x19)),
            '6' => Ok(Fe(0x1a)),
            'M' => Ok(Fe(0x1b)),
            'U' => Ok(Fe(0x1c)),
            'A' => Ok(Fe(0x1d)),
            '7' => Ok(Fe(0x1e)),
            'L' => Ok(Fe(0x1f)),
            x => Err(format!("invalid bech32 character {}", x)),
        }
    }
}

impl ops::Add<Fe> for Fe {
    type Output = Fe;
    fn add(self, other: Fe) -> Fe {
        Fe(self.0 ^ other.0)
    }
}
impl ops::Add<&Fe> for Fe {
    type Output = Fe;
    fn add(self, other: &Fe) -> Fe {
        Fe(self.0 ^ other.0)
    }
}

impl ops::Mul<Fe> for Fe {
    type Output = Fe;
    fn mul(self, other: Fe) -> Fe {
        self * &other
    }
}
impl ops::Mul<&Fe> for Fe {
    type Output = Fe;
    fn mul(mut self, other: &Fe) -> Fe {
        let mut ret = 0;
        let mut fe2 = other.0;
        while self.0 > 0 {
            if self.0 & 1 == 1 {
                ret ^= fe2;
            }

            self.0 >>= 1;
            fe2 <<= 1;

            if fe2 & 32 == 32 {
                fe2 ^= 32 + 8 + 1;
            }
        }
        Fe(ret)
    }
}

/// A polynomial in the bech32 field
#[derive(Clone, PartialEq, Eq, Debug, Default, Deserialize, Serialize)]
pub struct FePoly(Vec<Fe>);

impl From<Fe> for FePoly {
    fn from(fe: Fe) -> Self {
        FePoly(vec![fe])
    }
}

impl ops::Index<usize> for FePoly {
    type Output = Fe;
    fn index(&self, idx: usize) -> &Fe {
        self.0.get(idx).unwrap_or(&ZERO)
    }
}

impl FePoly {
    /// Helper function that drops any leading 0s from the polynomial
    fn normalize(&mut self) {
        let mut seen_nonzero = false;
        self.0.retain(|&elem| {
            if elem != Fe(0) {
                seen_nonzero = true;
            }
            seen_nonzero
        });
    }

    /// Reduce a polynomial modulo the codex32 generator polynomial
    fn polymod(&self, modulus: &[Fe]) -> Self {
        let mut ret = vec![Fe(0); 13];

        for ch in &self.0 {
            // Multiply residue by x
            let c13 = ret[0];
            for i in 0..modulus.len() - 1 {
                ret[i] = ret[i + 1];
            }
            // Add next character
            ret[12] = *ch;
            // Replace A*x^13 by A*polymod
            for i in 0..modulus.len() {
                ret[i] = ret[i] + c13 * modulus[i];
            }
        }

        let mut ret = FePoly(ret);
        ret.normalize();
        ret
    }

    /// Reduce a polynomial modulo the codex32 generator polynomial
    pub fn codex32_polymod(&self) -> Self {
        self.polymod(&CODEX32_POLYMOD)
    }

    /// Reduce a polynomial modulo the bech32 generator polynomial
    pub fn bech32_polymod(&self) -> Self {
        self.polymod(&BECH32_POLYMOD)
    }

    /// Convert a HRP into a polynomial
    fn hrp_residue(s: &str, modulus: &[Fe]) -> Self {
        let mut poly_1 = Vec::with_capacity(s.len() * 2 + modulus.len() + 2);
        poly_1.push(Fe(1));
        for ch in s.bytes() {
            poly_1.push(Fe(ch >> 5));
        }
        poly_1.push(Fe(0));
        for ch in s.bytes() {
            poly_1.push(Fe(ch & 0x1f));
        }
        poly_1.extend(iter::repeat(Fe(0)).take(modulus.len()));
        FePoly(poly_1).polymod(modulus)
    }

    /// Convert a HRP into a polynomial residue (codex32)
    pub fn codex32_hrp_residue(s: &str) -> Self {
        FePoly::hrp_residue(s, &CODEX32_POLYMOD)
    }

    /// Convert a HRP into a polynomial residue (bech32)
    pub fn bech32_hrp_residue(s: &str) -> Self {
        FePoly::hrp_residue(s, &BECH32_POLYMOD)
    }

    /// Return an iterator over the coefficients of the polynomial
    pub fn iter(&self) -> impl Iterator<Item = Fe> + '_ {
        self.0.iter().copied()
    }
}
