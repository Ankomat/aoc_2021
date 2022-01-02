#![allow(unused)]
use std::fs::File;
use std::path::Path;
use std::io::{self, Read, BufRead};
use std::collections::{HashMap, HashSet};
use std::any::type_name;
use std::time::Instant;
// --------------------------------------------
fn main() {
    let input = read_file("input/10.txt");
    let v: Vec<_> = input.trim().split('\n').collect();
    day10_part1(&v);
    // day10_part2(&v, &mut low_points);
}
// --------------------------------------------
// read from file into Vector of strings

fn day10_part1(v: &Vec<&str>) {
    let mut stack: Vec<char> = Vec::new();
    let mut count = 0;
    for line in v.iter() {
        for ch in line.chars() {
            match ch {
                '(' => stack.push(')'),
                '{' => stack.push('}'),
                '[' => stack.push(']'),
                '<' => stack.push('>'),
                ')' => if ')' != stack.pop().unwrap() { count += 3; },
                '}' => if '}' != stack.pop().unwrap() { count += 1197; },
                ']' => if ']' != stack.pop().unwrap() { count += 57; },
                '>' => if '>' != stack.pop().unwrap() { count += 25137; },
                _ => (),
            };
        }
    }
    println!("Illegal score = {}", count);
}

// fn day10_part2(v: &Vec<&str>, low_points: &HashSet<(usize,usize)>) {
//     let mut basins = HashMap::new();
//     for (i,j) in low_points {
//         let mut basin_size = 0;
//         let mut checked = HashSet::new();
//         basin_size += count_size(*i, *j, &mut checked, &v);
//         // println!("Low point ({},{}) has basin size {}", i, j, basin_size);
//         basins.insert((i,j), basin_size);
//     }
//     println!("{:?}", &basins);
//     println!("");
//     let mut temp = basins.values().cloned().collect::<Vec<usize>>();
//     temp.sort();
//     temp.reverse();
//     println!("{:?}", temp.get(0).unwrap()*temp.get(1).unwrap()*temp.get(2).unwrap());
// }
//
// fn count_size(i: usize, j: usize, checked: &mut HashSet<(usize,usize)>, v: &Vec<&str>) -> usize {
//     let mut height = v.get(i).unwrap().chars().nth(j).unwrap().to_digit(10).unwrap();;
//     let mut count = 0;
//     if !checked.contains(&(i,j)) && height < 9 {
//         checked.insert((i,j));
//         count += 1;
//         if i > 0 {
//             count += count_size(i-1, j, checked, &v);
//         }
//         if i < 99 {
//             count += count_size(i+1, j, checked, &v);
//         }
//         if j > 0 {
//             count += count_size(i, j-1, checked, &v);
//         }
//         if j < 99 {
//             count += count_size(i, j+1, checked, &v);
//         }
//     }
//     count
// }
//
// fn get_el(m: &Vec<&str>, row: usize, col: usize) -> Option<char> {
//     m.get(row)?.chars().nth(col)
// }

fn read_file<P>(filename: P) -> String where P: AsRef<Path>, {
    let mut buffer = String::new();
    File::open(filename).unwrap().read_to_string(&mut buffer).unwrap();
    buffer
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
