use enum_iterator::IntoEnumIterator;
use enum_map::Enum;
use fixed::{types::extra::U8, FixedU16};

type StatusValue = FixedU16<U8>;

#[derive(Clone, Copy, Debug, IntoEnumIterator, Enum)]
pub enum Status {
    Pow,
    Spd,
    Vit,
    Int,
    Lck,
}
