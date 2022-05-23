use enum_iterator::IntoEnumIterator;
use enum_map::Enum;
use fixed::types::U8F8;

type StatusValueT = U8F8;

#[derive(Clone, Copy, Debug, IntoEnumIterator, Enum)]
pub enum Status {
    Pow,
    Spd,
    Vit,
    Int,
    Lck,
}
