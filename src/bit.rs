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

#[cfg(test)]
mod tests {
    use super::{mask, extract, range};

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
}
