#![allow(unused)]
use std::fs::File;
use std::path::Path;
use std::io::{self, Read, BufRead};
use std::collections::{HashMap, HashSet};
use std::any::type_name;
use std::time::Instant;
// --------------------------------------------
fn main() {
    // day7_part1();
    day8_part1();
    // day8_part2();
}
// --------------------------------------------
fn day8_part1() {
    // read from file into Vector of strings
	let input = read_file("input/08.txt");
	let v: Vec<_> = input.trim().split('\n').collect();
    let mut iter = v.iter();

    // get only the parts after | in a vector
    let output = iter.map(|x| x.split_at(" | ").1).collect();
    println!("{:?}", output);
}

// fn day7_part2v1() {
//     // read from file into Vector of strings
// 	let input = read_file("input/07.txt");
// 	let v: Vec<_> = input.trim().split(',').map(|x| x.parse::<i64>().unwrap()).collect();
//     let mut iter = v.iter();
//     let mut horizontal_positions = HashMap::new();
//     let lower: i64 = *v.iter().min().unwrap();
//     let upper: i64 = *v.iter().max().unwrap()+1;
//     // make vector-map of crab positions
//     loop {
//         match iter.next() {
//             Some(position) => {
//                 let count = horizontal_positions.entry(*position).or_insert(0);
//                 *count += 1;
//             }
//             None => break
//         }
//     }
//     // recursion in for loop
//     let now = Instant::now();
//     let mut minimum_fuel = 0;
//     for key in lower..upper {
//         let mut fuel: i64 = 0;
//         for (pos, num) in &horizontal_positions {
//             fuel += fuel_rec((*pos as i64 - key as i64).abs()) * num;
//         }
//         if key == lower || fuel < minimum_fuel {
//             minimum_fuel = fuel;
//         }
//     }
//     println!("recursion in for loop: {:?}", now.elapsed().as_secs_f64());
//     println!("minimum fuel needed is {:?}", minimum_fuel);
// }
//
// fn day7_part2v2() {
//     // read from file into Vector of strings
// 	let input = read_file("input/07.txt");
// 	let v: Vec<_> = input.trim().split(',').map(|x| x.parse::<i64>().unwrap()).collect();
//     let mut iter = v.iter();
//     let mut horizontal_positions = HashMap::new();
//     let lower: i64 = *v.iter().min().unwrap();
//     let upper: i64 = *v.iter().max().unwrap()+1;
//     // make vector-map of crab positions
//     loop {
//         match iter.next() {
//             Some(position) => {
//                 let count = horizontal_positions.entry(*position).or_insert(0);
//                 *count += 1;
//             }
//             None => break
//         }
//     }
//     // recursion in iterator closure
//     let now = Instant::now();
//     let mut minimum_fuel = 0;
//     for key in lower..upper {
//         let fuel: i64 = horizontal_positions.iter().map(|(pos, num)| fuel_rec((*pos as i64 - key as i64).abs()) * *num as i64).sum();
//         if key == lower || fuel < minimum_fuel {
//             minimum_fuel = fuel;
//         }
//     }
//     println!("recursion in iterator closure: {:?}", now.elapsed().as_secs_f64());
//     println!("minimum fuel needed is {:?}", minimum_fuel);
// }
//
// fn day7_part2v3() {
//     // read from file into Vector of strings
// 	let input = read_file("input/07.txt");
// 	let v: Vec<_> = input.trim().split(',').map(|x| x.parse::<i64>().unwrap()).collect();
//     let mut iter = v.iter();
//     let mut horizontal_positions = HashMap::new();
//     let lower: i64 = *v.iter().min().unwrap();
//     let upper: i64 = *v.iter().max().unwrap()+1;
//     // make vector-map of crab positions
//     loop {
//         match iter.next() {
//             Some(position) => {
//                 let count = horizontal_positions.entry(*position).or_insert(0);
//                 *count += 1;
//             }
//             None => break
//         }
//     }
//     // iteration in iterator closure
//     let now = Instant::now();
//     let mut minimum_fuel = 0;
//     for key in lower..upper {
//         let fuel: i64 = horizontal_positions.iter().map(|(pos, num)| fuel_ite((*pos as i64 - key as i64).abs()) * *num as i64).sum();
//         if key == lower || fuel < minimum_fuel {
//             minimum_fuel = fuel;
//         }
//     }
//     println!("iteration in iterator closure: {:?}", now.elapsed().as_secs_f64());
//     println!("minimum fuel needed is {:?}", minimum_fuel);
// }
//
// fn day7_part2v4() {
//     // read from file into Vector of strings
// 	let input = read_file("input/07.txt");
// 	let v: Vec<_> = input.trim().split(',').map(|x| x.parse::<i64>().unwrap()).collect();
//     let mut iter = v.iter();
//     let mut horizontal_positions = HashMap::new();
//     let lower: i64 = *v.iter().min().unwrap();
//     let upper: i64 = *v.iter().max().unwrap()+1;
//     // make vector-map of crab positions
//     loop {
//         match iter.next() {
//             Some(position) => {
//                 let count = horizontal_positions.entry(*position).or_insert(0);
//                 *count += 1;
//             }
//             None => break
//         }
//     }
//     // iteration in for loop
//     let now = Instant::now();
//     let mut minimum_fuel = 0;
//     for key in lower..upper {
//         let mut fuel: i64 = 0;
//         for (pos, num) in &horizontal_positions {
//             fuel += fuel_ite((*pos as i64 - key as i64).abs()) * num;
//         }
//         if key == lower || fuel < minimum_fuel {
//             minimum_fuel = fuel;
//         }
//     }
//     println!("iteration in for loop: {:?}", now.elapsed().as_secs_f64());
//     println!("minimum fuel needed is {:?}", minimum_fuel);
// }
//
// fn day7_part2v5() {
//     // read from file into Vector of strings
// 	let input = read_file("input/07.txt");
// 	let v: Vec<_> = input.trim().split(',').map(|x| x.parse::<i64>().unwrap()).collect();
//     let mut iter = v.iter();
//     let mut horizontal_positions = HashMap::new();
//     let lower: i64 = *v.iter().min().unwrap();
//     let upper: i64 = *v.iter().max().unwrap()+1;
//     // make vector-map of crab positions
//     loop {
//         match iter.next() {
//             Some(position) => {
//                 let count = horizontal_positions.entry(*position).or_insert(0);
//                 *count += 1;
//             }
//             None => break
//         }
//     }
//     // cached iteration in for loop
//     let now = Instant::now();
//     let mut minimum_fuel = 0;
//     let mut fuel_cache: HashMap<i64, i64> = HashMap::new();
//     // let mut cache_hit = 0_u64;
//     // let mut loops = 0_u64;
//     for key in lower..upper {
//         let mut fuel: i64 = 0;
//         for (pos, num) in &horizontal_positions {
//             // loops += 1;
//             let delta = (*pos as i64 - key as i64).abs();
//             let temp = fuel_cache.entry(delta).or_insert(fuel_ite(delta));
//             fuel += *temp * num;
//             // fuel += match fuel_cache.get(&delta) {
//             //     Some(f) => {
//             //         cache_hit += 1;
//             //         *f * num
//             //     },
//             //     None => {
//             //         let temp = fuel_ite(delta);
//             //         fuel_cache.insert(delta, temp);
//             //         temp * num
//             //     },
//             // }
//         }
//         if key == lower || fuel < minimum_fuel {
//             minimum_fuel = fuel;
//         }
//     }
//     println!("cached iteration in for loop: {:?}", now.elapsed().as_secs_f64());
//     println!("minimum fuel needed is {:?}", minimum_fuel);
//     // println!("cache hit {:?} times in {:?} loops.", cache_hit, loops);
// }
//
// fn fuel_rec(i: i64) -> i64 {
//     if i == 0 {
//         0
//     } else {
//         i + fuel_rec(i-1)
//     }
// }
//
// fn fuel_ite(i: i64) -> i64 {
//     let mut fuel = 0;
//     for i in 1..i+1 {
//         fuel += i;
//     }
//     fuel
// }

fn read_file<P>(filename: P) -> String where P: AsRef<Path>, {
    let mut buffer = String::new();
    File::open(filename).unwrap().read_to_string(&mut buffer).unwrap();
    buffer
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
