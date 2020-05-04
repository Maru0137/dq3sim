use num;

fn mask<T: num::PrimInt + num::Unsigned>(size: usize) -> T {
    let bitwidth = 8 as usize * std::mem::size_of::<T>();
    if size >= bitwidth {
        return !T::zero();
    }

    return (T::one() << size) - T::one();
}

pub fn extract<T: num::PrimInt + num::Unsigned>(v: T, begin: usize, end: usize) -> T {
    let mask = mask::<T>(end) ^ mask::<T>(begin);
    return v & mask;
}

pub fn range<T: num::PrimInt + num::Unsigned>(v: T, begin: usize, end: usize) -> T {
    return extract(v, begin, end) >> begin;
}

pub fn is_powerof2<T: num::PrimInt + num::Unsigned>(v: T) -> bool {
    if v == T::zero() {
        return false;
    }

    return (v & (v - T::one())) == T::zero();
}

#[cfg(test)]
mod tests {
    use super::{extract, is_powerof2, mask, range};

    #[test]
    fn mask_test() {
        assert_eq!(mask::<u32>(0), 0);
        assert_eq!(mask::<u32>(1), 1);
        assert_eq!(mask::<u32>(8), 0xff);
        assert_eq!(mask::<u32>(31), 0x7fffffff);
        assert_eq!(mask::<u32>(32), 0xffffffff);
    }

    #[test]
    fn extract_test() {
        assert_eq!(extract(0x12345678 as u32, 0, 16), 0x00005678);
        assert_eq!(extract(0x12345678 as u32, 16, 32), 0x12340000);
    }

    #[test]
    fn range_test() {
        assert_eq!(range(0x12345678 as u32, 0, 16), 0x5678);
        assert_eq!(range(0x12345678 as u32, 16, 32), 0x1234);
    }

    #[test]
    fn is_powerof2_test() {
        assert_eq!(is_powerof2(0 as u8), false);
        assert_eq!(is_powerof2(1 as u8), true);
        assert_eq!(is_powerof2(2 as u8), true);
        assert_eq!(is_powerof2(3 as u8), false);
        assert_eq!(is_powerof2(u8::max_value()), false);
    }
}
