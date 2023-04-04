//! This implements xoroshiro128+
//! It is just a rust port of the implementation at <https://prng.di.unimi.it/xoroshiro128plus.c>

/// This structure contains the state of the xoroshiro128++ generator
pub struct Generator {
    state: [u64; 2],
}

impl Generator {
    /// The primary way to generate things with the generator. Modifies the
    /// internal state and then returns the generated u64.
    pub fn next(&mut self) -> u64 {
        let s0 = self.state[0];
        let mut s1 = self.state[1];
        let result = s0.wrapping_add(s1).rotate_left(17).wrapping_add(s0);

        s1 ^= s0;
        self.state[0] = s0.rotate_left(49) ^ s1 ^ (s1 << 21);
        self.state[1] = s1.rotate_left(28);

        return result;
    }

    /// Creates a new xoroshiro128++ generator using the given seeds
    pub fn new(seed1: u64, seed2: u64) -> Self {
        Self {
            state: [seed1, seed2],
        }
    }

    /// Creates a new xoroshiro128++ generator seeded with random numbers from
    /// the current generator.
    pub fn birth(&mut self) -> Generator {
        Self::new(self.next(), self.next())
    }
}

#[cfg(test)]
mod tests {
    use super::Generator;

    /// Test the first ten numbers output with seed (7, 10)
    #[test]
    fn first_10() {
        let mut gen = Generator::new(7, 10);
        assert_eq!(gen.next(), 0x220007);
        assert_eq!(gen.next(), 0xfa34001ba0029);
        assert_eq!(gen.next(), 0x6849741e969a1d9d);
        assert_eq!(gen.next(), 0xcafd8f10eb5d369);
        assert_eq!(gen.next(), 0x91f9fe1686b03485);
        assert_eq!(gen.next(), 0xb6e96e3cfcf8eddc);
        assert_eq!(gen.next(), 0x579f34186bfbf901);
        assert_eq!(gen.next(), 0xca6ea6f7073072d);
        assert_eq!(gen.next(), 0x400245371acd282e);
        assert_eq!(gen.next(), 0xeffe6df09c4f306f);
    }

    /// Test the birthing feature
    /// The input seed is (7, 10), and the first is dumped, so the birthed seed
    /// should be (0xfa34001ba0029, 0x68p9741e969a1d9d).
    #[test]
    fn birth() {
        let mut gen = Generator::new(7, 10);
        let mut birthed = gen.birth();
        assert_eq!(birthed.next(), 0x468003b800820026);
        assert_eq!(birthed.next(), 0x888758f1f994d0a0);
        assert_eq!(birthed.next(), 0x1699369376fe1099);
        assert_eq!(birthed.next(), 0xd0cc859916ba43d6);
        assert_eq!(birthed.next(), 0xb6ac9c4998d25a8b);
        assert_eq!(birthed.next(), 0x8321603713f661b0);
        assert_eq!(birthed.next(), 0x1a027a3babf373ee);
        assert_eq!(birthed.next(), 0x3bcb23b95e559eb7);
        assert_eq!(birthed.next(), 0x74b61526d1675e86);
        assert_eq!(birthed.next(), 0xaf9f92d6fc46196f);
    }
}
