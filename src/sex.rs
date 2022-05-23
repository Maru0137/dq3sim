use enum_iterator::IntoEnumIterator;
use enum_map::Enum;

#[derive(Clone, Copy, Debug, Enum, IntoEnumIterator)]
pub enum Sex {
    Men,
    Women,
}
