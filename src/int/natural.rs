use std::fmt::{Display, Formatter};
use super::bcd::*;
use super::Integer;
use super::MUint;

// Use Big Endian, Only Unsigned for now, and allows overflow
impl MUint for Integer {

    /**
    Create new object that has N bytes, N is the first parameter.

    # Examples
    ```
    # use crate::numtools::int::natural::MUint;
    # use crate::numtools::int::Integer;
    let mut i = Integer::new(256);
    ```
     */
    fn new(size: usize) -> Integer {
        if size == 0 { panic!("Not zero"); }
        let mut dat: Vec<u8> = Vec::new();
        dat.resize(size, 0);
        let dat = dat.into_boxed_slice();

        Integer {
            size,
            bytes: dat,
        }
    }

    fn from(t: &[u8]) -> Integer {
        Integer {
            size: t.len(),
            bytes: Box::from(t),
        }
    }

    /**
    Return this size

    # Examples
    ```
    # use crate::numtools::int::Integer;
    # use crate::numtools::int::natural::MUint;
    # fn test() {
    let mut i = Integer::new(1234);

    assert_eq!(1234, i.size());
    # }
    ```
     */
    fn size(&self) -> usize {
        self.size
    }

    /**
    Edit a byte at the offset

    # Panic
    It panics if offset is larger than size.

    # Examples
    ```
    # use crate::numtools::int::natural::MUint;
    # use crate::numtools::int::Integer;
    let mut i = MUint::new(256);
    ```
    */
    fn write_at(&mut self, offset: usize, insert_byte: u8) {
        if self.size <= offset {
            panic!("Invalid offset: {offset}");
        }
        self.bytes[offset] = insert_byte;
    }

    /**
    Initialize all of bytes with `insert_byte`

    # Examples
    ```
    # use crate::numtools::int::natural::MUint;
    # use crate::numtools::int::Integer;
      let mut i = MUint::new(256);
    ```
     */
    fn initialize(&mut self, insert_byte: u8) {
        for i in 0..self.size {
            self.bytes[i] = insert_byte;
        }
    }

    fn clone(&self) -> Integer {
        Integer {
            size: self.size(),
            bytes: self.bytes.clone(),
        }
    }

    /**
    Assign Integer value from string
    # Examples
    ```
    # use crate::numtools::int::natural::MUint;
    # use crate::numtools::int::Integer;
    let mut i = MUint::new(13);
    i.assign("1234567891");
    ```
    */
    fn assign(&mut self, s :&str) {
        self.bytes =  revcon(s, self.size);
    }

    /**
    Return string bits

    # Examples
    ```
    # use crate::numtools::int::natural::MUint;
    # use crate::numtools::int::Integer;
    # fn test() {
    let mut i = MUint::new(2);

    assert_eq!(&*i.bits(), "00000000 00000000");
    # }
    ````
     */
    fn bits(&self) -> String {
        let mut ss = String::new();
        for i in self.bytes.iter() {
            ss.push_str(&*format!("{:08b} ", i));
        }
        ss.pop(); // remove space of the end
        ss
    }

    /**
     Return string hexadecimal

     # Examples
     ```
    # use crate::numtools::int::natural::MUint;
    # use crate::numtools::int::Integer;
    # fn test() {
    let mut i = MUint::new(2);
    assert_eq!(i.hex(), "0x00 0x00");
    # }
     ```
     */
    fn hex(&self) -> String {
        let mut h = String::new();
        for i in self.bytes.iter() {
            h.push_str(&*format!("0x{:02x} ", i));
        }
        h.pop(); // remove space of the end
        h
    }

    /**
    Perform Rhs += Lhs
    Return true if it was overflow, else false

    # Panic
     It panics when Rhs\.size < Lhs\.size

    # Examples
    ```
    # use crate::numtools::int::natural::MUint;
    # use crate::numtools::int::Integer;
    # fn test() {
    let mut int1 = MUint::new(128);
    let mut int2 =  MUint::new(64);
    int1.assign("1234");
    int2.assign("12345678909876543210");
    int1.add(&int2);

    assert_eq!("12345678909876544444", int1);
    # }
    ```
     */
    fn add(&mut self, other: &Integer) -> bool {
        if other.size > self.size {
            panic!("Incompatible size: {} < {} \n Please consider swap the operation.", self.size, other.size);
        }

        let mut carry = false;
        for (count, i) in other.bytes.iter().enumerate().rev() {
            let index = self.size - other.size + count;
            if carry {
                (self.bytes[index], carry) = self.bytes[index].overflowing_add(1);
            }

            let b2 = self.bytes[index].overflowing_add(*i);
            self.bytes[index] = b2.0;
            if !carry { carry = b2.1; }
        }
        carry
    }

    /**
    Perform Rhs += Lhs
    Return true if it was overflow, else false

    # Panic
    It panics when Rhs\.size < Lhs\.size

    # Examples
    ```
    # use crate::numtools::int::natural::MUint;
    # fn test() {
    let mut int1 = MUint::new(3);
    let mut int2 = MUint::new(1);
    int1.assign("8080");
    int2.assign("127");
    int1.subtract(&int2);

    assert_eq!("7953", &*int1.to_string());
    # }
    ```
     */
    fn subtract(&mut self, other: &Integer) -> bool {
        if other.size > self.size {
            panic!("Incompatible size: {} < {} \n Please consider swap the operation.", self.size, other.size);
        }

        let mut carry = false;
        for (count, i) in other.bytes.iter().enumerate().rev() {
            let index = self.size - other.size + count;
            if carry {
                (self.bytes[index], carry) = self.bytes[index].overflowing_sub(1);
            }

            let b2 = self.bytes[index].overflowing_sub(*i);
            self.bytes[index] = b2.0;
            if !carry { carry = b2.1; }
        }
        carry
    }

    fn multiply(&mut self, other: &Integer) -> bool {
        todo!()
    }

    fn divide(&mut self, other: Integer) -> (Integer, bool) {
        todo!()
    }

    fn shr(&mut self, other: Integer) -> bool {
        todo!()
    }

    fn shl(&mut self, other: Integer) -> bool {
        todo!()
    }

    fn add_test(&self, other: &Integer) -> (Integer, bool) {
        todo!()
    }

    fn subtract_test(&self, other: &Integer) -> (Integer, bool) {
        todo!()
    }

    fn multiply_test(&self, other: &Integer) -> (Integer, bool) {
        todo!()
    }

    fn divide_test(&self, other: &Integer) -> (Integer, Integer, bool) {
        todo!()
    }

    fn shr_test(&mut self, other: Integer) -> (Integer, bool) {
        todo!()
    }

    fn shl_test(&mut self, other: Integer) -> (Integer, bool) {
        todo!()
    }
}


