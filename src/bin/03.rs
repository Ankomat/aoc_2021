#![allow(unused)]
use std::fs::File;
use std::path::Path;
use std::io::{self, Read, BufRead};
use std::any::type_name;
// use std::math::round;
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
        let mut v: Vec<&str> = input.trim().split('\n').collect();
        for i in 0..12 {
            if v.len() > 1 {
                let filter = most_common_bit(&v, &i);
                v = filter_by_char(v, i, filter);
            } else {
                break
            }
        }
        println!("O2 result = {:?} or {:?}", v[0], usize::from_str_radix(v[0], 2));
        let mut result = usize::from_str_radix(v[0], 2).unwrap();
        let mut v: Vec<&str> = input.trim().split('\n').collect();
        for i in 0..12 {
            if v.len() > 1 {
                let filter = least_common_bit(&v, &i);
                v = filter_by_char(v, i, filter);
            } else {
                break
            }
        }
        println!("CO2 result = {:?} or {:?}", v[0], usize::from_str_radix(v[0], 2));
        result *= usize::from_str_radix(v[0], 2).unwrap();
        println!("Result is {:}", result);
    }
}

fn filter_by_char(input: Vec<&str>, pos: usize, filter: char) -> Vec<&str> {
    input.iter().cloned().filter(|x| x.chars().nth(pos).unwrap() == filter).collect()
}

fn most_common_bit(input: &Vec<&str>, i: &usize) -> char {
    let mut count = 0;
    if input.len() == 2 {
        '1'
    } else {
        for s in input {
            count += s.chars().nth(*i).unwrap().to_digit(10).unwrap();
        }
        if count as f32 >= ((input.len() / 2) as f32).floor() {
            '1'
        } else {
            '0'
        }
    }
}

fn least_common_bit(input: &Vec<&str>, i: &usize) -> char {
    let mut count = 0;
    if input.len() == 2 {
        '0'
    } else {
        for s in input {
            count += s.chars().nth(*i).unwrap().to_digit(10).unwrap();
        }
        if count as f32 >= ((input.len() / 2) as f32).floor() {
            '0'
        } else {
            '1'
        }
    }
}
