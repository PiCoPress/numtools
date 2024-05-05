use std::fmt::{Display, Formatter};
use super::bcd::*;
#[derive(Debug)]
pub struct Integer {
    size: usize,
    bytes: Box<[u8]>, // Use Big Endian, Only Unsigned for now
}
impl Integer {
    pub fn new(size: usize) -> Integer {
        let mut dat: Vec<u8> = Vec::new();
        dat.resize(size, 0);
        let dat = dat.into_boxed_slice();
        Integer {
            size,
            bytes: dat,
        }
    }
    pub fn size(&self) -> usize {
        self.size
    }
    pub fn write_at(&mut self, offset: usize, insert_byte: u8) {
        if self.size <= offset {
            panic!("Invalid offset: {offset}");
        }
        self.bytes[offset] = insert_byte;
    }
    pub fn initalize(&mut self, insert_byte: u8) {
        for i in 0..self.size {
            self.bytes[i] = insert_byte;
        }
    }
    pub fn assign(&mut self, s :&str) {
        let tem =  revcon(s, self.size);
        self.bytes = tem;
    }
    pub fn bit(&self) -> String {
        let mut ss = String::new();
        for i in self.bytes.iter() {
            ss.push_str(&*format!("{:08b} ", i));
        }
        ss.pop(); // remove space of the end
        ss
    }
    pub fn add(&mut self, other: &Integer) {
        if other.size > self.size {
            panic!("Incompatible size: {} < {} \n Please consider swap the operation.", self.size, other.size);
        }
        let mut carry = false;
        for (count, i) in other.bytes.iter().enumerate().rev() {
            if carry {
                self.bytes[count] += 1;
            }
            let b2 = self.bytes[count].overflowing_add(*i);
            self.bytes[count] = b2.0;
            carry = b2.1;
        }
    }
    pub fn hex(&self) -> String {
        let mut vass = String::new();
        for i in self.bytes.iter() {
            vass.push_str(&*format!("0x{:02x} ", i));
        }
        vass.pop(); // remove space of the end
        vass
    }
}
impl Display for Integer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &*conv(&self.bytes.to_vec()))
    }
}