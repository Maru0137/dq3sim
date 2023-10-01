use crate::bit;

use std::borrow::Borrow;
use std::cell::UnsafeCell;
use std::fmt;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct State(u32);

/// A newtype for 32-bit state value in a Random Number Generator
#[wasm_bindgen]
impl State {
    #[wasm_bindgen(constructor)]
    pub fn new(v: u32) -> Self {
        Self(v)
    }

    pub fn init() -> Self {
        Self::default()
    }

    /// Return the next state of RNG.
    pub fn next(&self) -> State {
        let upper16 = bit::range(self.0, 16, 32) as u16;
        let lower16 = bit::range(self.0, 0, 16) as u16;
        let tmp = ((lower16 << 2) ^ upper16) << 1;
        let lowest8 = (tmp >> 8) as u8;

        return State((self.0 << 8) | (lowest8 as u32));
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

    pub fn rand_by_mask(&self, mask: u8) -> u8 {
        return self.rand() & mask;
    }
}

impl Default for State {
    fn default() -> Self {
        Self(0xaae21259)
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}

impl From<u32> for State {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl AsRef<u32> for State {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}

impl Borrow<u32> for State {
    fn borrow(&self) -> &u32 {
        &self.0
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Default)]
pub struct Rng {
    state: State,
}

/// A Random Number Generator.
#[wasm_bindgen]
impl Rng {
    pub fn state(&self) -> State {
        self.state
    }

    pub fn init(&mut self) {
        self.state = State::default();
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }

    /// Transit the state of RNG.
    pub fn transit(&mut self) {
        self.state.transit();
    }

    /// Return a 8-bit(i.e. [0, 255]) uniform random number.
    ///
    /// This method simulates the subroutine at 0x0012e3.
    pub fn rand(&mut self) -> u8 {
        self.transit();
        return self.state.rand();
    }

    /// Return a 8-bit random number inbound [0, upper].
    ///
    /// ret = rand() * 256 / (upper + 1)
    /// This method simulates the subroutine at 0x00133e.
    pub fn rand_by_multiply(&mut self, upper: u8) -> u8 {
        self.transit();
        return self.state.rand_by_multiply(upper);
    }

    /// Return a 8-bit multinomial random number.
    ///
    /// ret = (offset + multi(n=16, p= rand() & mask)) % 256
    ///
    /// This method simulates the subroutine at 0x0014d4.
    pub fn rand_multinomial(&mut self, offset: u8, mask: u8) -> u8 {
        assert!(bit::is_powerof2(mask as u16 + 1));

        let mut sum = offset as u16;
        let iter = 16;
        for _ in 0..iter {
            self.transit();
            sum += self.state.rand_by_mask(mask) as u16;
        }

        return (sum & 0xff) as u8;
    }
}

impl From<State> for Rng {
    fn from(state: State) -> Self {
        Self { state }
    }
}

impl From<u32> for Rng {
    fn from(state_value: u32) -> Self {
        Rng::from(State::from(state_value))
    }
}

#[derive(Clone, Debug)]
pub struct ThreadRng {
    rng: Rc<UnsafeCell<Rng>>,
}

impl ThreadRng {
    pub fn state(&self) -> State {
        unsafe { (*self.rng.get()).state() }
    }

    pub fn set_state(&mut self, state: State) {
        let rng = unsafe { &mut *self.rng.get() };
        rng.set_state(state);
    }

    /// Transit the state of RNG.
    pub fn transit(&mut self) {
        let rng = unsafe { &mut *self.rng.get() };
        rng.state.transit();
    }

    /// Return a 8-bit(i.e. [0, 255]) uniform random number.
    ///
    /// This method simulates the subroutine at 0x0012e3.
    pub fn rand(&mut self) -> u8 {
        let rng = unsafe { &mut *self.rng.get() };
        rng.rand()
    }

    /// Return a 8-bit random number inbound [0, upper].
    ///
    /// ret = rand() * 256 / (upper + 1)
    ///
    /// This method simulates the subroutine at 0x00133e.
    pub fn rand_by_multiply(&mut self, upper: u8) -> u8 {
        let rng = unsafe { &mut *self.rng.get() };
        rng.rand_by_multiply(upper)
    }

    /// Return a 8-bit multinomial random number.
    ///
    /// ret = (offset + multi(n=16, p= rand() & mask)) % 256
    ///
    /// This method simulates the subroutine at 0x0014d4.
    pub fn rand_multinomial(&mut self, offset: u8, mask: u8) -> u8 {
        let rng = unsafe { &mut *self.rng.get() };
        rng.rand_multinomial(offset, mask)
    }
}

thread_local! {
    static THREAD_RNG_KEY: Rc<UnsafeCell<Rng>> = Rc::new(UnsafeCell::new(Rng::default()));
}

pub fn thread_rng() -> ThreadRng {
    let rng = THREAD_RNG_KEY.with(|t| t.clone());
    ThreadRng { rng }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rand() {
        let mut rng = Rng::default();

        assert_eq!(rng.state(), State(0xaae21259));
        assert_eq!(rng.rand(), 0xc7);
        assert_eq!(rng.state(), State(0xe21259c7));
    }

    #[test]
    fn test_state_format() {
        let rng = Rng::default();

        assert_eq!(format!("{}", rng.state()), "0xaae21259");
    }

    #[test]
    fn test_rng_clone() {
        let rng = Rng::default();
        let mut rng_tmp = rng.clone();

        assert_eq!(rng_tmp.state(), State(0xaae21259));
        rng_tmp.transit();
        assert_eq!(rng_tmp.state(), State(0xe21259c7));
        assert_eq!(rng.state(), State(0xaae21259));
    }

    #[test]
    fn test_rand_by_multiply() {
        let mut rng = Rng::default();
        let upper: u8 = 16;

        assert_eq!(rng.state(), State(0xaae21259));
        assert_eq!(rng.rand_by_multiply(upper), 13_u8);
        assert_eq!(rng.state(), State(0xe21259c7));
    }

    #[test]
    fn test_rand_by_mask() {
        let mut rng = Rng::default();
        let mask: u8 = 31;
        let offset: u8 = 136;

        assert_eq!(rng.state(), State(0xaae21259));
        // TODO: Check the result.
        rng.rand_multinomial(offset, mask);
    }

    #[test]
    #[should_panic]
    fn test_rand_by_mask_panic() {
        let mut rng = Rng::default();
        let mask: u8 = 30;

        rng.rand_multinomial(0, mask);
    }

    #[test]
    fn test_state_hash() {
        let mut hist = vec![0; u32::max_value() as usize];

        let state0 = State::default();
        println!("{}", *state0.as_ref());
        hist[*state0.as_ref() as usize] += 1;

        let state1 = state0.next();
        hist[*state1.as_ref() as usize] += 1;

        assert_eq!(hist[*state0.as_ref() as usize], 1);
        assert_eq!(hist[*state1.as_ref() as usize], 1);
    }
}
