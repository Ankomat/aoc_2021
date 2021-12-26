#![allow(unused)]
use std::fs::File;
use std::path::Path;
use std::io::{self, Read, BufRead};
use std::collections::{HashMap, HashSet};
use std::any::type_name;
// --------------------------------------------
fn main() {
    day7_part1();
    // day7_part2();
}
// --------------------------------------------
fn day7_part1() {
    // read from file into Vector of strings
	let input = read_file("input/07.txt");
	let v: Vec<_> = input.trim().split(',').map(|x| x.parse::<i64>().unwrap()).collect();
    let mut iter = v.iter();
    let mut horizontal_positions = HashMap::new();
    let lower: i64 = *v.iter().min().unwrap();
    let upper: i64 = *v.iter().max().unwrap()+1;
    // make vector-map of crab positions
    loop {
        match iter.next() {
            Some(position) => {
                // println!("{:?}", type_of(position));
                let count = horizontal_positions.entry(*position).or_insert(0);
                *count += 1;
                // population[*fish] += 1;
            }
            None => break
        }
    }
    let mut minimum_fuel = 0;
    for key in lower..upper {
        let fuel: i32 = horizontal_positions.iter().map(|(pos, num)| (*pos as i32 - key as i32).abs() * *num as i32).sum();
        if key == lower || fuel < minimum_fuel {
            minimum_fuel = fuel;
        }
    }
    println!("minimum fuel needed is {:?}", minimum_fuel);
}

// fn day7_part2() {
//     // read from file into Vector of strings
// 	let input = read_file("input/test_07.txt");
// 	let v: Vec<_> = input.trim().split(',').map(|x| x.parse::<i64>().unwrap()).collect();
//     let mut iter = v.iter();
//     let mut horizontal_positions = HashMap::new();
//     let lower: i64 = *v.iter().min().unwrap();
//     let upper: i64 = *v.iter().max().unwrap()+1;
//     // make vector-map of crab positions
//     loop {
//         match iter.next() {
//             Some(position) => {
//                 // println!("{:?}", type_of(position));
//                 let count = horizontal_positions.entry(*position).or_insert(0);
//                 *count += 1;
//                 // population[*fish] += 1;
//             }
//             None => break
//         }
//     }
//     println!("positions: {:?}", horizontal_positions);
//
//     let mut minimum_fuel = 0;
//
//     // for key in horizontal_positions.keys() {
//     for key in lower..upper {
//         // let fuel: i32 = horizontal_positions.iter().map(|(pos, num)| (*pos as i32 - key as i32).abs() * *num as i32).sum();
//         let steps: i64 = horizontal_positions.iter().map(|(pos, num)| fuel_needed((*pos as i64 - key as i64).abs()) * *num as i64).sum();
//
//         if key == lower || fuel < minimum_fuel {
//             minimum_fuel = fuel;
//         }
//         println!("to get to {:?}, {:?} fuel is needed", &key, fuel);
//     }
//     println!("minimum fuel needed is {:?}", minimum_fuel);
//
//     // println!("{:?}", fuel_needed(11));
// }

fn fuel_needed(i: i64) -> i64 {
    if i == 1 {
        1
    } else {
        i + fuel_needed(i-1)
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
