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
    day2_part1();
    day2_part2();
}
// --------------------------------------------
fn day2_part1() {
	match read_file("input/02.txt") {
		Ok(input) => {
			let v: Vec<&str> = input.trim().split('\n').collect();
			let mut x: u32 = 0;
			let mut z: u32 = 0;

			for i in &v {
				let mut iter = i.split(' ');
				match iter.next().unwrap() {
					"forward" => x += iter.next().unwrap().parse::<u32>().unwrap(),
					"up" => z -= iter.next().unwrap().parse::<u32>().unwrap(),
					"down" => z += iter.next().unwrap().parse::<u32>().unwrap(),
					_ => (),
				}
			}
			println!("New coordinates are x = {} and z = {} and result = {}", x, z, x * z);
		}
		Err(e) => println!("{}", e),
	}
}
fn day2_part2() {
	match read_file("input/02.txt") {
		Ok(input) => {
			let v: Vec<&str> = input.trim().split('\n').collect();
			let mut x: i64 = 0;
			let mut a: i64 = 0;
			let mut z: i64 = 0;

			for i in &v {
				let mut iter = i.split(' ');
				match iter.next().unwrap() {
					"forward" => {
						let factor = iter.next().unwrap().parse::<i64>().unwrap();
						x += factor;
						z += factor * a;
					},
					"up" => a -= iter.next().unwrap().parse::<i64>().unwrap(),
					"down" => a += iter.next().unwrap().parse::<i64>().unwrap(),
					_ => (),
				}
			}
			println!("x = {}, z = {}, a = {} and result = {}", x, z, a, x * z);
		}
		Err(e) => println!("{}", e),
	}
}
