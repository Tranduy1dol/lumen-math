# Mathematical Theory & Concepts in lumen-math

This document explains the mathematical concepts, algorithms, and theory implemented in the lumen-math library.

---

## Table of Contents

1. [Big Integer Arithmetic](#big-integer-arithmetic)
2. [Finite Field Arithmetic](#finite-field-arithmetic)
3. [Montgomery Multiplication](#montgomery-multiplication)
4. [Polynomial Operations](#polynomial-operations)
5. [Number Theoretic Transform (NTT)](#number-theoretic-transform-ntt)
6. [Negacyclic NTT](#negacyclic-ntt)
7. [Number Theory Algorithms](#number-theory-algorithms)
8. [Cryptographic Applications](#cryptographic-applications)

---

## Big Integer Arithmetic

### U1024 - 1024-bit Unsigned Integers

The `U1024` type represents unsigned integers up to 2^1024 - 1, stored as 16 × 64-bit limbs in little-endian order.

**Representation:**
```
U1024([limb₀, limb₁, ..., limb₁₅])
value = limb₀ + limb₁·2⁶⁴ + limb₂·2¹²⁸ + ... + limb₁₅·2⁹⁶⁰
```

**Operations implemented:**
- Addition/Subtraction with carry/borrow propagation
- Multiplication using schoolbook algorithm: O(n²)
- Division using binary long division
- Left/Right bit shifts
- Modular exponentiation (square-and-multiply)

### I1024 - 1024-bit Signed Integers

The `I1024` type uses magnitude + sign representation for signed arithmetic.

---

## Finite Field Arithmetic

### Prime Fields F_p

A prime field F_p is the set of integers {0, 1, 2, ..., p-1} with arithmetic modulo prime p.

**Properties:**
- Closure: a + b, a × b ∈ F_p
- Inverse exists for all non-zero elements: a⁻¹ where a · a⁻¹ ≡ 1 (mod p)
- Computed via Fermat's Little Theorem: a⁻¹ = a^(p-2) mod p

### FieldElement<C>

Generic field element parameterized by a `FieldConfig` trait that defines:
- `MODULUS`: The prime p
- `R2`: R² mod p (for Montgomery form)
- `N_PRIME`: Montgomery constant
- `ROOT_OF_UNITY`: Primitive Nth root of unity

---

## Montgomery Multiplication

Montgomery multiplication enables fast modular multiplication without division.

### Key Idea

Instead of computing a · b mod p directly, work in "Montgomery form":
- Convert: ā = a · R mod p (where R = 2^1024)
- Multiply: ā · b̄ · R⁻¹ mod p = (a·b·R) mod p
- Convert back: result · R⁻¹ mod p

### Montgomery Reduction (REDC)

Given T = a · b (2048-bit product), compute T · R⁻¹ mod p:

```
function REDC(T):
    m = (T mod R) · N' mod R      // N' satisfies N·N' ≡ -1 (mod R)
    t = (T + m·N) / R
    if t ≥ N: return t - N
    return t
```

### Constants

- **R** = 2^1024 (implicit, power of 2 for efficiency)
- **R²** = R² mod p (for converting to Montgomery form)
- **N'** = -p⁻¹ mod R (Montgomery constant)

---

## Polynomial Operations

### Univariate Polynomials

A polynomial p(x) = c₀ + c₁x + c₂x² + ... + cₙxⁿ over F_p.

**Implemented operations:**
- Arithmetic: +, -, ×, ÷ with remainder
- Evaluation using **Horner's Method**: O(n)
- Derivative: p'(x) = c₁ + 2c₂x + 3c₃x² + ...
- **Lagrange Interpolation**: Given n points, find unique polynomial of degree < n

### Horner's Method

Efficient evaluation: p(x) = c₀ + x(c₁ + x(c₂ + x(...)))

```
result = cₙ
for i from n-1 down to 0:
    result = result · x + cᵢ
```

Time: O(n) multiplications instead of O(n²) for naive method.

### Lagrange Interpolation

Given points (x₀, y₀), ..., (xₙ₋₁, yₙ₋₁), the interpolating polynomial is:

```
p(x) = Σᵢ yᵢ · Lᵢ(x)

where Lᵢ(x) = Πⱼ≠ᵢ (x - xⱼ)/(xᵢ - xⱼ)
```

### Multivariate Polynomials

Sparse representation using `BTreeMap<Exponent, Coefficient>` where exponent is a vector [e₀, e₁, ..., eₖ] representing x₀^e₀ · x₁^e₁ · ... · xₖ^eₖ.

---

## Number Theoretic Transform (NTT)

The NTT is the finite field analog of the FFT, enabling O(n log n) polynomial multiplication.

### Cyclic NTT

Operates over the ring Z_q[X]/(X^N - 1).

**Forward NTT:**
Given coefficients [c₀, c₁, ..., cₙ₋₁], compute evaluations at powers of ω:

```
ĉₖ = Σⱼ cⱼ · ωʲᵏ mod q
```

where ω is a primitive Nth root of unity (ω^N ≡ 1 mod q).

**Inverse NTT:**
```
cⱼ = N⁻¹ · Σₖ ĉₖ · ω⁻ʲᵏ mod q
```

### Cooley-Tukey Algorithm

In-place butterfly computation with bit-reversal permutation:

```
for each layer len = 2, 4, 8, ..., N:
    ω_len = ω^(N/len)      // twiddle factor
    for each group:
        w = 1
        for j in half_len:
            u = coeffs[j]
            v = coeffs[j + half_len] · w
            coeffs[j] = u + v
            coeffs[j + half_len] = u - v
            w = w · ω_len
```

### Requirements

1. N must be a power of 2
2. q must be prime with q ≡ 1 (mod N)
3. ω must be a primitive Nth root of unity: ω^N ≡ 1, ω^(N/2) ≢ 1

---

## Negacyclic NTT

For lattice-based cryptography (Kyber, Dilithium), we need polynomial multiplication in the ring:

**R_q = Z_q[X]/(X^N + 1)**

This means X^N ≡ -1, so coefficients "wrap around" with negation.

### Primitive 2Nth Root of Unity

Requires ψ such that ψ^N ≡ -1 (mod q) and ψ^(2N) ≡ 1 (mod q).

### Algorithm (Twist Method)

**Forward Negacyclic NTT:**
1. Pre-multiply: c'ᵢ = cᵢ · ψⁱ
2. Apply standard NTT with ω = ψ²

**Inverse Negacyclic NTT:**
1. Apply inverse NTT
2. Post-multiply: cᵢ = c'ᵢ · ψ⁻ⁱ · N⁻¹

### Field Configurations

| Scheme | q | N | ψ (2Nth root) | ω = ψ² |
|--------|---|---|---------------|--------|
| **Kyber** | 3329 | 256 | 17 | 289 |
| **Dilithium** | 8380417 | 256 | 1753 | 3073009 |

---

## Number Theory Algorithms

### Extended Euclidean Algorithm (ExtGCD)

Computes gcd(a, b) and Bézout coefficients x, y such that:

**ax + by = gcd(a, b)**

```
function extended_gcd(a, b):
    if b = 0: return (a, 1, 0)
    (old_r, r) = (a, b)
    (old_s, s) = (1, 0)
    (old_t, t) = (0, 1)
    
    while r ≠ 0:
        q = old_r / r
        (old_r, r) = (r, old_r - q·r)
        (old_s, s) = (s, old_s - q·s)
        (old_t, t) = (t, old_t - q·t)
    
    return (old_r, old_s, old_t)
```

### Modular Inverse

a⁻¹ mod m exists iff gcd(a, m) = 1.

Using ExtGCD: if ax + my = 1, then a⁻¹ ≡ x (mod m).

### Chinese Remainder Theorem (CRT)

Given a system of congruences:
```
x ≡ a₁ (mod n₁)
x ≡ a₂ (mod n₂)
...
x ≡ aₖ (mod nₖ)
```

If all nᵢ are pairwise coprime, there exists a unique solution modulo N = n₁·n₂·...·nₖ:

```
x = Σᵢ aᵢ · Nᵢ · yᵢ (mod N)

where Nᵢ = N/nᵢ and yᵢ = Nᵢ⁻¹ mod nᵢ
```

---

## Cryptographic Applications

### Zero-Knowledge Proofs

- **Polynomial commitments**: Commit to polynomials, prove evaluations
- **Lagrange interpolation**: Reconstruct polynomials from point evaluations
- **FFT/NTT**: Fast polynomial multiplication in proof systems

### Lattice-Based Cryptography

The negacyclic NTT enables efficient operations in:

- **Kyber** (ML-KEM): Key encapsulation mechanism
- **Dilithium** (ML-DSA): Digital signature algorithm

Both operate over R_q = Z_q[X]/(X^N + 1) where polynomial multiplication is the core operation.

### Big Integer Cryptography

- **RSA**: Modular exponentiation of 2048+ bit integers
- **Elliptic Curves**: Large prime field arithmetic
- **Paillier**: Homomorphic encryption with large composites

---

## References

1. **Montgomery Multiplication**: P. Montgomery, "Modular Multiplication Without Trial Division" (1985)
2. **Cooley-Tukey FFT**: J. Cooley & J. Tukey, "An Algorithm for the Machine Calculation of Complex Fourier Series" (1965)
3. **NTT**: A. Agarwal & J. Cooley, "New algorithms for digital convolution" (1977)
4. **Kyber/Dilithium**: NIST Post-Quantum Cryptography Standardization
5. **Chinese Remainder Theorem**: Classical number theory result (Sun Tzu, ~3rd century CE)
