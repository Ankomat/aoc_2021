#![allow(unused)]
use std::fs::File;
use std::path::Path;
use std::io::{self, Read, BufRead};
use std::collections::{HashMap, HashSet};
use std::any::type_name;
use std::time::Instant;
// --------------------------------------------
fn main() {
    day8_part1();
    day8_part2();
}
// --------------------------------------------
fn day8_part1() {
    // read from file into Vector of strings
	let input = read_file("input/08.txt");
	let v: Vec<_> = input.trim().split('\n').collect();
    let output_long: Vec<&str> = v.into_iter()
        .map(|x| *x.split(" | ").collect::<Vec<&str>>().get(1).unwrap())
        .collect();
    let mut count = 0;
    for o in output_long {
        let output_short: Vec<&str> = o.trim().split(' ').collect();
        let sub_count = output_short.iter().filter(|x| [2,3,4,7].contains(&x.len())).count();
        count += sub_count;
    }
    println!("There are {} outputs of length 2, 3, 4 or 7", count);
}

fn day8_part2() {
    // read from file into Vector of strings
	let input = read_file("input/08.txt");
    let v: Vec<_> = input.trim().split('\n').collect();
    let mut count: u32 = 0;
    // go through each line
    for line in v {
        // prepare variables
        let mut wirings: HashMap<char,u8> = "pabcdefg".chars().zip([0b1111_1111u8;8]).collect();
        let mut five_buffer = String::new();
        let mut six_buffer = String::new();
        let mut temp = line.split(" | ");
        let inputs = temp.next().unwrap();
        let outputs = temp.next().unwrap();
        // go through input words
        // do the mapping on the unique numbers (1, 4, 7) (forget about 8 it does not help)
        // if five or six letter word, collect for later
        for word in inputs.split(' ') {
            let mut filter = 0b1111_1111u8;
            match word.len() {
                2 => {
                    filter = 0b0011_0000u8;
                    wirings = wirings.into_iter().map(|(x,y)| if word.contains(x) {(x , y & filter)} else {(x , y & !filter)}).collect();
                },
                3 => {
                    filter = 0b0111_0000u8;
                    wirings = wirings.into_iter().map(|(x,y)| if word.contains(x) {(x , y & filter)} else {(x , y & !filter)}).collect();
                },
                4 => {
                    filter = 0b0011_0011u8;
                    wirings = wirings.into_iter().map(|(x,y)| if word.contains(x) {(x , y & filter)} else {(x , y & !filter)}).collect();
                },
                5 => five_buffer.push_str(word),
                6 => six_buffer.push_str(word),
                _ => (),
            }
        }
        // do the mapping on all the five letter words (numbers 2, 3 and 5)
        let mut five_map = char_freq(&five_buffer);
        for (num, chars) in five_map.iter() {
            let mut filter = 0b1111_1111u8;
            match num {
                1 => {
                    filter = 0b0000_0110u8;
                    wirings = wirings.into_iter().map(|(x,y)| if chars.contains(&x) {(x , y & filter)} else {(x , y & !filter)}).collect();
                },
                2 => {
                    filter = 0b0011_0000u8;
                    wirings = wirings.into_iter().map(|(x,y)| if chars.contains(&x) {(x , y & filter)} else {(x , y & !filter)}).collect();
                },
                3 => {
                    filter = 0b0100_1001u8;
                    wirings = wirings.into_iter().map(|(x,y)| if chars.contains(&x) {(x , y & filter)} else {(x , y & !filter)}).collect();
                },
                _ => (),
            }
        }
        // do the mapping on all the six letter words (numbers 6, 0 and 9)
        let mut six_map = char_freq(&six_buffer);
        for (num, chars) in six_map.iter() {
            let mut filter = 0b1111_1111u8;
            match num {
                2 => {
                    filter = 0b0010_0101u8;
                    wirings = wirings.into_iter().map(|(x,y)| if chars.contains(&x) {(x , y & filter)} else {(x , y & !filter)}).collect();
                },
                3 => {
                    filter = 0b0101_1010u8;
                    wirings = wirings.into_iter().map(|(x,y)| if chars.contains(&x) {(x , y & filter)} else {(x , y & !filter)}).collect();
                },
                _ => (),
            }
        }
        // finished with all inputs, all letters are now mapped to bits(=segments) in unique way (1 to 1)
        // letter p is "parity" bit, should only be on left most bit to ensure all bits are mapped
        // p: 0b1000_0000

        // now work with the outputs, match to numbers
        let mut result = String::new();
        for word in outputs.split(' ') {
            match word.len() {
                2 => result.push_str("1"),
                3 => result.push_str("7"),
                4 => result.push_str("4"),
                7 => result.push_str("8"),
                _ => {
                    let mut temp = 0b0000_0000u8;
                    for ch in word.chars() {
                        temp |= wirings.get(&ch).unwrap();
                    }
                    match temp {
                        0b0110_1101u8 => result.push_str("2"),
                        0b0111_1001u8 => result.push_str("3"),
                        0b0101_1011u8 => result.push_str("5"),
                        0b0101_1111u8 => result.push_str("6"),
                        0b0111_1110u8 => result.push_str("0"),
                        0b0111_1011u8 => result.push_str("9"),
                        _ => (),
                    }
                }
            }
        }
        count += result.parse::<u32>().unwrap();
    }
    println!("{}", count);
}

fn read_file<P>(filename: P) -> String where P: AsRef<Path>, {
    let mut buffer = String::new();
    File::open(filename).unwrap().read_to_string(&mut buffer).unwrap();
    buffer
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn char_freq(word: &str) -> HashMap<usize,HashSet<char>> {
    let mut char_freq = HashMap::new();
    for ch in word.chars() {
        let freq = word.matches(ch).count();
        let set = char_freq.entry(freq).or_insert(HashSet::new());
        set.insert(ch);
    }
    char_freq
}
