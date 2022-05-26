use crate::attr::{Attr, AttrValue};
use crate::job::{get_job_table, Job};
use crate::personality::{get_personality_table, Personality};
use crate::rand;
use crate::sex::Sex;

use enum_iterator::IntoEnumIterator;
use enum_map::{enum_map, Enum, EnumMap};
use fixed::traits::ToFixed;

pub type Attrs = EnumMap<Attr, AttrValue>;

#[derive(Debug, Clone)]
pub struct Player {
    pub lv: u8,
    // exp: u32,
    pub max_hp: u16,
    // hp: u16,
    pub max_mp: u16,
    // mp: u16,
    pub attrs: Attrs,
    // name: String,
    // is_poisoned: bool,
    // is_paralyzed: bool,
    // is_cursed: bool,
    // is_dying: bool,
    // is_dead: bool,
    pub sex: Sex,
    pub personality: Personality,
    pub job: Job,
}

impl Player {
    pub fn level(&self) -> u8 {
        self.lv
    }

    pub fn attr(&self, attr: Attr) -> u8 {
        self.attrs[attr].to_num()
    }

    pub fn sex(&self) -> Sex {
        self.sex
    }

    pub fn personality(&self) -> Personality {
        self.personality
    }

    pub fn job(&self) -> Job {
        self.job
    }

    fn growth_attr(&self, lv: u8, attr: Attr) -> AttrValue {
        let growth_base = get_job_table(self.job).growth_value(lv, attr);

        let mut rng = rand::thread_rng();

        let range = get_job_table(self.job()).range_attr_value(self.lv, attr);
        let upper = range.max().unwrap();
        if self.attr(attr) > upper {
            return (rng.rand() % 2).into();
        }

        let randomized =
            ((growth_base.to_bits() as u16) * (rng.rand_multinomial(136, 31) as u16) >> 3) & 0x0ff0;

        let factor: AttrValue = get_personality_table(self.personality)
            .growth_factor(attr)
            .into();
        AttrValue::from_bits(randomized) * factor
    }

    pub fn levelup(&mut self) {
        self.lv += 1;

        for attr in Attr::into_enum_iter() {
            let before = self.attrs[attr];
            let range = get_job_table(self.job()).range_attr_value(self.lv, attr);

            let mut after = before + self.growth_attr(self.lv, attr);

            let lower = range.min().unwrap().to_fixed();

            if after < lower {
                after = lower
            }

            self.attrs[attr] = after;
        }

        // TODO: growth for HP/MP correctly
        self.max_hp = self.attrs[Attr::Vit].to_num::<u16>() * 2;

        if !(self.job() == Job::Soldier || self.job() == Job::Fighter) {
            self.max_mp = self.attrs[Attr::Int].to_num::<u16>() * 2;
        }
    }

    pub fn job_change(&mut self, job: Job) {
        self.lv = 1;
        self.job = job;

        self.attrs
            .values_mut()
            .for_each(|x| *x = *x / AttrValue::from(2));
    }
}

#[derive(Debug)]
pub struct PlayerInit {
    pub lv: u8,
    pub max_hp: u16,
    pub max_mp: u16,
    pub pow: u8,
    pub spd: u8,
    pub vit: u8,
    pub int: u8,
    pub lck: u8,
    pub sex: Sex,
    pub personality: Personality,
    pub job: Job,
}

impl Default for PlayerInit {
    fn default() -> Self {
        Self {
            lv: 1,
            max_hp: 1,
            max_mp: 0,
            pow: 0,
            spd: 0,
            vit: 0,
            int: 0,
            lck: 0,
            sex: Sex::Man,
            personality: Personality::Ordinary,
            job: Job::Soldier,
        }
    }
}

impl PlayerInit {
    pub fn init(self) -> Player {
        let attrs: Attrs = enum_map! {
            Attr::Pow => self.pow.to_fixed(),
            Attr::Spd => self.spd.to_fixed(),
            Attr::Vit => self.vit.to_fixed(),
            Attr::Int => self.int.to_fixed(),
            Attr::Lck => self.lck.to_fixed(),
        };

        Player {
            lv: self.lv,
            max_hp: self.max_hp,
            max_mp: self.max_mp,
            attrs: attrs,
            sex: self.sex,
            personality: self.personality,
            job: self.job,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let player = PlayerInit {
            lv: 3,
            max_hp: 50,
            vit: 25,
            ..Default::default()
        }
        .init();

        assert_eq!(player.lv, 3);
        assert_eq!(player.max_hp, 50);
        assert_eq!(player.max_mp, 0);
        assert_eq!(player.attrs[Attr::Pow], 0u8.to_fixed::<AttrValue>());
        assert_eq!(player.attrs[Attr::Spd], 0u8.to_fixed::<AttrValue>());
        assert_eq!(player.attrs[Attr::Vit], 25u8.to_fixed::<AttrValue>());
        assert_eq!(player.attrs[Attr::Int], 0u8.to_fixed::<AttrValue>());
        assert_eq!(player.attrs[Attr::Lck], 0u8.to_fixed::<AttrValue>());
        assert_eq!(player.sex, Sex::Man);
        assert_eq!(player.personality, Personality::Ordinary);
        assert_eq!(player.job, Job::Soldier);
    }
}
