use crate::int::Integer;
use crate::int::util::bcd::*;

impl PartialEq<&str> for Integer {
    fn eq(&self, other: &&str) -> bool {
        let s = conv(&mut self.bytes.to_vec());

        return &s == other
    }
}

impl PartialEq<u64> for Integer {
    fn eq(&self, other: &u64) -> bool {
        if self.size != 8 {
            return false;
        }
        if self.bytes[0..8].as_ptr() as u64 == *other {
            return true;
        }
        false
    }
}

impl PartialEq<Integer> for &str {
    fn eq(&self, other: &Integer) -> bool {
        other == self
    }
}

impl PartialEq for Integer {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }
        for t in self.bytes.iter().enumerate() {
            if *t.1 != other.bytes[t.0] {
                return false;
            }
        }
        true
    }
}