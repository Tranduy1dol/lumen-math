// This assumes `u256.rs` is part of a `num` module exposed by your library.
// If not, you might need `mod num;` here.
use mathlib::num::u256::U256;
use std::hint::black_box;

// We use #[inline(never)] to make sure this function is not optimized away
// and is easy to find in the assembly output.
#[inline(never)]
fn u256_add_example(a: &U256, b: &U256) -> U256 {
    // This will call your `carrying_add` implementation.
    a + b
}

// A separate function for subtraction.
#[inline(never)]
fn u256_sub_example(a: &U256, b: &U256) -> U256 {
    // This will call your `borrowing_sub` implementation.
    a - b
}

fn main() {
    // Create two U256 instances.
    let a = U256([100, 200, 300, 400]);
    let b = U256([50, 60, 70, 80]);

    // Use black_box to ensure the function calls are not optimized out.
    let sum = u256_add_example(black_box(&a), black_box(&b));
    let diff = u256_sub_example(black_box(&a), black_box(&b));

    // Print the results to make sure they are used.
    println!("Sum: {:?}, Diff: {:?}", sum, diff);
}
