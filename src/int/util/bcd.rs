use std::ops::Shl;

/**
    Convert binary to Decimal string <br>
    By using Double Dabble Algorithm.
 */
pub fn conv(data: &mut Vec<u8>) -> String {

    let mut arr: Vec<u8> = Vec::new(); // The vector for push BCDs
    let mut check: bool = false;       // Carry
    let mut cap = data.len();   // The index pointing the first non-zero element
    let data_len = cap;         // Backup the origin length; For this time, cap == data.len()

    // Trim zeros
    // If it meets a non-zero firstly,
    // stop this loop
    for u in data.iter() {
        if *u != 0u8 { break; }
        cap -= 1;
    }

    // Preallocate size
    arr.resize((data_len - cap) << 2, 0);
    for _ in 0..cap * 8 {
        let len = arr.len();

        // Double Dabble Algorithm
        // If one is not less than 5, add 3.
        for j3 in 0..len {
            if arr[j3] >= 5 { arr[j3] += 3; }
        }

        // Left shift BCD codes
        // For code convenience, a BCD is a byte.
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
        for j2 in 0..cap {
            let idx = data_len - cap + j2;
            let (val, _) = data[idx].overflowing_shl(1);
            let mut carry = false;
            if data[idx] & 128 != 0 { carry = true; }
            if j2 == 0 {
                if let Some(last) = arr.last_mut() { *last += carry as u8; }
            }
            data[idx] = val;
            if carry && j2 != 0 { data[idx - 1] += 1; }
        }
    }
    let mut result = String::new();
    let mut is_zero = true;

    // Trim zeros at start.
    for e in arr {
        if e == 0 && is_zero { continue; }
        else { is_zero = false; }
        result.push_str(&*e.to_string());
    }
    if result.is_empty() { String::from("0") } else { result }

}

/** Sourced from `ChatGPT` <br>
    # Prototype is here
    ```rs
    fn divide_big_u8_array_by_ten(bytes: &[u8]) -> Vec<u8> {
        let mut result = Vec::new();
        let mut remainder = 0u32;
        for &byte in bytes {
            let current = remainder * 256 + byte as u32;
            result.push((current / 10) as u8);
            remainder = current % 10;
        }
        while result.len() > 1 && result[0] == 0 {
            result.remove(0);
        }
        result
    }
    ```
 */
pub fn div10(bytes: &[u8]) -> (Vec<u8>, u32) {
    let mut result = Vec::new();
    let mut remainder = 0u32;
    let mut zero = true;

    for &byte in bytes {

        // Skip zero
        match (byte, zero) {
            (0, true) => { continue; },
            (_, true) => { zero = false; },
            _ => (),
        }

        let current = (remainder << 8) + byte as u32;
        let tmp = (current / 10) as u8;
        remainder = current % 10;

        // Prevent pushing zero after operation
        match (tmp, zero) {
            (0, true) => continue,
            (_, true) => zero = false,
            (_, _) => (),
        }
        result.push(tmp);
    }

    (result, remainder)
}

/**
    Reverse of conv <br>
    Convert string to byte array. <br>
    It does not trim string `s` except numeric. <br>
 */
pub fn revcon(s: &str, dat: &mut Box<[u8]>) -> usize {
    let siz = dat.len();

    // Str ==> unsigned byte
    let tmp = Vec::from(s).into_iter().map(|x| atoi_p(x));
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
                out[oi] |= 128;
                check = false;
            }
            if is_odd {
                if oi == out.len() - 1 {
                    out.push(128);
                } else { check = true; }
            }
            if arr[len - 1] % 2 == 1 { out[0] |= 128; }
        }

        // BCD stream
        for ai in 0..len {
            let is_odd = arr[ai] % 2 == 1;
            arr[ai] >>= 1;
            if check {
                arr[ai] |= 8;
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

    // Zeroing
    let _ = dat.into_iter().map(|_| 0);
    let mut count = 1;
    for el in out.iter().rev() {
        dat[siz - count] = *el;
        count += 1;
    }
    siz - (count - 1)
}

fn atoi_p(v: u8) -> u8 {
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