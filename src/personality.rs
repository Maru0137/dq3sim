use crate::loader;
use crate::status::Status;

use enum_iterator::IntoEnumIterator;
use enum_map::{enum_map, Enum, EnumMap};
use fixed::types::U1F7;

type GrowthFactorT = U1F7;

pub struct Personality {
    name: String,
    growth_factors: EnumMap<Status, GrowthFactorT>,
}

impl Personality {
    pub fn growth_factor(&self, status: Status) -> GrowthFactorT {
        self.growth_factors[status]
    }
}

impl loader::FromRecord for Personality {
    fn from_record(record: &csv::StringRecord) -> Self {
        let name = record[0].parse().unwrap();

        let mut growth_factors = EnumMap::<Status, GrowthFactorT>::default();
        for (i, status) in Status::into_enum_iter().enumerate() {
            growth_factors[status] = GrowthFactorT::from_bits(record[1 + i].parse().unwrap());
        }

        Self {
            name,
            growth_factors,
        }
    }
}

lazy_static! {
    pub static ref PERSONALITY_TABLE: Vec<Personality> = {
        let data = include_str!("../assets/0xc424bc_personalities.csv");
        loader::from_csv(data)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_growth_factor() {
        assert_eq!(
            PERSONALITY_TABLE[2].growth_factor(Status::Pow).to_bits(),
            179
        );
    }
}
