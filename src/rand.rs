use crate::bit;
use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct State(u32);

#[wasm_bindgen]
impl State {
    pub fn next(state: &State) -> State {
        let upper16 = bit::range(state.0, 16, 32) as u16;
        let lower16 = bit::range(state.0, 0, 16) as u16;
        let tmp = ((lower16 << 2) ^ upper16) << 1;
        let lowest8 = (tmp >> 8) as u8;

        return State((state.0 << 8) | (lowest8 as u32));
    }

    pub fn transit(&mut self) {
        *self = State::next(self);
    }

    pub fn rand(&self) -> u8 {
        return (self.0 & (u8::max_value() as u32)) as u8;
    }

    pub fn rand_by_multiply(&self, upper: u8) -> u8 {
        let rand = self.rand();
        if upper == u8::max_value() {
            return rand;
        }

        let ret = (rand as u16) * ((upper as u16) + 1) / (u8::max_value() as u16 + 1);
        return ret as u8;
    }

    pub fn rand_by_mask(&self, offset: u8, mask: u8) -> u8 {
        assert!(bit::is_powerof2(mask as u16 + 1));

        let iter = 16;
        let mut sum = offset as u16;
        for _ in 0..iter {
            sum += (self.rand() & mask) as u16;
        }

        return (sum & 0xff) as u8;
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Generator {
    state: State,
}

#[wasm_bindgen]
impl Generator {
    #[wasm_bindgen(constructor)]
    pub fn new(init: Option<State>) -> Self {
        Generator {
            state: init.unwrap_or(State(0xaae21259)),
        }
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn transit(&mut self) {
        self.state.transit();
    }

    pub fn rand(&mut self) -> u8 {
        self.state.transit();
        return self.state.rand();
    }

    pub fn rand_by_multiply(&mut self, upper: u8) -> u8 {
        self.state.transit();
        return self.state.rand_by_multiply(upper);
    }

    pub fn rand_by_mask(&mut self, offset: u8, mask: u8) -> u8 {
        self.state.transit();
        return self.state.rand_by_mask(offset, mask);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rand_test() {
        let mut rng = Generator::new(None);

        assert_eq!(rng.state(), State(0xaae21259));
        assert_eq!(rng.rand(), 0xc7);
        assert_eq!(rng.state(), State(0xe21259c7));
    }

    #[test]
    fn rand_print_test() {
        let rng = Generator::new(None);

        assert_eq!(format!("{}", rng.state()), "0xaae21259");
    }

    #[test]
    fn rng_clone_test() {
        let rng = Generator::new(None);
        let mut rng_tmp = rng.clone();

        assert_eq!(rng_tmp.state(), State(0xaae21259));
        rng_tmp.transit();
        assert_eq!(rng_tmp.state(), State(0xe21259c7));
        assert_eq!(rng.state(), State(0xaae21259));
    }

    #[test]
    fn rand_by_multiply_test() {
        let mut rng = Generator::new(None);
        let upper: u8 = 16;

        assert_eq!(rng.state(), State(0xaae21259));
        assert_eq!(rng.rand_by_multiply(upper), 13_u8);
        assert_eq!(rng.state(), State(0xe21259c7));
    }

    #[test]
    fn rand_by_mask_test() {
        let mut rng = Generator::new(None);
        let mask: u8 = 31;
        let offset: u8 = 136;

        assert_eq!(rng.state(), State(0xaae21259));
        // TODO: Check the result.
        rng.rand_by_mask(offset, mask);
    }

    #[test]
    #[should_panic]
    fn rand_by_mask_panic_test() {
        let mut rng = Generator::new(None);
        let mask: u8 = 30;

        rng.rand_by_mask(0, mask);
    }
}
