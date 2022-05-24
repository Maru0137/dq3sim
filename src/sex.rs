use enum_iterator::IntoEnumIterator;
use enum_map::Enum;

#[derive(Clone, Copy, Debug, Enum, EnumString, IntoEnumIterator, PartialEq, Eq)]
pub enum Sex {
    #[strum(serialize = "おとこ")]
    Man,
    #[strum(serialize = "おんな")]
    Women,
}
