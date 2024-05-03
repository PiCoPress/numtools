mod mbyte;

#[cfg(test)]
mod tests {
    use crate::mbyte::int::Integer;

    #[test]
    fn tt() {
        let mut q = Integer::new(3);
        assert_eq!(q.size(), 3);
        assert_eq!("00000000 00000000 00000000", q.show_as_bits());
        q.write_at(2, 8);
        let mut qq = Integer::new(3);
        qq.write_at(2, 26);
        qq.write_at(1, 16);
        println!("{} {qq}", q);
        q.add(&qq);
        print!("{}", q);
    }
}