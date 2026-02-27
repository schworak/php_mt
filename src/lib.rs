use std::time::{SystemTime, UNIX_EPOCH};

pub struct PhpMt {
    state: [u32; 624],
    index: usize,
}

impl PhpMt {
    pub fn new(seed: u32) -> Self {
        let mut mt = PhpMt {
            state: [0; 624],
            index: 624,
        };

        mt.seed(seed);
        mt
    }

    pub fn new_auto() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos(); // u32

        Self::new(seed)
    }

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
    pub fn mt_rand(&mut self) -> u32 {
        self.next_u32() >> 1
    }

    /// Equivalent to PHP mt_rand(min, max)
    pub fn mt_rand_range(&mut self, min: u32, max: u32) -> u32 {
        assert!(min <= max);

        let range = max - min + 1;
        min + (self.mt_rand() % range)
    }
}
