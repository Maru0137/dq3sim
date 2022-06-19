use enum_iterator::IntoEnumIterator;
use enum_map::Enum;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Enum, EnumString, IntoEnumIterator, PartialEq, Eq, Deserialize, Serialize,
)]
pub enum Sex {
    #[strum(serialize = "おとこ")]
    Man,
    #[strum(serialize = "おんな")]
    Women,
}
