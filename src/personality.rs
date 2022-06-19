use crate::attr::Attr;
use crate::loader;

use enum_iterator::IntoEnumIterator;
use enum_map::{enum_map, Enum, EnumMap};
use fixed::types::U1F7;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub type GrowthFactor = U1F7;

#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    Enum,
    EnumString,
    IntoEnumIterator,
    PartialEq,
    Eq,
    Deserialize,
    Serialize,
)]
pub enum Personality {
    #[strum(serialize = "ちからじまん")]
    Jock,
    #[strum(serialize = "ごうけつ")]
    Valiant,
    #[strum(serialize = "らんぼうもの")]
    Bully,
    #[strum(serialize = "おとこまさり")]
    Amazon,
    #[strum(serialize = "おおぐらい")]
    Unique,
    #[strum(serialize = "ねっけつかん")]
    Macho,
    #[strum(serialize = "おてんば")]
    Tomboy,
    #[strum(serialize = "いのちしらず")]
    Fearless,
    #[strum(serialize = "むっつりスケベ")]
    Lewd,
    #[strum(serialize = "おっちょこちょい")]
    Foolish,
    #[strum(serialize = "まけずぎらい")]
    Deflant,
    #[strum(serialize = "のんきもの")]
    Carefree,
    #[strum(serialize = "せけんしらず")]
    Naive,
    #[strum(serialize = "がんこもの")]
    Stubborn,
    #[strum(serialize = "おじょうさま")]
    Ladylike,
    #[strum(serialize = "しょうじきもの")]
    Honest,
    #[strum(serialize = "ふつう")]
    Ordinary,
    #[strum(serialize = "でんこうせっか")]
    Quick,
    #[strum(serialize = "すばしっこい")]
    Agile,
    #[strum(serialize = "ぬけめがない")]
    Alert,
    #[strum(serialize = "きれもの")]
    Sharp,
    #[strum(serialize = "ひねくれもの")]
    Twisted,
    #[strum(serialize = "みえっぱり")]
    Vain,
    #[strum(serialize = "わふぁまま")]
    Selfish,
    #[strum(serialize = "なきむし")]
    Weepy,
    #[strum(serialize = "さびしがりや")]
    Lonesome,
    #[strum(serialize = "タフガイ")]
    Tough,
    #[strum(serialize = "てつじん")]
    Ironman,
    #[strum(serialize = "へこたれない")]
    Hekotarenai,
    #[strum(serialize = "くろうにん")]
    Diligent,
    #[strum(serialize = "がんばりや")]
    Eager,
    #[strum(serialize = "おせっかい")]
    Meddler,
    #[strum(serialize = "なまけもの")]
    Lazy,
    #[strum(serialize = "ずのうめいせき")]
    Smart,
    #[strum(serialize = "あたまでっかち")]
    Logical,
    #[strum(serialize = "ロマンチック")]
    Romantic,
    #[strum(serialize = "あまえんぼう")]
    Helpless,
    #[strum(serialize = "ラッキーマン")]
    Lucky,
    #[strum(serialize = "しあわせもの")]
    Happy,
    #[strum(serialize = "うっかりもの")]
    Careless,
    #[strum(serialize = "いっぴきおおかみ")]
    Solitary,
    #[strum(serialize = "いくじなし")]
    Cowardly,
    #[strum(serialize = "おちょうしもの")]
    Silly,
    #[strum(serialize = "ひっこみじあん")]
    Timid,
    #[strum(serialize = "やさしいひと")]
    Kindly,
    #[strum(serialize = "セクシーギャル")]
    Sexy,
}

pub struct PersonalityTable {
    name: String,
    growth_factors: EnumMap<Attr, GrowthFactor>,
}

impl PersonalityTable {
    pub fn growth_factor(&self, attr: Attr) -> GrowthFactor {
        self.growth_factors[attr]
    }
}

impl loader::FromRecord for PersonalityTable {
    fn from_record(record: &csv::StringRecord) -> Self {
        let name = record[0].parse().unwrap();

        let mut growth_factors = EnumMap::<Attr, GrowthFactor>::default();
        for (i, status) in Attr::into_enum_iter().enumerate() {
            growth_factors[status] = GrowthFactor::from_bits(record[1 + i].parse().unwrap());
        }

        Self {
            name,
            growth_factors,
        }
    }
}

lazy_static! {
    static ref PERSONALITY_TABLE: Vec<PersonalityTable> = {
        let data = include_str!("../assets/0xc424bc_personalities.csv");
        loader::from_csv(data)
    };
}

pub fn get_personality_table(personality: Personality) -> &'static PersonalityTable {
    &PERSONALITY_TABLE[personality as usize + 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_personality_string() {
        assert_eq!(Personality::Valiant.to_string(), "ごうけつ");
        assert_eq!(
            Personality::from_str("タフガイ").unwrap(),
            Personality::Tough
        );
    }

    #[test]
    fn test_growth_factor() {
        assert_eq!(
            get_personality_table(Personality::Valiant)
                .growth_factor(Attr::Pow)
                .to_bits(),
            179
        );
    }
}
