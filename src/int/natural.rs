use super::util::bcd::*;
use super::Integer;
use super::MUint;

// Use Big Endian, Only Unsigned for now, and allows overflow
impl MUint for Integer {

    /**
    Return this size

    # Examples
    ```
    # use crate::numtools::int::*;
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
    Check whether `Rhs` is equal to `Lhs`, whatever each size is different. <br>
    If so, return true, else false.
     */
    fn equal(&self, rhs: &Integer) -> bool {
        let less = match self.size - rhs.size {
            0 => { rhs.size },
            _ => { self.size }
        };
        for i in 0..less {
            if self.bytes[self.size - i - 1] != rhs.bytes[rhs.size - i - 1] {
                return false;
            }
        }
        true
    }

    /**
    Edit a byte at the offset

    # Panic
    It panics if offset is larger than size.

    # Examples
    ```
    # use crate::numtools::int::*;
    let mut i = Integer::new(256);
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
    # use crate::numtools::int::*;
      let mut i = Integer::new(256);
    ```
     */
    fn initialize(&mut self, insert_byte: u8) {
        for i in 0..self.size {
            self.bytes[i] = insert_byte;
        }
    }

    /**
    Assign Integer value from string
    # Examples
    ```
    # use crate::numtools::int::*;
    let mut i = Integer::new(13);
    i.assign("1234567891");
    ```
    */
    fn assign(&mut self, s :&str) {
        let s = s.trim_start_matches("0");
        revcon(s, &mut self.bytes);
    }

    /**
    Return string bits

    # Examples
    ```
    # use crate::numtools::int::*;
    # fn test() {
    let mut i = Integer::new(2);

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
     Return hexadecimal string

     # Examples
     ```
    # use crate::numtools::int::*;
    # fn test() {
    let mut i = Integer::new(2);
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
    Perform Rhs += Lhs <br>
    Return true if it was overflow, else false

    # Panic
     It panics when Rhs\.size > Lhs\.size

    # Examples
    ```
    # use crate::numtools::int::*;
    # fn test() {
    let mut int1 = Integer::new(128);
    let mut int2 =  Integer::new(64);
    int1.assign("1234");
    int2.assign("12345678909876543210");
    int1.muint_add(&int2);

    assert_eq!("12345678909876544444", int1);
    # }
    ```
     */
    fn muint_add(&mut self, other: &Integer) -> bool {
        if other.size > self.size {
            panic!("Incompatible size: {} < {} \n Please consider swap the operation.", self.size, other.size);
        }

        let mut carry = false;
        for index in (0..self.size).rev() {
            if carry {
                (self.bytes[index], carry) = self.bytes[index].overflowing_add(1);
            }
            let val = match other.size as i128 - (self.size - index) as i128 {
                i@ 0.. => other.bytes[i as usize],
                _ => 0,
            };
            let b2 = self.bytes[index].overflowing_add(val);
            self.bytes[index] = b2.0;
            if !carry { carry = b2.1; }
        }
        carry
    }

    /**
    Perform Rhs += Lhs
    Return true if it was overflow, else false

    # Panic
    It panics when Rhs\.size > Lhs\.size

    # Examples
    ```
    # use crate::numtools::int::*;
    # fn test() {
    let mut int1 = Integer::new(3);
    let mut int2 = Integer::new(1);
    int1.assign("8080");
    int2.assign("127");
    int1.subtract(&int2);

    assert_eq!("7953", &*int1.to_string());
    # }
    ```
     */
    fn subtract(&mut self, other: &Integer) -> bool {
        if other.size > self.size {
            panic!("Incompatible size: {} < {} \n Please consider swap the operation.",
                   self.size, other.size);
        }

        let mut carry = false;
        for index in (0..self.size).rev() {
            if carry {
                (self.bytes[index], carry) = self.bytes[index].overflowing_sub(1);
            }
            let val = match other.size as i128 - (self.size - index) as i128 {
                 i@ 0.. => other.bytes[i as usize],
                _ => 0,
            };
            let b2 = self.bytes[index].overflowing_sub(val);
            self.bytes[index] = b2.0;
            if !carry { carry = b2.1; }
        }
        carry
    }

    fn multiply(&mut self, other: &Integer) -> bool {
        todo!()
    }

    fn divide(&mut self, other: &Integer) -> (Integer, bool) {
        todo!()
    }

    /**
    Left shift `count` times. <br>
    It returns true if overflow, else false.
     */
    fn shr(&mut self, count: u64) -> bool {
        if count >= (8 * self.size) as u64 {
            self.bytes = Vec::with_capacity(self.size).into_boxed_slice();
            return true;
        }

        let mut i = 0;
        for &k in self.bytes.iter() {
            if k != 0 { break; }
            i += 1;
        }

        for _ in 0..count {
            let mut carry = false;

            // Check the following loop is run
            let mut chk = true;
            for idx in (i..self.size).rev() {
                chk = false;
                let ii = self.bytes[idx] % 2;
                self.bytes[idx] >>= 1;

                if carry { self.bytes[idx] |= 128; }
                carry = ii == 1;
            }
            if chk { return chk; }
            if self.bytes[i] == 0 { i += 1; }
        }
        false
    }

    fn shl(&mut self, count: u64) -> bool {
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

    fn shr_test(&self, count: u64) -> (Integer, bool) {
        todo!()
    }

    fn shl_test(&self, count: u64) -> (Integer, bool) {
        todo!()
    }
}
