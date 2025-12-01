use std::fmt;
use std::ops::{Add, BitXor, Mul, Sub};

use crate::big_int::backend::gmp;
use crate::traits::BigInt;

// U1024 = 16 x 64-bit limbs
const LIMBS: usize = 16;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct U1024(pub [u64; LIMBS]);

impl U1024 {
    #[inline(always)]
    fn gmp_add(&self, rhs: &Self) -> (Self, bool) {
        let mut result = U1024([0; LIMBS]);
        unsafe {
            let carry = gmp::__gmpn_add_n(
                result.0.as_mut_ptr(),
                self.0.as_ptr(),
                rhs.0.as_ptr(),
                LIMBS as u64,
            );
            (result, carry != 0)
        }
    }

    /// Wrapper an toàn cho GMP Sub
    #[inline(always)]
    fn gmp_sub(&self, rhs: &Self) -> (Self, bool) {
        let mut result = U1024([0; LIMBS]);
        unsafe {
            let borrow = gmp::__gmpn_sub_n(
                result.0.as_mut_ptr(),
                self.0.as_ptr(),
                rhs.0.as_ptr(),
                LIMBS as u64,
            );
            (result, borrow != 0)
        }
    }
}

// --- Implement Trait BigInt ---
impl BigInt for U1024 {
    const NUM_LIMBS: usize = LIMBS;

    fn zero() -> Self {
        U1024([0; LIMBS])
    }

    fn one() -> Self {
        let mut v = [0; LIMBS];
        v[0] = 1;
        U1024(v)
    }

    fn from_u64(v: u64) -> Self {
        let mut arr = [0; LIMBS];
        arr[0] = v;
        U1024(arr)
    }

    fn carrying_add(&self, rhs: &Self) -> (Self, bool) {
        // Nếu feature "gmp" được bật (mặc định), dùng GMP
        #[cfg(feature = "gmp")]
        return self.gmp_add(rhs);

        // Nếu không (fallback), dùng code Rust thuần (sẽ làm vào ngày 4)
        #[cfg(not(feature = "gmp"))]
        unimplemented!("Native rust backend not implemented yet");
    }

    fn borrowing_sub(&self, rhs: &Self) -> (Self, bool) {
        #[cfg(feature = "gmp")]
        return self.gmp_sub(rhs);

        #[cfg(not(feature = "gmp"))]
        unimplemented!("Native rust backend not implemented yet");
    }

    fn conditional_select(a: &Self, b: &Self, choice: bool) -> Self {
        let mut res = U1024([0; LIMBS]);
        let mask = if choice { u64::MAX } else { 0 };
        for i in 0..LIMBS {
            res.0[i] = (a.0[i] & mask) | (b.0[i] & !mask);
        }
        res
    }
}

// --- Operator Overloading ---
impl Add for U1024 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        self.carrying_add(&rhs).0
    }
}

impl Sub for U1024 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self.borrowing_sub(&rhs).0
    }
}

// Placeholder cho Mul (sẽ làm vào Ngày 2)
impl Mul for U1024 {
    type Output = Self;
    fn mul(self, _rhs: Self) -> Self {
        unimplemented!("Mul implemented in Day 2")
    }
}

// Placeholder cho BitXor
impl BitXor for U1024 {
    type Output = Self;
    fn bitxor(self, _rhs: Self) -> Self {
        unimplemented!("BitXor implemented in Day 4")
    }
}

// Hiển thị Hex để debug
impl Default for U1024 {
    fn default() -> Self {
        Self::zero()
    }
}
impl fmt::Debug for U1024 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x")?;
        for limb in self.0.iter().rev() {
            write!(f, "{:016x}", limb)?;
        }
        Ok(())
    }
}
impl fmt::Display for U1024 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
