#![allow(unused)]
use std::fs::File;
use std::path::Path;
use std::io::{self, Read, BufRead};
use std::any::type_name;
// --------------------------------------------
fn read_file<P>(filename: P) -> io::Result<String> where P: AsRef<Path>, {
    let mut buffer = String::new();

    match File::open(filename)?.read_to_string(&mut buffer) {
        Ok(n) => {
            println!("Read {:?} bytes", n);
            return Ok(buffer)
        }
        Err(err) => return Err(err)
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
// --------------------------------------------
fn main() {
    day3_part1();
    day3_part2();
}
// --------------------------------------------
fn day3_part1() {
	match read_file("03.txt") {
		Ok(input) => {
			let v: Vec<&str> = input.trim().split('\n').collect();
			let mut gamma: i64 = 0;
            let mut storage: Vec<i64> = vec![0; 12];

			for i in &v {
				let mut iter = i.chars();
                for j in 0..12 {
                    storage[j] += iter.next().unwrap().to_digit(10).unwrap() as i64;
                }
            }

            for (i, x) in storage.iter().rev().enumerate() {
                if *x > 500 {
                    gamma += 2_i64.pow(i as u32);
                }
            }
            println!("{:?}", storage);
            let epsilon = 2_i64.pow(12_u32) - 1 - gamma;
            println!("gamma is {:?}, epsilon is {:?} and power is {:?}", gamma, epsilon, gamma * epsilon);
		}
		Err(e) => println!("{}", e),
	}
}
fn day3_part2() {
    if let Ok(input) = read_file("03.txt") {
        let filter = ['0', '1', '0']; //[0, 12];
        let mut v = input.trim().split('\n').collect();
        for (i, x) in filter.iter().enumerate() {
            println!("Filtering for {} on position {}:", x, i);
            v = filter_by_char(v, i, *x);
        }
        println!("{:?}", v);
        // for i in 0..v.len() {
        //     let mut count = 0;
        //     for s in &v {
        //         count += s.chars().nth(i).to_digit(10);
        //     }
        // }
    }
}

// fn filter_by_char<'a>(input: Vec<&'a str>, pos: usize, filter: char) -> Vec<&'a str> {
fn filter_by_char(input: Vec<&str>, pos: usize, filter: char) -> Vec<&str> {
    // input.into_iter().filter(|x| x.chars().nth(pos).unwrap() == filter).collect()
    input.iter().cloned().filter(|x| x.chars().nth(pos).unwrap() == filter).collect()
}

// fn filter_on_most_common_bit (input: Vec<&str>) -> &str {
//
// }
