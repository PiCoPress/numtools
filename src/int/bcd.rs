use std::ops::Shl;

/**
    Convert binary to Decimal string
    By using Double Dabble Algorithm.
 */
pub fn conv(data: &Vec<u8>) -> String {
    let mut arr: Vec<u8> = Vec::new();
    let mut cpy = data.clone();
    let mut check: bool = false;
    let mut real_len = data.len();

    // Trim the nulls
    for u in data {
        if *u != 0u8 { break; }
        real_len -= 1;
    }

    arr.resize(real_len << 2 , 0);
    for _ in 0..(data.len() * 8) {
        let len = arr.len();

        // Double Dabble Algorithm
        for j3 in 0..len {
            if arr[j3] >= 5 { arr[j3] += 3; }
        }

        // Left shift BCD codes
        for j1 in (0..len).rev() {
            let val = arr[j1].shl(1);
            arr[j1] = val & 15u8;
            if check {
                arr[j1] += 1;
                check = false;
            }
            if val & 16u8 != 0u8 { check = true; }
        }

        // Left shift the copy of data
        for j2 in 0..data.len() {
            let (val, _) = cpy[j2].overflowing_shl(1);
            let mut carry = false;
            if cpy[j2] & 128 != 0 { carry = true; }
            if j2 == 0 {
                if let Some(last) = arr.last_mut() { *last += carry as u8; }
            }
            cpy[j2] = val;
            if carry && j2 != 0 { cpy[j2 - 1] += 1; }
        }
    }
    let mut result = String::new();
    let mut zero = true;
    for (i, e) in arr.iter().enumerate() {

        // Trim zeros
        if zero {
            if *e != 0 {
                if i == arr.len() - 1 {
                    return String::from(
                        format!("{}", match data.last() {
                            Some(v) => v,
                            _ => &0u8 }));
                }
                zero = false;
            }
            else { continue; }
        }
        result.push_str(&*format!("{}", e));
    }
    if result.is_empty() { String::from("0") } else { result }
}

fn shl(a: &mut Vec<u8>) ->  u8 {
    let c = a[0];
    a.remove(0);
    a.push(0);
    c
}

/*
    Allocate copy of data each of bits to bytes
    It takes more than about 3 times
*/
pub fn conv2(data: &Vec<u8>) -> String {
    let mut bitvec: Vec<u8> = Vec::new();
    for d in data {
        let mut q = 128;
        for _ in 0..8 {
            bitvec.push(if *d & q > 0 { 1 } else { 0 });
            q /= 2;
        }
    }
    let mut arr: Vec<u8> = Vec::new();
    let mut check: bool = false;
    let mut real_len = data.len();

    arr.resize(real_len << 2 , 0);
    for _ in 0..bitvec.len() {
        let len = arr.len();

        // Double Dabble Algorithm
        for j3 in 0..len {
            if arr[j3] >= 5 { arr[j3] += 3; }
        }

        // Left shift BCD codes
        for j1 in (0..len).rev() {
            let val = arr[j1].shl(1);
            arr[j1] = val & 15u8;
            if check {
                arr[j1] += 1;
                check = false;
            }
            if val & 16u8 != 0u8 { check = true; }
        }

        // Left shift the copy of data
        arr[len - 1] |= shl(&mut bitvec);
    }
    let mut result = String::new();
    let mut zero = true;
    for (_, e) in arr.iter().enumerate() {

        // Trim zeros
        if zero {
            if *e != 0 {
                zero = false;
            }
            else { continue; }
        }
        result.push_str(&*format!("{}", e));
    }
    if result.is_empty() { String::from("0") } else { result }
}

/*
    Maybe using recursion?
 */
pub fn conv3(data: &Vec<u8>) -> String {

    todo!()
}

/**
    Reverse of conv
    Convert string to byte array.
    It does not trim string `s` except numeric.
 */
pub fn revcon(s: &str, siz: usize) -> Box<[u8]> {
    let s = s.trim_start_matches("0");

    // Str ==> unsigned byte
    let tmp = Vec::from(s).into_iter().map(|x| atoi(x));
    let mut arr: Vec<u8> = Vec::new();
    for k in tmp { arr.push(k); }

    let len = arr.len();
    let mut out: Vec<u8> = Vec::new();
    out.push(0);

    // A variable `is_odd` is LSB
    for _ in 0..siz * 8 {
        let mut check = false;

        // The data of binary what supplied to Integer.
        for oi in 0..out.len() {
            let is_odd = out[oi] % 2 == 1;
            out[oi] >>= 1;
            if check {
                out[oi] = out[oi] | 128;
                check = false;
            }
            if is_odd {
                if oi == out.len() - 1 {
                    out.push(128);
                } else { check = true; }
            }
            if arr[len - 1] % 2 == 1 { out[0] = out[0] | 128; }
        }

        // BCD stream
        for ai in 0..len {
            let is_odd = arr[ai] % 2 == 1;
            arr[ai] >>= 1;
            if check {
                arr[ai] = arr[ai] | 8;
                check = false;
            }
            if is_odd { check = true; }
        }

        // Inverting DD-Algorithm
        for j3 in 0..len {
            if arr[j3] >= 8 { arr[j3] -= 3; }
        }
    }
    let verity = (siz - out.len()) as isize;
    if verity < 0 {
        panic!("Can't assign: {} < {}", siz, out.len());
    }

    // fill the zero at first to equalize size
    for _ in 0..verity { out.insert(0, 0); }
    out.into_boxed_slice()
}

fn atoi(v: u8) -> u8 {
    match v {
        b'1' => 1,
        b'2' => 2,
        b'3' => 3,
        b'4' => 4,
        b'5' => 5,
        b'6' => 6,
        b'7' => 7,
        b'8' => 8,
        b'9' => 9,
        b'0' => 0,
        _ => panic!("Not a number")
    }
}