use enum_iterator::IntoEnumIterator;
use enum_map::{enum_map, Enum, EnumMap};
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
pub type Attrs = EnumMap<Attr, AttrValue>;

pub fn attrs_new(pow: u8, spd: u8, vit: u8, int: u8, lck: u8) -> Attrs {
    enum_map! {
        Attr::Pow => pow.to_fixed(),
        Attr::Spd => spd.to_fixed(),
        Attr::Vit => vit.to_fixed(),
        Attr::Int => int.to_fixed(),
        Attr::Lck => lck.to_fixed(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_attrs() {
        let attrs = attrs_new(8, 2, 9, 2, 3);

        assert_eq!(attrs[Attr::Pow], 8u8.to_fixed::<AttrValue>());
        assert_eq!(attrs[Attr::Spd], 2u8.to_fixed::<AttrValue>());
        assert_eq!(attrs[Attr::Vit], 9u8.to_fixed::<AttrValue>());
        assert_eq!(attrs[Attr::Int], 2u8.to_fixed::<AttrValue>());
        assert_eq!(attrs[Attr::Lck], 3u8.to_fixed::<AttrValue>());
    }
}
