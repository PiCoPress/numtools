use std::fmt::Display;
use std::ops::Add;

pub mod natural;
pub mod util;
pub mod impls;

#[derive(Debug)]
pub struct Integer {
    size: usize,
    bytes: Box<[u8]>,
}
pub trait MUint: Display + PartialEq + Add + Sized {

    /**
    Create new object that has N bytes, N is the first parameter.

    # Examples
    ```
    # use crate::numtools::int::*;
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

    fn size(&self) -> usize;
    fn equal(&self, rhs: &Integer) -> bool;

    fn write_at(&mut self, offset: usize, insert_byte: u8);
    fn initialize(&mut self, insert_byte: u8);
    fn assign(&mut self, s: &str);
    fn bits(&self) -> String;
    fn hex(&self) -> String;

    fn muint_add(&mut self, other: &Integer) -> bool;
    fn subtract(&mut self, other: &Integer) -> bool;
    fn multiply(&mut self, other: &Integer) -> bool;
    fn divide(&mut self, other: &Integer) -> (Integer, bool);
    fn shr(&mut self, count: u64) -> bool;
    fn shl(&mut self, count: u64) -> bool;

    fn add_test(&self, other: &Integer) -> (Integer, bool);
    fn subtract_test(&self, other: &Integer) -> (Integer, bool);
    fn multiply_test(&self, other: &Integer) -> (Integer, bool);
    fn divide_test(&self, other: &Integer) -> (Integer, Integer, bool);
    fn shr_test(&self, count: u64) -> (Integer, bool);
    fn shl_test(&self, count: u64) -> (Integer, bool);
}
