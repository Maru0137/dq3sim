use crate::attr::{Attr, AttrValue};
use crate::loader;
use crate::sex::Sex;

use enum_iterator::IntoEnumIterator;
use enum_map::{enum_map, Enum, EnumMap};
use fixed::types::U4F4;
use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;

/// Job kind enum
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    Enum,
    EnumString,
    IntoEnumIterator,
    PartialEq,
    Eq,
    Serialize,
)]
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

pub type AttrIncrementValue = U4F4;

#[derive(Clone, Debug, Default)]
pub struct AttrIncrementEntry {
    upper_lv: u8,
    value: AttrIncrementValue,
}

#[derive(Debug)]
pub struct JobEntry {
    name: String,
    exps: [u32; 98],
    attr_inits: EnumMap<Attr, EnumMap<Sex, u8>>,
    attr_increments: EnumMap<Attr, Vec<AttrIncrementEntry>>,
}

impl JobEntry {
    pub fn attr_increment(&self, lv: u8, attr: Attr) -> AttrIncrementValue {
        assert!(lv >= 2);
        assert!(lv <= 99);

        let growths = &self.attr_increments[attr];

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

    fn sum_of_attr_increments(&self, lv: u8, attr: Attr) -> u8 {
        assert!(lv >= 1);
        assert!(lv <= 99);

        if lv == 1 {
            return 0;
        }

        let sum = (2..=lv)
            .fold(AttrValue::from(0), |acc, lv| {
                acc.saturating_add(AttrValue::from(self.attr_increment(lv, attr)))
            })
            .to_num();

        sum
    }

    pub fn initial_attr(&self, attr: Attr, sex: Sex) -> u8 {
        self.attr_inits[attr][sex]
    }

    pub fn standard_attr(&self, lv: u8, attr: Attr) -> u8 {
        assert!(lv >= 1);
        assert!(lv <= 99);

        let init = self.attr_inits[attr][Sex::Man];

        init + self.sum_of_attr_increments(lv, attr)
    }

    pub fn range_attr(&self, lv: u8, attr: Attr) -> RangeInclusive<u8> {
        assert!(lv >= 1);
        assert!(lv <= 99);

        let standard = self.standard_attr(lv, attr);
        let upper = standard.saturating_add(15).saturating_add(lv * 2);

        let lower_factor = match attr {
            Attr::Pow | Attr::Spd => 70,
            Attr::Vit => 60,
            Attr::Int | Attr::Lck => 50,
        };

        let lower = ((standard as u16) * lower_factor / 100) as u8 + 1;

        lower..=upper
    }

    pub fn standard_intelligence_for_learning(&self, lv: u8) -> u8 {
        assert!(lv >= 2);
        assert!(lv <= 99);

        self.sum_of_attr_increments(lv - 1, Attr::Int) + 5
    }

    pub fn intelligence_thresh_for_learning(&self, lv: u8) -> (u8, u8) {
        assert!(lv >= 2);
        assert!(lv <= 99);

        let std = self.standard_intelligence_for_learning(lv);

        (std.saturating_sub(15), std.saturating_add(11))
    }
}

impl loader::FromRecord for JobEntry {
    fn from_record(record: &csv::StringRecord) -> Self {
        let name = record[0].parse().unwrap();
        let mut exps: [u32; 98] = [0; 98];
        for (i, exp) in exps.iter_mut().enumerate() {
            *exp = record[12 + i].parse().unwrap();
        }

        let attr_increment_entry_nums = [5, 5, 6, 5, 5];

        let mut attr_inits = EnumMap::<Attr, EnumMap<Sex, u8>>::default();
        let mut attr_increments = EnumMap::<Attr, Vec<AttrIncrementEntry>>::default();

        for (attr_i, attr) in Attr::into_enum_iter().enumerate() {
            let attr_increment_entry_num_sum =
                attr_increment_entry_nums[0..attr_i].iter().sum::<usize>();
            let offset: usize = (2 * attr_i + 2 * attr_increment_entry_num_sum) as usize;

            // Parse initial status.
            let inits = {
                let mut inits = EnumMap::<Sex, u8>::default();
                for (sex_i, sex) in Sex::into_enum_iter().enumerate() {
                    inits[sex] = record[110 + offset + sex_i].parse().unwrap();
                }
                inits
            };
            attr_inits[attr] = inits;

            // Parse growth of status.
            let increments = {
                let mut increments =
                    vec![AttrIncrementEntry::default(); attr_increment_entry_nums[attr_i]];
                for (increment_i, increment) in increments.iter_mut().enumerate() {
                    increment.upper_lv = record[112 + offset + 2 * increment_i].parse().unwrap();
                    increment.value = AttrIncrementValue::from_bits(
                        record[113 + offset + 2 * increment_i].parse().unwrap(),
                    );
                }
                increments
            };
            attr_increments[attr] = increments;
        }

        Self {
            name,
            exps,
            attr_inits,
            attr_increments,
        }
    }
}

lazy_static! {
    static ref JOB_TABLE: Vec<JobEntry> = {
        let data = include_str!("../assets/0xc4179e_jobs.csv");
        loader::from_csv(data)
    };
}

pub fn get_job_entry(job: Job) -> &'static JobEntry {
    &JOB_TABLE[job as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_attr() {
        assert_eq!(
            get_job_entry(Job::Soldier).initial_attr(Attr::Spd, Sex::Man),
            2
        );
        assert_eq!(
            get_job_entry(Job::Soldier).initial_attr(Attr::Spd, Sex::Women),
            3
        );
        assert_eq!(
            get_job_entry(Job::Soldier).initial_attr(Attr::Vit, Sex::Man),
            9
        );
        assert_eq!(
            get_job_entry(Job::Soldier).initial_attr(Attr::Vit, Sex::Man),
            9
        );
        assert_eq!(
            get_job_entry(Job::Thief).initial_attr(Attr::Vit, Sex::Women),
            5
        );
        assert_eq!(
            get_job_entry(Job::Wizard).initial_attr(Attr::Vit, Sex::Women),
            4
        );
    }

    #[test]
    fn test_attr_increment() {
        assert_eq!(get_job_entry(Job::Soldier).attr_increment(8, Attr::Vit), 2);
        assert_eq!(
            get_job_entry(Job::Soldier).attr_increment(9, Attr::Vit),
            5.5
        );
        assert_eq!(get_job_entry(Job::Soldier).attr_increment(99, Attr::Vit), 1);
        assert_eq!(get_job_entry(Job::Sage).attr_increment(2, Attr::Int), 0.5);
    }
}
