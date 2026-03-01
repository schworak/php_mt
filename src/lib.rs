//! # PhpMt
//!
//! A Rust implementation of PHP's MT19937-based random number generator.
//!
//! This implementation is bit-for-bit compatible with PHP 7.1+ for:
//! - `mt_srand(seed)`
//! - `mt_rand()`
//! - `mt_rand(min, max)`
//!
//! Notes:
//! - In PHP 7.1+, `rand()` is an alias of `mt_rand()`.
//! - In PHP 7.1+, `srand(seed)` is an alias of `mt_srand(seed)`.
//! - This implementation matches the Zend Engine MT19937 algorithm.
//!
//! Implementation details:
//! - 624-element MT19937 state array
//! - Exact Zend tempering constants
//! - `mt_rand()` returns 31-bit output (`next_u32() >> 1`)
//! - `mt_rand(min, max)` uses integer rejection sampling (no float scaling)
//!
//! This crate is intended for deterministic cross-language compatibility
//! and reproducible test vectors.
//!
//! Not cryptographically secure.

#![forbid(unsafe_code)]

mod tests;

use std::time::{SystemTime, UNIX_EPOCH};

pub struct PhpMt {
    state: [u32; 624],
    index: usize,
}

impl PhpMt {
    /// Creates a new PHP Mersenne Twister with a specific seed.
    ///
    /// # Example
    /// ```
    /// use php_mt::PhpMt;
    /// let mut rng = PhpMt::new(1234);
    /// assert_eq!(rng.mt_rand(), 411284887);
    /// ```
    pub fn new(seed: u32) -> Self {
        let mut mt = PhpMt {
            state: [0; 624],
            index: 624,
        };

        mt.seed(seed);
        mt
    }

    /// Creates a new PHP Mersenne Twister with no seed.
    ///
    /// # Example
    /// ```
    /// use php_mt::PhpMt;
    /// let mut rng = PhpMt::new_auto();
    /// assert_ne!(rng.mt_rand(), 0);
    /// ```
    pub fn new_auto() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos(); // u32

        Self::new(seed)
    }

    /// Reset the seed after creating with/without an initial seed
    ///
    /// # Example
    /// ```
    /// use php_mt::PhpMt;
    /// let mut rng = PhpMt::new_auto();
    /// assert_ne!(rng.mt_rand(), 0);
    /// rng.seed(1234);
    /// assert_ne!(rng.mt_rand(), 0);

    /// ```
    pub fn seed(&mut self, seed: u32) {
        self.state[0] = seed;

        for i in 1..624 {
            self.state[i] = 1812433253u32
                .wrapping_mul(self.state[i - 1] ^ (self.state[i - 1] >> 30))
                .wrapping_add(i as u32);
        }

        self.index = 624;
    }

    fn twist(&mut self) {
        const UPPER_MASK: u32 = 0x80000000;
        const LOWER_MASK: u32 = 0x7fffffff;
        const MATRIX_A: u32 = 0x9908b0df;

        for i in 0..624 {
            let x = (self.state[i] & UPPER_MASK) | (self.state[(i + 1) % 624] & LOWER_MASK);

            let mut x_a = x >> 1;

            if x & 1 != 0 {
                x_a ^= MATRIX_A;
            }

            self.state[i] = self.state[(i + 397) % 624] ^ x_a;
        }

        self.index = 0;
    }

    fn next_u32(&mut self) -> u32 {
        if self.index >= 624 {
            self.twist();
        }

        let mut y = self.state[self.index];
        self.index += 1;

        // Tempering (exact MT19937)
        y ^= y >> 11;
        y ^= (y << 7) & 0x9d2c5680;
        y ^= (y << 15) & 0xefc60000;
        y ^= y >> 18;

        y
    }

    /// Equivalent to PHP mt_rand()
    ///
    /// # Example
    /// ```
    /// use php_mt::PhpMt;
    /// let mut rng = PhpMt::new(1234);
    /// assert_eq!(rng.mt_rand(), 411284887);
    /// assert_eq!(rng.mt_rand(), 1068724585);
    /// assert_eq!(rng.mt_rand(), 1335968403);
    /// ```
    pub fn mt_rand(&mut self) -> u32 {
        self.next_u32() >> 1
    }

    /// Equivalent to PHP mt_rand(min, max)
    ///
    /// # Example
    /// ```
    /// use php_mt::PhpMt;
    /// let mut rng = PhpMt::new(1234);
    /// assert_eq!(rng.mt_rand_range(0,100), 20);
    /// assert_eq!(rng.mt_rand_range(0,100), 8);
    /// assert_eq!(rng.mt_rand_range(0,100), 87);
    /// ```
    pub fn mt_rand_range(&mut self, min: u32, max: u32) -> u32 {
        if min == max {
            return min;
        }

        assert!(min < max);

        let range = max - min;
        let range_plus_one = range.wrapping_add(1);

        // Equivalent to PHP's UINT32_MAX
        let max_u32 = u32::MAX;

        // Largest multiple of range+1 that fits in u32
        let limit = max_u32 - (max_u32 % range_plus_one);

        loop {
            let r = self.next_u32();
            if r <= limit {
                return min + (r % range_plus_one);
            }
        }
    }
}
