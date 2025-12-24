//! Cryptographic protocols and algorithms.
//!
//! This module provides implementations of fundamental cryptographic protocols:
//! - `gcd` - Extended Euclidean Algorithm with modular inverse
//! - `crt` - Chinese Remainder Theorem solver

pub mod crt;
pub mod gcd;

pub use crt::{CrtError, chinese_remainder, chinese_remainder_solver};
pub use gcd::{ExtendedGcdResult, extended_gcd, mod_inverse};
