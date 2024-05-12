#[cfg(test)]
pub mod misc {
    use crate::int::{Integer, MUint};

    #[test]
    fn test_1() {
        let mut i = Integer::from(438279u64);
        assert_eq!("438279", i);
        assert_eq!(i.bits(),
                   "00000000 00000000 00000000 00000000 00000000 00000110 10110000 00000111");

        assert_eq!(i.size(), 8);
        assert_eq!(i.hex(), "0x00 0x00 0x00 0x00 0x00 0x06 0xb0 0x07");
    }
    #[test]
    fn test_2() {
        let mut j = Integer::new(3);
        assert_eq!(j.to_string(), "0");

        j.initialize(0xff);
        assert_eq!("16777215", j.to_string());

        let mut j2 = Integer::new(7);
        j2.muint_add(&j);

        assert_eq!(true, j.muint_add(&Integer::from(1u8)));
        assert_eq!("0", j.to_string());

        assert_eq!(true, j.subtract(&Integer::from(1u16)));
        assert_eq!("16777215", j.to_string());
    }
}