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
    day1_part1();
    day1_part2()
}
// --------------------------------------------
fn day1_part1() {
    if let Ok(lines) = read_lines("01.txt") {
        // let mut previous_depth: i32 = -1;
        let mut previous_depth: Option<u32> = None;
        let mut changes: u32 = 0;
        for line in lines {
            // println!("Previous depth is {:?}", previous_depth);
            if let Ok(depth_string) = line {
                let depth: u32 = depth_string.parse().unwrap();
                // println!("New depth is {:?}", depth);
                // if let Some(previous_depth) < depth {
                match previous_depth {
                    Some(i) => {
                        if i < depth {
                            changes += 1;
                            // println!("Detected increase by {} to {}. Already {} increases.", depth - i, depth, changes);
                        }
                    },
                    _ => {},
                };
                previous_depth = Some(depth);
            }
        }
            println!("Detected {} changes.", changes);
    }
}
fn day1_part2() {
	match read_file("01.txt") {
		Ok(input) => {
			let v: Vec<u32> = input.trim().split('\n').map(|l| l.parse::<u32>().unwrap()).collect();
			let mut increases = 0;

			for i in 3..v.len() {
        		let v1 = v[i-3] + v[i-2] + v[i-1];
				let v2 = v[i-2] + v[i-1] + v[i];
				if v2 > v1 {
					increases += 1;
				}
			}
			println!("{}", increases);
		}
		Err(e) => println!("{}", e),
	}
}
