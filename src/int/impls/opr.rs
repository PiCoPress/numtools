use std::ops::Add;

use crate::int;
use crate::int::Integer;

impl Add for Integer {
    type Output = Integer;
    fn add(mut self, rhs: Self) -> Self::Output {
        let _ = int::MUint::muint_add(&mut self, &rhs);
        self
    }
}
impl Add<u8> for Integer {
    type Output = Integer;
    fn add(self, rhs: u8) -> Self::Output {
        let cpy = self.clone();
        let _ = cpy.add(Integer::from(rhs));
        self
    }
}

impl Clone for Integer {
    fn clone(&self) -> Self {
        Integer {
            size: self.size,
            bytes: self.bytes.clone(),
        }
    }
}