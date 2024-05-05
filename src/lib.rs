mod mbyte;

#[cfg(test)]
mod tests {
    use crate::mbyte::int::Integer;

    #[test]
    fn tt() {
        let mut q = Integer::new(3);
        q.initalize(0);
        assert_eq!(q.size(), 3);
        assert_eq!("00000000 00000000 00000000", q.bit());

        let mut qq = Integer::new(3);
        q.write_at(2, 8);
        qq.write_at(2, 26);
        qq.write_at(1, 16);
        assert_eq!(q.to_string(), "8");
        assert_eq!(qq.to_string(), "4122");

        q.add(&qq);
        assert_eq!("0x00 0x10 0x22", q.hex());
        assert_eq!(q.to_string(), "4130");

        let mut wow = Integer::new(2075); // 2075 byte
        wow.assign("126");
        assert_eq!("126", wow.to_string());
        // About 2 seconds

        wow.initalize(0xff);
        print!("{}", wow);
    }
}