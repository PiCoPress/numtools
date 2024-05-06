pub mod natural;
mod bcd;
pub mod impls;

#[derive(Debug)]
pub struct Integer {
    size: usize,
    bytes: Box<[u8]>,
}

pub trait MUint {
    fn new(size: usize) -> Integer;
    fn from(t: &[u8]) -> Integer;

    fn size(&self) -> usize;

    fn write_at(&mut self, offset: usize, insert_byte: u8);
    fn initialize(&mut self, insert_byte: u8);
    fn clone(&self) -> Integer;

    fn assign(&mut self, s: &str);
    fn bits(&self) -> String;
    fn hex(&self) -> String;

    fn add(&mut self, other: &Integer) -> bool;
    fn subtract(&mut self, other: &Integer) -> bool;
    fn multiply(&mut self, other: &Integer) -> bool;
    fn divide(&mut self, other: Integer) -> (Integer, bool);
    fn shr(&mut self, other: Integer) -> bool;
    fn shl(&mut self, other: Integer) -> bool;

    fn add_test(&self, other: &Integer) -> (Integer, bool);
    fn subtract_test(&self, other: &Integer) -> (Integer, bool);
    fn multiply_test(&self, other: &Integer) -> (Integer, bool);
    fn divide_test(&self, other: &Integer) -> (Integer, Integer, bool);
    fn shr_test(&mut self, other: Integer) -> (Integer, bool);
    fn shl_test(&mut self, other: Integer) -> (Integer, bool);
}
