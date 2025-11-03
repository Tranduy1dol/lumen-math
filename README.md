# mathlib

### Structure
```
mathlib/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   │
│   ├── arch/
│   │   ├── mod.rs
│   │   └── x86_64/
│   │       ├── mod.rs
│   │       └── avx2.rs
│   │
│   ├── num/
│   │   ├── mod.rs
│   │   ├── u256.rs
│   │   └── u512.rs
│   │
│   ├── field/
│   │   ├── mod.rs
│   │   └── montgomery.rs
│   │
│   └── poly/
│       ├── mod.rs
│       ├── fft.rs
│       └── ntt.rs
│
└── benches/
└── u256_arith.rs
```