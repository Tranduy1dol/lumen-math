use std::env;

fn main() {
    if env::var("CARGO_FEATURE_GMP").is_ok() {
        match pkg_config::Config::new().probe("gmp") {
            Ok(_) => {
                println!("cargo:rustc-link-lib=gmp");
            }
            Err(_) => {
                println!("cargo:rustc-link-lib=gmp");
            }
        }
    }

    println!("cargo:rerun-if-changed=build.rs");
}
