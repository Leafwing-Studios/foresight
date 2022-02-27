/// A dead simple RNG function that generates a sequence of u8 values
/// Repeats after approximately 17000 steps
///
/// Based on the SM64 RNG function
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleRNG {
    s: u8,
    t: u8,
}

impl SimpleRNG {
    /// Create a new RNG with the given internal state
    #[must_use]
    pub fn new(s: u8, t: u8) -> Self {
        SimpleRNG { s, t }
    }

    /// Generates a new random number between 0 and 255
    #[must_use]
    pub fn gen(&mut self) -> u8 {
        self.step();
        self.s ^ self.t
    }

    /// Advances the RNG once
    fn step(&mut self) {
        self.s = self.s.wrapping_mul(5).wrapping_add(1);
        self.t = self.t.wrapping_mul(2);

        if Self::get_bit_at(self.t, 4) == Self::get_bit_at(self.t, 7) {
            self.t = self.t.wrapping_add(1);
        }
    }

    fn get_bit_at(input: u8, n: u8) -> bool {
        if n < 8 {
            input & (1 << n) != 0
        } else {
            false
        }
    }
}

impl Default for SimpleRNG {
    fn default() -> Self {
        // In general, it is bad to always initialize your RNG function to the same value
        // But this isn't supposed to be a good RNG function >:3
        Self { s: 42, t: 69 }
    }
}
