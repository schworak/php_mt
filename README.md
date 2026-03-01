# php_mt

Bit-for-bit compatible implementation of PHP 7.1+ MT19937 (`mt_rand`) in Rust.

This crate reproduces the exact output of:

- `mt_srand(seed)`
- `mt_rand()`
- `mt_rand(min, max)`

for PHP 7.1 and newer.

It is intended for deterministic cross-language compatibility and reproducible
test vectors — **not** for cryptographic use.

---

## Features

- Exact Zend Engine MT19937 constants
- 624-element state array
- 31-bit output for `mt_rand()` (matches PHP)
- Integer rejection sampling for `mt_rand(min, max)`
- No floating point scaling
- No external dependencies
- Fully deterministic

---

## Example

```rust
use php_mt::PhpMt;

let mut rng = PhpMt::new(1234);

assert_eq!(rng.mt_rand(), 411284887);
assert_eq!(rng.mt_rand_range(0, 100), 20);