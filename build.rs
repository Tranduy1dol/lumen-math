fn main() {
    pkg_config::Config::new()
        .probe("gmp")
        .expect("GMP library not found. Please install libgmp-dev or gmp-devel");

    println!("cargo:rustc-link-lib=gmp");
    println!("cargo:rerun-if-changed=build.rs");
}
