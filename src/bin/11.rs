#![allow(unused)]
use std::fs::File;
use std::path::Path;
use std::io::{self, Read, BufRead};
use std::collections::{HashMap, HashSet};
use std::any::type_name;
use std::time::Instant;
use core::cmp::{max, min};
// --------------------------------------------
fn main() {
    // read from file into Vector of strings
    let input = read_file("input/11.txt");
    let mut v: Vec<_> = input.trim().split('\n').collect();
    day11_part1(&v);
    // day11_part2(&mut v);
}
// --------------------------------------------

fn day11_part1(v: &Vec<&str>) {
    let mut squids = Vec::new();
    let width = 10;
    for line in v {
        line.chars().for_each(|x| squids.push(x.to_digit(10).unwrap()));
    }
    println!("before: {:?}", squids);

// increase everything by 1
    for s in squids.iter_mut() {
        *s += 1;
    }
    println!("after step 1: {:?}", squids);

// find elements > 9
    for (i, s) in squids.iter_mut().enumerate() {
        if *s > 9 {
            increase_neigbours(squids, i);
        }
    }
}

fn increase_neigbours(squids: Vec<u32>, i: usize) {
    let row = i.div_euclid(9); // vanaf 10 is het rij 2
    let col = i.rem_euclid(9);
    let mut neighbours = HashSet::new();
    neighbours.insert((max(0, row-1), col)); // top
    neighbours.insert((min(9, row+1), col)); // bottom
    neighbours.insert((row, max(0, col-1))); // left
    neighbours.insert((row, min(9, col-1))); // right
    neighbours.insert((max(0, row-1), max(0, col-1))); // top left
    neighbours.insert((min(9, row+1), max(0, col-1))); // bottom left
    neighbours.insert((max(0, row-1), min(9, col-1))); // top right
    neighbours.insert((min(9, row+1), min(9, col-1))); // bottom right
}

// // for each flasher find all neighbours (max 8) and increase by 1
//     for (i, j) in flashers {
//         let mut neighbours: HashMap<usize,usize> = HashMap::new();
//         if let Some(neighbour) == squids.get_mut(1) {
//             *elem = 42;
//         }
//         if (1..9.contains(i) && 1..9.contains(j)) {
//             squids.get();
//         }
//         for (i,j) in neighbours
//     }
// }


// fn day11_part2(v: &mut Vec<&str>) {
//     let mut scores: Vec<u64> = Vec::new();
//     for (i,line) in v.clone().iter().enumerate() { // only incomplete lines remain
//         let mut stack: Vec<char> = Vec::new();
//         let mut line_score: u64 = 0;
//         // println!("line {} = {} needs:", i, line);
//         for ch in line.chars() {
//             match ch {
//                 '(' => { stack.push(')'); None},
//                 '{' => { stack.push('}'); None},
//                 '[' => { stack.push(']'); None},
//                 '<' => { stack.push('>'); None},
//                 ')' => stack.pop(),
//                 '}' => stack.pop(),
//                 ']' => stack.pop(),
//                 '>' => stack.pop(),
//                 _ => None,
//             };
//         }
//         stack.reverse();
//         for ch in stack {
//             line_score *= 5;
//             line_score += match ch {
//                 ')' => 1,
//                 ']' => 2,
//                 '}' => 3,
//                 '>' => 4,
//                 _ => 0,
//             }
//         }
//         scores.push(line_score);
//     }
//     scores.sort();
//     println!("median score is: {:?}", scores.get(scores.len()/2));
// }

fn read_file<P>(filename: P) -> String where P: AsRef<Path>, {
    let mut buffer = String::new();
    File::open(filename).unwrap().read_to_string(&mut buffer).unwrap();
    buffer
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
