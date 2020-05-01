use crate::bit;

#[derive(Debug)]
pub struct Rng {
    state: u32,
}

impl Rng {
    pub fn new(init_state: impl Into<Option<u32>>) -> Self {
        Rng {
            state: init_state.into().unwrap_or(0xaae21259),
        }
    }

    pub fn state(&self) -> u32 {
        self.state
    }

    pub fn rand(&mut self, upper: impl Into<Option<u8>>) -> u8 {
        let upper = upper.into().unwrap_or(u8::max_value());

        let upper16 = bit::range(self.state, 16, 32) as u16;
        let lower16 = bit::range(self.state, 0, 16) as u16;
        let tmp = ((lower16 << 2) ^ upper16) << 1;
        let lowest8 = (tmp >> 8) as u8;

        self.state = (self.state << 8) | (lowest8 as u32);

        let ret = (lowest8 as u16) * ((upper as u16) + 1) / (u8::max_value() as u16 + 1);
        return ret as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::Rng;

    #[test]
    fn rng_test() {
        let mut rng = Rng::new(None);

        assert_eq!(rng.state(), 0xaae21259);
        assert_eq!(rng.rand(None), 0xc7);
        assert_eq!(rng.state(), 0xe21259c7);

        let upper: u8 = 16;
        assert_eq!(rng.rand(upper), (0x0a * (upper as u16) / 256) as u8);
        assert_eq!(rng.state(), 0x1259c70a);
    }
}
