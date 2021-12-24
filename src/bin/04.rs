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
    // read from file into Vector of strings
	let input = read_file("04.txt");
	let v: Vec<&str> = input.trim().split('\n').collect();
    let mut iter = v.iter();

    // reshuffle input into useful form
    let mut bingo_numbers: Vec<&str> = iter.next().unwrap().split(',').collect();
    let mut board_numbers: Vec<&str> = Vec::new(); // will be 1 vec with all board numbers in a row
    for _ in 0..100 { // there are 100 boards
        let mut row_as_numbers: Vec<&str> = Vec::new();
        iter.next(); // empty line
        row_as_numbers = iter.next().unwrap().split(' ').filter(|x| *x != "").collect(); // first row
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

    // make hashmap of board_numbers
    let mut board_lookup = HashMap::new();
    for (i, &x) in board_numbers.iter().enumerate() {
        let board = i / 25;
        let pos = (board, (i - 25 * board) / 5, (i - 25 * board) % 5);
        let stat = board_lookup.entry(x).or_insert(Vec::new());
        stat.push(pos);
    }

    // now run through bingo_numbers, lookup positions in hashmap
    // and add 1 to bingo_count for each position until count = 5 per row or per col
    // then this board has won
    let mut bingo_count = vec![vec![vec![0;5];2];100]; // 100 boards of 5 rows & columns
    let mut numbers_called = vec![vec![vec![0;5];5];100];
    // println!("{:?}", bingo_count[1][1][1]);
    'outer: for x in bingo_numbers {
        let positions = board_lookup.get(x).unwrap();
        for pos in positions {
            let (board, row, col) = *pos;
            bingo_count[board][0][row] += 1;
            bingo_count[board][1][col] += 1;
            numbers_called[board][row][col] = 1;
            if bingo_count[board][0][row] >= 5 || bingo_count[board][1][col] >= 5 {
                println!("Board {} has won with {} on position {:?}!", board, x, (row, col));
                println!("{:?}", bingo_count[board]);
                println!("{:?}", numbers_called[board]);

                break 'outer;
            }
        }
    }
    // println!("{} can be found at {:?}", x, positions);
}

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
