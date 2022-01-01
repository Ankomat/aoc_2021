#![allow(unused)]
use std::fs::File;
use std::path::Path;
use std::io::{self, Read, BufRead};
use std::collections::{HashMap, HashSet};
use std::any::type_name;
use std::time::Instant;
// --------------------------------------------
fn main() {
    day9_part1();
    day9_part2();
}
// --------------------------------------------
fn day9_part1() {
    // read from file into Vector of strings
	let input = read_file("input/09.txt");
	let v: Vec<_> = input.trim().split('\n').collect();
    let mut count = 0;
    for (i,line) in v.iter().enumerate() {
        for (j,ch) in line.chars().enumerate() {
            let val = ch.to_digit(10).unwrap();
            let mut neighbours = Vec::new();
            if i > 0 {
                let top = v.get(i-1).unwrap().chars().nth(j).unwrap();
                neighbours.push(top.to_digit(10).unwrap());
            }
            if i < (v.len() - 1) {
                let bottom = v.get(i+1).unwrap().chars().nth(j).unwrap();
                neighbours.push(bottom.to_digit(10).unwrap());
            }
            if j > 0 {
                let left = line.chars().nth(j-1).unwrap();
                neighbours.push(left.to_digit(10).unwrap());
            }
            if j < (line.len() - 1) {
                let right = line.chars().nth(j+1).unwrap();
                neighbours.push(right.to_digit(10).unwrap());
            }
            if val < *neighbours.iter().min().unwrap() {
                count += 1 + val;
            }
        }
    }
    println!("{}", count);
}

fn day9_part2() {
    // startpunten zijn de low points, vaan daaruit uitwaaieren
    // en 2 hashsets van coordinaten bijhouden: unchecked en checked
    // zodra een unchecked coordinaat aan de beurt is, verzetten naar unchecked
    // is een coordinaat < 9 dan uitwaaieren boven, rechts, onder, links (recursief)

}

fn read_file<P>(filename: P) -> String where P: AsRef<Path>, {
    let mut buffer = String::new();
    File::open(filename).unwrap().read_to_string(&mut buffer).unwrap();
    buffer
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

// fn char_freq(word: &str) -> HashMap<usize,HashSet<char>> {
//     let mut char_freq = HashMap::new();
//     for ch in word.chars() {
//         let freq = word.matches(ch).count();
//         let set = char_freq.entry(freq).or_insert(HashSet::new());
//         set.insert(ch);
//     }
//     char_freq
// }
