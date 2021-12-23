#![allow(unused)]
use std::fs::File;
use std::path::Path;
use std::io::{self, Read, BufRead};
use std::collections::HashMap;
use std::any::type_name;
// use std::math::round;
// --------------------------------------------
fn main() {
    day4_part1();
    // day4_part2();
}
// --------------------------------------------
fn day4_part1() {
	let input = read_file("04.txt");
	let v: Vec<&str> = input.trim().split('\n').collect();
    let mut iter = v.iter();
    let mut bingo_numbers: Vec<&str> = iter.next().unwrap().split(',').collect();
    let mut board_numbers: Vec<&str> = Vec::new();
    for _ in 0..100 {
        let mut row_as_numbers: Vec<&str> = Vec::new();
        iter.next();
        row_as_numbers = iter.next().unwrap().split(' ').filter(|x| *x != "").collect();
        board_numbers.extend_from_slice(&row_as_numbers);
        row_as_numbers = iter.next().unwrap().split(' ').filter(|x| *x != "").collect();
        board_numbers.extend_from_slice(&row_as_numbers);
        row_as_numbers = iter.next().unwrap().split(' ').filter(|x| *x != "").collect();
        board_numbers.extend_from_slice(&row_as_numbers);
        row_as_numbers = iter.next().unwrap().split(' ').filter(|x| *x != "").collect();
        board_numbers.extend_from_slice(&row_as_numbers);
        row_as_numbers = iter.next().unwrap().split(' ').filter(|x| *x != "").collect();
        board_numbers.extend_from_slice(&row_as_numbers);
    }
    // now make hashmap of board_numbers
    let mut board_lookup = HashMap::new();
    //: HashMap<&str, Vec<(u8,u8,u8)>

    // let mut bingo_count: Hash =
    // for x in bingo_numbers {
    for (i, x) in board_numbers.iter().enumerate() {
        let pos = (i / 25, (i - 25 * (i / 25)) / 5, (i - 25 * (i / 25)) % 5);
        let stat = board_lookup.entry(x).or_insert(vec!(pos));  // FIX THIS
        stat.push(pos);  // FIX THIS
    }
    println!("{:?}", board_lookup);

    // let board_numbers = binnenlezen als 1 lange lijn van getallen
    // dus lege lijnen laten vallen en nieuwe lijn achter de vorige te zetten
    // maar splitten per ','

    // board_numbers 1 voor 1 binnenlezen, locatie op bord is te vinden door
    // for (i, x) in board_numbers.enumerate() {
    // board = i mod 25
    // row = (i - 25 * board) mod 5
    // column = (i - 25 * board) remainder na mod 5
    // }

    // let bingo_count = hierin de score per bord en per lijn en kolom bijhouden (tot 5)
    // [bord 1[[lijn 1,lijn 2,0,0,0],[kolom 1,kolom 2,0,0,0]],[[0,0,0,0,0],[0,0,0,0,0]]]
    // per bingo_numbers bereken de locatie met modulo
    // bekijk wat de huidige score is, indien 4 --> dit bord wint
    // else bingo_count + 1


	// let mut gamma: i64 = 0;
    // let mut storage: Vec<i64> = vec![0; 12];
    //
	// for i in &v {
	// 	let mut iter = i.chars();
    //     for j in 0..12 {
    //         storage[j] += iter.next().unwrap().to_digit(10).unwrap() as i64;
    //     }
    // }
    //
    // for (i, x) in storage.iter().rev().enumerate() {
    //     if *x > 500 {
    //         gamma += 2_i64.pow(i as u32);
    //     }
    // }
    // println!("{:?}", storage);
    // let epsilon = 2_i64.pow(12_u32) - 1 - gamma;
    // println!("gamma is {:?}, epsilon is {:?} and power is {:?}", gamma, epsilon, gamma * epsilon);
}

// fn day3_part2() {
//     if let Ok(input) = read_file("03.txt") {
//         let mut v: Vec<&str> = input.trim().split('\n').collect();
//         for i in 0..v[0].len() {
//             if v.len() > 1 {
//                 let filter = most_common_bit(&v, &i);
//                 v = filter_by_char(v, i, filter);
//                 // println!("O2 result = {:?}", v);
//             } else {
//                 break
//             }
//         }
//         println!("O2 result = {:?} or {:?}", v[0], usize::from_str_radix(v[0], 2));
//
//         let mut result = usize::from_str_radix(v[0], 2).unwrap();
//
//         let mut v: Vec<&str> = input.trim().split('\n').collect();
//         for i in 0..v[0].len() {
//             if v.len() > 1 {
//                 let filter = least_common_bit(&v, &i);
//                 v = filter_by_char(v, i, filter);
//                 // println!("CO2 result = {:?}", v);
//             } else {
//                 break
//             }
//         }
//         println!("CO2 result = {:?} or {:?}", v[0], usize::from_str_radix(v[0], 2));
//         result *= usize::from_str_radix(v[0], 2).unwrap();
//         println!("Result is {:}", result);
//     }
// }

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
        // println!("Count = {} versus half length {}", count, input.len() as f32 / 2.0);
        if count as f32 >= (input.len() as f32 / 2.0) {
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
        // println!("Count = {} versus half length {}", count, input.len() as f32 / 2.0);
        if count as f32 >= (input.len() as f32 / 2.0) {
            '0'
        } else {
            '1'
        }
    }
}

// fn read_file<P>(filename: P) -> io::Result<String> where P: AsRef<Path>, {
//     let mut buffer = String::new();
//
//     match File::open(filename)?.read_to_string(&mut buffer) {
//         Ok(n) => {
//             println!("Read {:?} bytes", n);
//             return Ok(buffer)
//         }
//         Err(err) => return Err(err)
//     }
// }

fn read_file<P>(filename: P) -> String where P: AsRef<Path>, {
    let mut buffer = String::new();
    File::open(filename).unwrap().read_to_string(&mut buffer).unwrap();
    buffer
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
