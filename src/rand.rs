use crate::bit;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Rng {
    state: u32,
}

#[wasm_bindgen]
impl Rng {
    #[wasm_bindgen(constructor)]
    pub fn new(init_state: Option<u32>) -> Self {
        Rng {
            state: init_state.unwrap_or(0xaae21259),
        }
    }

    pub fn state(&self) -> u32 {
        self.state
    }

    pub fn rand(&mut self) -> u8 {
        let upper16 = bit::range(self.state, 16, 32) as u16;
        let lower16 = bit::range(self.state, 0, 16) as u16;
        let tmp = ((lower16 << 2) ^ upper16) << 1;
        let lowest8 = (tmp >> 8) as u8;

        self.state = (self.state << 8) | (lowest8 as u32);

        return lowest8;
    }

    pub fn rand_by_multiply(&mut self, upper: u8) -> u8 {
        let rand = self.rand();

        if upper == u8::max_value() {
            return rand;
        }

        let ret = (rand as u16) * ((upper as u16) + 1) / (u8::max_value() as u16 + 1);
        return ret as u8;
    }

    pub fn rand_by_mask(&mut self, offset: u8, mask: u8) -> u8 {
        assert!(bit::is_powerof2(mask as u16 + 1));

        let iter = 16;
        let mut sum = offset as u16;
        for _ in 0..iter {
            sum += (self.rand() & mask) as u16;
        }

        return (sum & 0xff) as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::Rng;

    #[test]
    fn rand_test() {
        let mut rng = Rng::new(None);

        assert_eq!(rng.state(), 0xaae21259);
        assert_eq!(rng.rand(), 0xc7);
        assert_eq!(rng.state(), 0xe21259c7);
    }

    #[test]
    fn rand_by_multiply_test() {
        let mut rng = Rng::new(None);
        let upper: u8 = 16;

        assert_eq!(rng.state(), 0xaae21259);
        assert_eq!(
            rng.rand_by_multiply(upper),
            (0xc7 * ((upper as u16) + 1) / 256) as u8
        );
        assert_eq!(rng.state(), 0xe21259c7);
    }

    #[test]
    fn rand_by_mask_test() {
        let mut rng = Rng::new(None);
        let mask: u8 = 31;
        let offset: u8 = 136;

        assert_eq!(rng.state(), 0xaae21259);
        // TODO: Check the result.
        rng.rand_by_mask(offset, mask);
    }

    #[test]
    #[should_panic]
    fn rand_by_mask_panic_test() {
        let mut rng = Rng::new(None);
        let mask: u8 = 30;

        rng.rand_by_mask(0, mask);
    }
}
