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
    match read_file("03.txt") {
		Ok(input) => {
            let v: Vec<&str> = input.trim().split('\n').collect();
    // let input = vec!["00100","11110","10110","10111","10101","01111","00111",
    //     "11100","10000","11001","00010","01010"];
            let output: Vec<_> = v
                .iter()
                .filter(|x| x.chars().nth(0).unwrap().to_digit(10).unwrap() == 0)
                .collect();
            println!("{:?}", output);
            println!("{}", type_of(output));
            println!("{}", type_of(input));
        }
        Err(e) => println!("{}", e),
	}
}
