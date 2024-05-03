use std::ops::Shl;

pub fn encode(data: &Vec<u8>) -> String {
    let mut arr: Vec<u8> = Vec::new();
    let mut cpy = data.clone();
    let mut check: bool = false;
    let mut real_len = data.len();
    for u in data {
        if *u != 0u8 { break; }
        real_len -= 1;
    }

    arr.resize(real_len * 2, 0);
    for _ in 0..(data.len() * 8) {
        let len = arr.len();
        // Left shift BCD codes
        for j1 in (0..len).rev() {
            let val = arr[j1].shl(1);
            arr[j1] = val & 15u8;
            if check {
                arr[j1] += 1;
                check = false;
            }
            if val >> 4 != 0u8 { check = true; }
        }
        // Left shift the copy
        for j2 in 0..data.len() {
            let (val, _) = cpy[j2].overflowing_shl(1);
            let mut carry = false;
            if cpy[j2] >= 128 { carry = true; }
            if j2 == 0 {
                if let Some(last) = arr.last_mut() {
                    *last += carry as u8;
                }
            }
            cpy[j2] = val;
            if carry && j2 != 0 { cpy[j2 - 1] += 1; }
        }
        // Double Dabble Algorithm
        for j3 in (0..len).rev() {
            if arr[j3] >= 5 {
                arr[j3] += 3;
            }
        }
    }
    let mut result = String::new();
    let mut zero = true;
    for (i, e) in arr.iter().enumerate() {
        // Trim zeros
        if zero {
            if *e != 0 {
                if i == arr.len() - 1 { return String::from(format!("{}", match data.last() { Some(v) => v, _ => &0u8 } ));}
                zero = false;
            }
            else { continue; }
        }
        result.push_str(&*format!("{}", e));
    }
    if result.is_empty() { String::from("0") } else { result }
}