pub mod int;

#[cfg(test)]
mod tests {
    use crate::int::*;

    #[test]
    fn tt() {
        let mut q = Integer::new(3);
        q.initialize(0);
        assert_eq!(q.size(), 3);
        assert_eq!("00000000 00000000 00000000", q.bits());

        let mut qq = Integer::new(3);
        q.write_at(2, 8);
        qq.write_at(2, 26);
        qq.write_at(1, 16);
        assert_eq!(q.to_string(), "8");
        assert_eq!(qq.to_string(), "4122");

        q.add(&qq);
        assert_eq!("0x00 0x10 0x22", q.hex());
        assert_eq!(q.to_string(), "4130");

        let mut wow = Integer::new(1300);
        wow.assign("2332");
        assert_eq!("2332", wow);

        wow.initialize(0x00);
        assert_eq!(true, wow.subtract(&qq));

        wow.add(&q);
        assert_eq!("8", wow);

        assert_eq!(qq, "4122");
    }
}