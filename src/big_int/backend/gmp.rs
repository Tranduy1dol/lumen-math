use libc::c_ulong;

pub type Limb = u64;

#[link(name = "gmp")]
unsafe extern "C" {
    pub fn __gmpn_add_n(rp: *mut Limb, s1p: *const Limb, s2p: *const Limb, n: c_ulong) -> Limb;

    pub fn __gmpn_sub_n(rp: *mut Limb, s1p: *const Limb, s2p: *const Limb, n: c_ulong) -> Limb;

    pub fn __gmpn_cmp(s1p: *const Limb, s2p: *const Limb, n: c_ulong) -> std::ffi::c_int;
}
