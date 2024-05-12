use std::fmt::{Display, Formatter};
use crate::int::Integer;
use crate::int::util::bcd::*;

impl Display for Integer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut data = self.bytes.to_vec();
        let mut v = String::new();
        loop {
            match data.len() {
                0 => break,
                1 if data[0] < 10  => {
                    if data[0] != 0 { v.insert_str(0, &*data[0].to_string()); }
                    break;
                },
                _ => {
                    let tmp = div10(data.as_slice());
                    data = tmp.0;
                    v.insert_str(0, &*tmp.1.to_string());
                },
            }
        };
        if v.is_empty() { v.push_str("0"); }
        write!(f, "{}", &*v)
    }
}