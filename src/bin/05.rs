#![allow(unused)]
use std::fs::File;
use std::path::Path;
use std::io::{self, Read, BufRead};
use std::collections::{HashMap, HashSet};
use std::any::type_name;
// --------------------------------------------
fn main() {
    day5_part1();
    day5_part2();
}
// --------------------------------------------
fn day5_part1() {
    // read from file into Vector of strings
	let input = read_file("input/05.txt");
	let v: Vec<&str> = input.trim().split('\n').collect();
    let mut iter = v.iter();
    let mut points_visited = HashMap::new();
    let mut danger_count = 0;

    // reshuffle input into useful form
    loop {
        match iter.next() {
            Some(line) => {
                let points: Vec<&str> = line.split(" -> ").collect();
                let mut begin: Vec<u32> = points[0].split(',').map(|x| x.parse::<u32>().unwrap()).collect();
                let mut end: Vec<u32> = points[1].split(',').map(|x| x.parse::<u32>().unwrap()).collect();
                if begin[0] == end[0] || begin[1] == end[1] {
                    if begin[0] > end[0] {
                        let temp = begin[0];
                        begin[0] = end[0];
                        end[0] = temp;
                    }
                    if begin[1] > end[1] {
                        let temp = begin[1];
                        begin[1] = end[1];
                        end[1] = temp;
                    }
                    for x in begin[0]..(end[0]+1) {
                        for y in begin[1]..(end[1]+1) {
                            let count = points_visited.entry((x,y)).or_insert(0);
                            *count += 1;
                            if *count == 2 {
                                danger_count += 1;
                            }
                        }
                    }
                }
                // println!("line from {:?} to {:?}", begin, end);
            }
            None => break
        }
    }
    println!("Part 1: number of dangerous points is {:?}", danger_count);
}

fn day5_part2() {
    // read from file into Vector of strings
	let input = read_file("input/05.txt");
	let v: Vec<&str> = input.trim().split('\n').collect();
    let mut iter = v.iter();
    let mut points_visited: HashMap<(u32, u32), u32> = HashMap::new();
    let mut danger_count = 0;

    // reshuffle input into useful form
    loop {
        match iter.next() {
            Some(line) => {
                let points: Vec<&str> = line.split(" -> ").collect();
                let mut begin: Vec<u32> = points[0].split(',').map(|x| x.parse::<u32>().unwrap()).collect();
                let mut end: Vec<u32> = points[1].split(',').map(|x| x.parse::<u32>().unwrap()).collect();
                let mut x_range = Vec::new();
                let mut y_range = Vec::new();
                let delta_x = (begin[0] as i32 - end[0] as i32).abs() as usize;
                let delta_y = (begin[1] as i32 - end[1] as i32).abs() as usize;

                if begin[0] == end[0] { // vertical
                    x_range = vec![end[0] ; delta_y + 1];
                } else if begin[0] < end[0] {   // delta x > 0
                    let temp = begin[0]..(end[0]+1);
                    x_range = temp.collect();
                } else {    // delta x < 0
                    let temp = end[0]..(begin[0]+1);
                    x_range = temp.rev().collect();
                }

                if begin[1] == end[1] { // horizontal
                    y_range = vec![end[1] ; delta_x + 1];
                } else if begin[1] < end[1] {  // delta y > 0
                    let temp = begin[1]..(end[1]+1);
                    y_range = temp.collect();
                } else {    // delta y < 0
                    let temp = end[1]..begin[1]+1;
                    y_range = temp.rev().collect();
                }

                let mut line_points = x_range.into_iter().zip(y_range).collect::<Vec<_>>();
                for (x,y) in &line_points {
                    let count = points_visited.entry((*x,*y)).or_insert(0);
                    *count += 1;
                    if *count == 2 {
                        danger_count += 1;
                    }
               }
            }
            None => break
        }
    }
    println!("Part 2: number of dangerous points is {:?}", danger_count);
}

fn read_file<P>(filename: P) -> String where P: AsRef<Path>, {
    let mut buffer = String::new();
    File::open(filename).unwrap().read_to_string(&mut buffer).unwrap();
    buffer
}

// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// where P: AsRef<Path>, {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
