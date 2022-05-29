use crate::attr::{Attr, AttrValue};
use crate::loader;
use crate::sex::Sex;

use enum_iterator::IntoEnumIterator;
use enum_map::{enum_map, Enum, EnumMap};
use fixed::types::U4F4;
use std::ops::RangeInclusive;

pub type GrowthValueT = U4F4;

/// Job kind enum
#[derive(Clone, Copy, Debug, Display, IntoEnumIterator, Enum, EnumString, PartialEq, Eq)]
pub enum Job {
    #[strum(serialize = "せんし", serialize = "せ")]
    Soldier,
    #[strum(serialize = "ぶとうか", serialize = "ぶ")]
    Fighter,
    #[strum(serialize = "まほうつかい", serialize = "ま")]
    Wizard,
    #[strum(serialize = "そうりょ", serialize = "そ")]
    Pligrim,
    #[strum(serialize = "しょうにん", serialize = "し")]
    Merchant,
    #[strum(serialize = "あそびにん", serialize = "あ")]
    GoofOff,
    #[strum(serialize = "とうぞく", serialize = "と")]
    Thief,
    #[strum(serialize = "けんじゃ", serialize = "け")]
    Sage,
    #[strum(serialize = "ゆうしゃ", serialize = "ゆ")]
    Hero,
}

#[derive(Clone, Debug, Default)]
pub struct GrowthEntry {
    upper_lv: u8,
    value: GrowthValueT,
}

#[derive(Debug)]
pub struct JobTable {
    name: String,
    exps: [u32; 98],
    attr_inits: EnumMap<Attr, EnumMap<Sex, u8>>,
    attr_growths: EnumMap<Attr, Vec<GrowthEntry>>,
}

impl JobTable {
    pub fn growth_value(&self, lv: u8, attr: Attr) -> GrowthValueT {
        assert!(lv >= 2);

        let growths = &self.attr_growths[attr];

        // Search growth step id.
        let mut id = 0;
        for (i, growth) in growths.iter().enumerate() {
            id = i;
            if lv < growth.upper_lv {
                break;
            }
        }

        growths[id].value
    }

    fn sum_growth_value(&self, lv: u8, attr: Attr) -> u8 {
        assert!(lv >= 1);

        if lv == 1 {
            return 0;
        }

        let sum = (2..=lv)
            .fold(AttrValue::from(0), |acc, lv| {
                acc.saturating_add(AttrValue::from(self.growth_value(lv, attr)))
            })
            .to_num();

        sum
    }

    pub fn standard_attr_value(&self, lv: u8, attr: Attr) -> u8 {
        assert!(lv >= 1);

        let init = self.attr_inits[attr][Sex::Man];

        init + self.sum_growth_value(lv, attr)
    }

    pub fn range_attr_value(&self, lv: u8, attr: Attr) -> RangeInclusive<u8> {
        assert!(lv >= 1);

        let standard = self.standard_attr_value(lv, attr);
        let upper = standard.saturating_add(15).saturating_add(lv * 2);

        let lower_factor = match attr {
            Attr::Pow | Attr::Spd => 70,
            Attr::Vit => 60,
            Attr::Int | Attr::Lck => 50,
        };

        let lower = ((standard as u16) * lower_factor / 100) as u8 + 1;

        lower..=upper
    }

    pub fn standard_intelligence_by_learning(&self, lv: u8) -> u8 {
        assert!(lv >= 2);

        self.sum_growth_value(lv - 1, Attr::Int) + 5
    }

    pub fn intelligence_thresh_for_learning(&self, lv: u8) -> (u8, u8) {
        assert!(lv >= 2);

        let std = self.standard_intelligence_by_learning(lv);

        (std.saturating_sub(15), std.saturating_add(11))
    }
}

impl loader::FromRecord for JobTable {
    fn from_record(record: &csv::StringRecord) -> Self {
        let name = record[0].parse().unwrap();
        let mut exps: [u32; 98] = [0; 98];
        for (i, exp) in exps.iter_mut().enumerate() {
            *exp = record[12 + i].parse().unwrap();
        }

        let mut attr_inits = EnumMap::<Attr, EnumMap<Sex, u8>>::default();
        let mut attr_growths = EnumMap::<Attr, Vec<GrowthEntry>>::default();

        let attr_growth_nums = [5, 5, 6, 5, 5];

        for (attr_i, attr) in Attr::into_enum_iter().enumerate() {
            let growth_num_sum = attr_growth_nums[0..attr_i].iter().sum::<usize>();
            let offset: usize = 2 * attr_i + 2 * growth_num_sum;

            // Parse initial status.
            let mut inits = EnumMap::<Sex, u8>::default();
            for (sex_i, sex) in Sex::into_enum_iter().enumerate() {
                inits[sex] = record[110 + offset + sex_i].parse().unwrap();
            }
            attr_inits[attr] = inits;

            // Parse growth of status.
            let mut growths = vec![GrowthEntry::default(); attr_growth_nums[attr_i]];
            for (growth_i, growth) in growths.iter_mut().enumerate() {
                growth.upper_lv = record[112 + offset + 2 * growth_i].parse().unwrap();
                growth.value =
                    GrowthValueT::from_bits(record[113 + offset + 2 * growth_i].parse().unwrap());
            }
            attr_growths[attr] = growths;
        }

        Self {
            name,
            exps,
            attr_inits,
            attr_growths,
        }
    }
}

lazy_static! {
    static ref JOB_TABLE: Vec<JobTable> = {
        let data = include_str!("../assets/0xc4179e_jobs.csv");
        loader::from_csv(data)
    };
}

pub fn get_job_table(job: Job) -> &'static JobTable {
    &JOB_TABLE[job as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_growth_value() {
        assert_eq!(get_job_table(Job::Soldier).growth_value(9, Attr::Vit), 5.5);
        assert_eq!(get_job_table(Job::Sage).growth_value(2, Attr::Int), 0.5);
    }
}
