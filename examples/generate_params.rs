use num_bigint::{BigUint, RandBigInt};
use num_integer::Integer;
use num_traits::One;

fn main() {
    println!("🔍 Searching for 1024-bit NTT-friendly prime...");

    // Mục tiêu: Tìm P = k * 2^32 + 1 sao cho P ~ 2^1024
    let n_power = 32; // Hỗ trợ NTT size tối đa 2^32
    let two_pow_32 = BigUint::one() << n_power;

    // Target bit size = 1024.
    // k cần có kích thước khoảng 1024 - 32 = 992 bits.
    let mut rng = rand::thread_rng();

    loop {
        // 1. Random k (số lẻ để đảm bảo P là số lẻ)
        let k = rng.gen_biguint(992);
        if k.is_even() {
            continue;
        }

        // 2. Tính P = k * 2^32 + 1
        let p: BigUint = &k * &two_pow_32 + BigUint::one();

        // 3. Kiểm tra tính nguyên tố (Rabin-Miller test)
        // is_prob_prime(20) cho độ tin cậy rất cao
        if p.bits() == 1024 && is_prob_prime(&p, 20) {
            println!("✅ FOUND PRIME P!");
            println!("P (Hex) = 0x{:X}", p);

            // 4. Tìm Căn nguyên thủy (Primitive Root of Unity)
            // Tìm g sao cho order của g là P-1.
            // Sau đó w = g^k mod P sẽ là căn nguyên thủy bậc 2^32.
            let root = find_primitive_root(&p, &k, n_power);
            println!("✅ FOUND ROOT OF UNITY w (order 2^32)!");
            println!("w (Hex) = 0x{:X}", root);
            break;
        }
    }
}

fn find_primitive_root(p: &BigUint, k: &BigUint, n: u32) -> BigUint {
    // Ta cần tìm w sao cho w^(2^n) = 1 mod P và w^(2^(n-1)) != 1 mod P
    // Cách làm: Chọn ngẫu nhiên g, tính w = g^k mod P.
    // Kiểm tra nếu w^(2^(n-1)) != 1 thì w là căn nguyên thủy bậc 2^n.

    let mut rng = rand::thread_rng();
    let one = BigUint::one();
    let two_pow_n_minus_1 = BigUint::one() << (n - 1);

    loop {
        // Chọn g ngẫu nhiên trong [2, P-1]
        let g = rng.gen_biguint_range(&BigUint::from(2u32), p);

        // w = g^k mod P
        let w = g.modpow(k, p);

        // Check order: w^(2^(32-1)) != 1
        let check = w.modpow(&two_pow_n_minus_1, p);

        if check != one && w != one {
            return w;
        }
    }
}

fn is_prob_prime(n: &BigUint, k: usize) -> bool {
    if n <= &BigUint::from(1u32) {
        return false;
    }
    if n <= &BigUint::from(3u32) {
        return true;
    }
    if n.is_even() {
        return false;
    }

    // Viết n-1 = d * 2^r
    let one = BigUint::one();
    let two = BigUint::from(2u32);
    let n_minus_1 = n - &one;

    let mut d = n_minus_1.clone();
    let mut r = 0;
    while d.is_even() {
        d >>= 1;
        r += 1;
    }

    let mut rng = rand::thread_rng();
    'witness: for _ in 0..k {
        // Chọn a ngẫu nhiên trong [2, n-2]
        // n_minus_1 = n - 1. Range [2, n-1) tức là [2, n-2].
        let a = rng.gen_biguint_range(&two, &n_minus_1);

        let mut x = a.modpow(&d, n);

        if x == one || x == n_minus_1 {
            continue;
        }

        for _ in 0..r - 1 {
            x = x.modpow(&two, n);
            if x == n_minus_1 {
                continue 'witness;
            }
        }
        return false; // Hợp số
    }
    true // Có khả năng cao là nguyên tố
}
