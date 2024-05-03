use std::fmt::{Display, Formatter};
use super::bcd::encode;
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
    pub fn write_all(&mut self, insert_byte: u8) {
        for i in 0..self.size {
            self.bytes[i] = insert_byte;
        }
    }
    pub fn show_as_bits(&self) -> String {
        let mut ss = String::new();
        for i in self.bytes.iter() {
            ss.push_str(&*format!("{:08b} ", i));
        }
        ss.pop(); // remove bottom of space
        ss
    }
    pub fn add(&mut self, other: &Integer) {
        if other.size > self.size {
            panic!("Incompatible size: {} {} \n Please Consider swap the operation.", self.size, other.size);
        }
        let mut carry = false;
        for (count, i) in other.bytes.iter().enumerate().rev() {
            if carry {
                self.bytes[count] += 1;
                carry = false;
            }
            let b2: u16 = (self.bytes[count] as u16) + (*i as u16);
            self.bytes[count] += *i;
            if b2 > 255 {
                carry = true;
            }
        }
    }
    pub fn hex(&self) -> String {
        let mut vass = String::new();
        for i in self.bytes.iter() {
            vass.push_str(&*format!("0x{:02x} ", i));
        }
        vass
    }
}
fn u8_to_str(v: u8) -> &'static str {
    match v {
        1 => "1",
        2 => "2",
        3 => "3",
        4 => "4",
        5 => "5",
        6 => "6",
        7 => "7",
        8 => "8",
        9 => "9",
        0 => "0",
        _ => "?"
    }
}
impl Display for Integer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &*encode(&self.bytes.to_vec()))
    }
}