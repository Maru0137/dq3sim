use crate::rand::Rng;
use num;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum Character {
    Player,
    Monster,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Default)]
pub struct Battle {
    rng: Rng,
    // TODO: implement more
}

#[wasm_bindgen]
impl Battle {
    #[wasm_bindgen(constructor)]
    pub fn new(rng: Rng) -> Self {
        Self { rng }
    }

    fn physical_damage_0or1(&mut self) -> i16 {
        return (self.rng.rand() % 2) as i16;
    }

    fn physical_damage_defensive(&mut self, atk: i16) -> i16 {
        return (((atk / 8) as i32) * (self.rng.rand() as i32) / 256) as i16;
    }

    fn physical_damage_normal(&mut self, base: i16) -> i16 {
        const RAND_LOWER: u8 = 99;
        const RAND_UPPER: u8 = 153;
        let rand = num::clamp(self.rng.rand_multinomial(6, 0xf), RAND_LOWER, RAND_UPPER) as i32;
        return ((base as i32) * rand / 256) as i16;
    }

    fn physical_damage_by_player(&mut self, base: i16) -> i16 {
        return if base < 2 {
            self.physical_damage_0or1()
        } else {
            self.physical_damage_normal(base)
        };
    }

    fn physical_damage_by_monster(&mut self, base: i16, atk: i16) -> i16 {
        return if base <= atk / 8 {
            if atk < 16 {
                self.physical_damage_0or1()
            } else {
                self.physical_damage_defensive(atk)
            }
        } else {
            if atk < 8 {
                self.physical_damage_0or1()
            } else {
                self.physical_damage_normal(base)
            }
        };
    }

    pub fn physical_damage(
        &mut self,
        atk: i16,
        def: i16,
        twinhits: bool,
        attacker: Character,
    ) -> i16 {
        let base = atk - (def / 2);
        let damage = match attacker {
            Character::Player => self.physical_damage_by_player(base),
            Character::Monster => self.physical_damage_by_monster(base, atk),
        };
        let coef = if twinhits { 2 } else { 1 };
        return coef * damage;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn damage_twinhits_test() {
        let atk = 250;
        let def = 0;

        let damage_normal = {
            let rng = Rng::default();
            let mut battle_normal = Battle::new(rng);
            battle_normal.physical_damage(atk, def, false, Character::Player)
        };

        let damage_twinhits = {
            let rng = Rng::default();
            let mut battle_twinhits = Battle::new(rng);
            battle_twinhits.physical_damage(atk, def, true, Character::Player)
        };

        assert_eq!(damage_normal * 2, damage_twinhits);
    }
}
