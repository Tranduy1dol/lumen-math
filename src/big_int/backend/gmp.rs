use libc::{c_int, c_long};

#[cfg(target_pointer_width = "64")]
pub type Limb = u64;

#[cfg(not(target_pointer_width = "64"))]
compile_error!("GMP backend currently only supports 64-bit targets");

#[cfg(all(feature = "gmp", target_pointer_width = "64"))]
#[link(name = "gmp")]
unsafe extern "C" {
    pub fn __gmpn_add_n(rp: *mut Limb, s1p: *const Limb, s2p: *const Limb, n: c_long) -> Limb;

    pub fn __gmpn_sub_n(rp: *mut Limb, s1p: *const Limb, s2p: *const Limb, n: c_long) -> Limb;

    pub fn __gmpn_cmp(s1p: *const Limb, s2p: *const Limb, n: c_long) -> c_int;

    pub fn __gmpn_mul_n(rp: *mut Limb, s1p: *const Limb, s2p: *const Limb, n: c_long);

    pub fn __gmpn_tdiv_qr(
        qp: *mut Limb,
        rp: *mut Limb,
        qxn: c_long,
        np: *const Limb,
        nn: c_long,
        dp: *const Limb,
        dn: c_long,
    );
}
