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
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();
    for line in v {
        let mut temp = line.split(" | ");
        inputs.push(temp.next().unwrap());
        outputs.push(temp.next().unwrap());
    }
    for (i, line) in inputs.iter().enumerate() {
        let mut wirings: HashMap<_,_> = "abcdefg".chars().zip([0b111_1111;7]).collect();
        let mut five_buffer = String::new();
        let mut six_buffer = String::new();
        // let mut temp = 0b111_1111;
        for word in line.split(' ') {
            // match word.len() {
            //     2 => temp = 0b011_0000,
            //     3 => temp = 0b111_0000,
            //     4 => temp = 0b011_0011,
            //     5 => five_buffer.push_str(word),
            //     6 => six_buffer.push_str(word),
            //     _ => (),
            // }
            // for ch in word.chars() {
            //     wirings = wirings.into_iter().map(|(x,y)| {if x == ch {(x,y&temp)} else {(x,y)}}).collect();
            // }
            match word.len() {
                2 => for ch in word.chars() {
                    let temp = wirings.get_mut(&ch).unwrap();
                    *temp &= 0b011_0000;
                    // for wire in wirings.iter() {
                    //     print!("{} = {:#09b};", wire.0, wire.1);
                    // }
                    // print!("\n");
                },
                3 => for ch in word.chars() {
                    let temp = wirings.get_mut(&ch).unwrap();
                    *temp &= 0b111_0000;
                },
                4 => for ch in word.chars() {
                    let temp = wirings.get_mut(&ch).unwrap();
                    *temp &= 0b011_0011;
                },
                5 => {
                    five_buffer.push_str(word);
                    // println!("5 letter word, 5-buffer is {}", five_buffer);
                },
                6 => {
                    six_buffer.push_str(word);
                },
                _ => (),
            }
        }
        // println!("finished short words in line {}", i);
        let mut five_map = count_chars(&five_buffer);
        for (ch, num) in five_map.iter() {
            match num {
                1 => {
                    let temp = wirings.get_mut(&ch).unwrap();
                    *temp &= 0b000_0110;
                },
                2 => {
                    let temp = wirings.get_mut(&ch).unwrap();
                    *temp &= 0b011_0000;
                },
                3 => {
                    let temp = wirings.get_mut(&ch).unwrap();
                    *temp &= 0b100_1001;
                },
                _ => (),
            }
        }
        let mut six_map = count_chars(&six_buffer);
        for (ch, num) in six_map.iter() {
            match num {
                2 => {
                    let temp = wirings.get_mut(&ch).unwrap();
                    *temp &= 0b010_0101;
                },
                3 => {
                    let temp = wirings.get_mut(&ch).unwrap();
                    *temp &= 0b101_1010;
                },
                _ => (),
            }
        }
        println!("after line {}:", i);
        for wire in wirings.iter() {
            print!("{} = {:#09b};", wire.0, wire.1);
        }
        print!("\n");
    }
}

fn read_file<P>(filename: P) -> String where P: AsRef<Path>, {
    let mut buffer = String::new();
    File::open(filename).unwrap().read_to_string(&mut buffer).unwrap();
    buffer
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn count_chars(word: &str) -> HashMap<char,usize> {
    let mut char_count = HashMap::new();
    for ch in word.chars() {
        let count = char_count.entry(ch).or_insert(0);
        *count += 1;
    }
    char_count
}
