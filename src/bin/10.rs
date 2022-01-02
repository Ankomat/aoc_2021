#![allow(unused)]
use std::fs::File;
use std::path::Path;
use std::io::{self, Read, BufRead};
use std::collections::{HashMap, HashSet};
use std::any::type_name;
use std::time::Instant;
// --------------------------------------------
fn main() {
    // read from file into Vector of strings
    let input = read_file("input/10.txt");
    let mut v: Vec<_> = input.trim().split('\n').collect();
    day10_part1(&mut v);
    day10_part2(&mut v);
}
// --------------------------------------------

fn day10_part1(v: &mut Vec<&str>) {
    let mut stack: Vec<char> = Vec::new();
    let mut count = 0;
    // println!("Beginning lines = {}", v.len());
    let mut j = 0; //keep track of deleted lines because counter is not correct after v.remove()
    for (i, line) in v.clone().iter().enumerate() {
        // println!("Removed lines = {}",j);
        for ch in line.chars() {
            match ch {
                '(' => stack.push(')'),
                '{' => stack.push('}'),
                '[' => stack.push(']'),
                '<' => stack.push('>'),
                ')' => if ')' != stack.pop().unwrap() { count += 3; v.remove(i-j); j+=1; },
                '}' => if '}' != stack.pop().unwrap() { count += 1197; v.remove(i-j); j+=1; },
                ']' => if ']' != stack.pop().unwrap() { count += 57; v.remove(i-j); j+=1; },
                '>' => if '>' != stack.pop().unwrap() { count += 25137; v.remove(i-j); j+=1; },
                _ => (),
            };
        }
    }
    println!("Illegal score = {}", count);
}

fn day10_part2(v: &mut Vec<&str>) {
    let mut scores: Vec<u64> = Vec::new();
    for (i,line) in v.clone().iter().enumerate() { // only incomplete lines remain
        let mut stack: Vec<char> = Vec::new();
        let mut line_score: u64 = 0;
        // println!("line {} = {} needs:", i, line);
        for ch in line.chars() {
            match ch {
                '(' => { stack.push(')'); None},
                '{' => { stack.push('}'); None},
                '[' => { stack.push(']'); None},
                '<' => { stack.push('>'); None},
                ')' => stack.pop(),
                '}' => stack.pop(),
                ']' => stack.pop(),
                '>' => stack.pop(),
                _ => None,
            };
        }
        stack.reverse();
        for ch in stack {
            line_score *= 5;
            line_score += match ch {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => 0,
            }
        }
        scores.push(line_score);
    }
    scores.sort();
    println!("median score is: {:?}", scores.get(scores.len()/2));
}

fn read_file<P>(filename: P) -> String where P: AsRef<Path>, {
    let mut buffer = String::new();
    File::open(filename).unwrap().read_to_string(&mut buffer).unwrap();
    buffer
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
