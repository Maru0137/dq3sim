use crate::attr::Attr;
use crate::loader;
use crate::sex::Sex;

use enum_iterator::IntoEnumIterator;
use enum_map::{Enum, EnumMap};
use fixed::types::U4F4;

type GrowthValueT = U4F4;

#[derive(Clone, Copy, Debug, Display, IntoEnumIterator, Enum, EnumString, PartialEq, Eq)]
pub enum Job {
    #[strum(serialize = "せんし", serialize = "せ")]
    Soldier,
    #[strum(serialize = "そうりょ", serialize = "そ")]
    Pligrim,
    #[strum(serialize = "まほうつかい", serialize = "ま")]
    Wizard,
    #[strum(serialize = "ぶとうか", serialize = "ぶ")]
    Fighter,
    #[strum(serialize = "しょうにん", serialize = "し")]
    Merchant,
    #[strum(serialize = "あそびにん", serialize = "あ")]
    GoofOff,
    #[strum(serialize = "とうぞく", serialize = "と")]
    Thief,
    #[strum(serialize = "ゆうしゃ", serialize = "ゆ")]
    Hero,
    #[strum(serialize = "けんじゃ", serialize = "け")]
    Sage,
}

#[derive(Debug, Default)]
pub struct Growth {
    upper_lv: u8,
    value: GrowthValueT,
}

#[derive(Debug)]
pub struct JobTable {
    name: String,
    exps: [u32; 98],
    attr_inits: EnumMap<Attr, EnumMap<Sex, u8>>,
    attr_growths: EnumMap<Attr, [Growth; 5]>,
}

impl JobTable {
    pub fn growth_value(&self, lv: u8, attr: Attr) -> GrowthValueT {
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
}

impl loader::FromRecord for JobTable {
    fn from_record(record: &csv::StringRecord) -> Self {
        let name = record[0].parse().unwrap();
        let mut exps: [u32; 98] = [0; 98];
        for (i, exp) in exps.iter_mut().enumerate() {
            *exp = record[12 + i].parse().unwrap();
        }

        let mut attr_inits = EnumMap::<Attr, EnumMap<Sex, u8>>::default();
        let mut attr_growths = EnumMap::<Attr, [Growth; 5]>::default();

        for (status_i, status) in Attr::into_enum_iter().enumerate() {
            // Parse initial status.
            let mut inits = EnumMap::<Sex, u8>::default();
            for (sex_i, sex) in Sex::into_enum_iter().enumerate() {
                inits[sex] = record[110 + 12 * status_i + sex_i].parse().unwrap();
            }
            attr_inits[status] = inits;

            // Parse growth of status.
            let mut growths: [Growth; 5] = Default::default();
            for (growth_i, growth) in growths.iter_mut().enumerate() {
                growth.upper_lv = record[112 + 12 * status_i + 2 * growth_i].parse().unwrap();
                growth.value = GrowthValueT::from_bits(
                    record[113 + 12 * status_i + 2 * growth_i].parse().unwrap(),
                );
            }
            attr_growths[status] = growths;
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
    }
}
