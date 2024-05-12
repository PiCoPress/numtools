use crate::int::Integer;

impl From<&[u8]> for Integer {
    fn from(value: &[u8]) -> Self {
        Integer {
            size: value.len(),
            bytes: Box::from(value),
        }
    }
}
impl From<u128> for Integer {
    fn from(value: u128) -> Self {
        Integer {
            size: 16,
            bytes: Box::from((0..16u8).into_iter().map(|x|
                ((value >> (120 - x * 8)) & 255) as u8).collect::<Vec<u8>>()),
        }
    }
}
impl From<u64> for Integer {
    fn from(value: u64) -> Self {
        Integer {
            size: 8,
            bytes: Box::from((0..8u8).map(|x|
                ((value >> (56 - x * 8)) & 255) as u8).collect::<Vec<u8>>()),
        }
    }
}
impl From<u32> for Integer {
    fn from(value: u32) -> Self {
        Integer {
            size: 4,
            bytes: Box::from((0..4u8).map(|x|
                ((value >> (24 - x * 8)) & 255) as u8).collect::<Vec<u8>>()),
        }
    }
}
impl From<u16> for Integer {
    fn from(value: u16) -> Self {
        Integer {
            size: 2,
            bytes: Box::from([(value >> 8) as u8, (value & 255) as u8]),
        }
    }
}
impl From<u8> for Integer {
    fn from(value: u8) -> Self {
        Integer {
            size: 1,
            bytes: Box::from([value]),
        }
    }
}