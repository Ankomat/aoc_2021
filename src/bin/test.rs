use std::collections::{HashMap, HashSet};

fn main() {
    let word = "abc";
    let wirings = HashMap::from([('a', 0b1000_0000u8),('b',0b0001_0000u8),('c',0b0000_0100u8)]);
    let mut temp = 0b0000_0000u8;
    for ch in word.chars() {
        temp |= wirings.get(&ch).unwrap();
    }
    println!("{:#010b}", temp);
}
