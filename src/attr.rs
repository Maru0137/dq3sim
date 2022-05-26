use enum_iterator::IntoEnumIterator;
use enum_map::Enum;
use fixed::traits::ToFixed;
use fixed::types::U8F8;

#[derive(Clone, Copy, Debug, Display, Enum, EnumString, IntoEnumIterator, PartialEq, Eq)]
pub enum Attr {
    #[strum(serialize = "ちから")]
    Pow,
    #[strum(serialize = "すばやさ")]
    Spd,
    #[strum(serialize = "たいりょく")]
    Vit,
    #[strum(serialize = "かしこさ")]
    Int,
    #[strum(serialize = "うんのよさ")]
    Lck,
}

pub type AttrValue = U8F8;
