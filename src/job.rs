use crate::loader;
use crate::sex::Sex;
use crate::status::Status;

use enum_iterator::IntoEnumIterator;
use enum_map::EnumMap;
use fixed::types::U4F4;

type GrowthValueT = U4F4;

#[derive(Debug, Default)]
pub struct Growth {
    upper_lv: u8,
    value: GrowthValueT,
}

#[derive(Debug)]
pub struct Job {
    name: String,
    exps: [u32; 98],
    status_inits: EnumMap<Status, EnumMap<Sex, u8>>,
    status_growths: EnumMap<Status, [Growth; 5]>,
}

impl Job {
    pub fn growth_value(&self, lv: u8, status: Status) -> GrowthValueT {
        let growths = &self.status_growths[status];

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

impl loader::FromRecord for Job {
    fn from_record(record: &csv::StringRecord) -> Self {
        let name = record[0].parse().unwrap();
        let mut exps: [u32; 98] = [0; 98];
        for (i, exp) in exps.iter_mut().enumerate() {
            *exp = record[12 + i].parse().unwrap();
        }

        let mut status_inits = EnumMap::<Status, EnumMap<Sex, u8>>::default();
        let mut status_growths = EnumMap::<Status, [Growth; 5]>::default();

        for (status_i, status) in Status::into_enum_iter().enumerate() {
            // Parse initial status.
            let mut inits = EnumMap::<Sex, u8>::default();
            for (sex_i, sex) in Sex::into_enum_iter().enumerate() {
                inits[sex] = record[110 + 12 * status_i + sex_i].parse().unwrap();
            }
            status_inits[status] = inits;

            // Parse growth of status.
            let mut growths: [Growth; 5] = Default::default();
            for (growth_i, growth) in growths.iter_mut().enumerate() {
                growth.upper_lv = record[112 + 12 * status_i + 2 * growth_i].parse().unwrap();
                growth.value =
                    U4F4::from_bits(record[113 + 12 * status_i + 2 * growth_i].parse().unwrap());
            }
            status_growths[status] = growths;
        }

        Self {
            name,
            exps,
            status_inits,
            status_growths,
        }
    }
}

lazy_static! {
    pub static ref JOB_TABLE: Vec<Job> = {
        let data = include_str!("../assets/0xc4179e_jobs.csv");
        loader::from_csv(data)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_growth_value() {
        assert_eq!(JOB_TABLE[0].growth_value(9, Status::Vit), 5.5);
    }
}
