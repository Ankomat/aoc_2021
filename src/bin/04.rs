#![allow(unused)]
use std::fs::File;
use std::path::Path;
use std::io::{self, Read, BufRead};
use std::collections::{HashMap, HashSet};
use std::any::type_name;
// --------------------------------------------
fn main() {
    day4_part1();
    day4_part2();
}
// --------------------------------------------
fn day4_part1() {
    // read from file into Vector of strings
	let input = read_file("input/04.txt");
	let v: Vec<&str> = input.trim().split('\n').collect();
    let mut iter = v.iter();

    // reshuffle input into useful form
    let mut bingo_numbers: Vec<_> = iter.next().unwrap().split(',').map(|x| x.parse::<u32>().unwrap()).collect();
    let mut board_numbers = Vec::new(); // will be 1 vec with all board numbers in a row
    for _ in 0..100 { // there are 100 boards
        let mut row_as_numbers = Vec::new();
        iter.next(); // empty line
        row_as_numbers = iter.next().unwrap().split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
        board_numbers.extend_from_slice(&row_as_numbers);
        row_as_numbers = iter.next().unwrap().split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
        board_numbers.extend_from_slice(&row_as_numbers);
        row_as_numbers = iter.next().unwrap().split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
        board_numbers.extend_from_slice(&row_as_numbers);
        row_as_numbers = iter.next().unwrap().split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
        board_numbers.extend_from_slice(&row_as_numbers);
        row_as_numbers = iter.next().unwrap().split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
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
    let mut bingo_count = vec![vec![vec![0;5];2];100]; // 100 boards of 5 rows & columns
    let mut numbers_not_called = vec![1;2500];
    'outer: for x in bingo_numbers {
        let positions = board_lookup.get(&x).unwrap();
        for pos in positions {
            let (board, row, col) = *pos;
            bingo_count[board][0][row] += 1;
            bingo_count[board][1][col] += 1;
            numbers_not_called[25*board+5*row+col] = 0;
            if bingo_count[board][0][row] >= 5 || bingo_count[board][1][col] >= 5 {
                println!("Board {} has won with {} on position {:?}!", board, x, (row, col));
                println!("{:?}", bingo_count[board]);
                println!("{:?}", &numbers_not_called[25*board..25*(board+1)]);
                let this_board = &board_numbers[25*board..25*(board+1)];
                println!("{:?}", this_board);
                let sum: u32 = numbers_not_called[25*board..25*(board+1)].iter().zip(this_board).map(|(x, y)| x * y).sum();
                println!("{:?} * {} is {}", sum, x, sum * x);
                break 'outer;
            }
        }
    }
}

fn day4_part2() {
    // read from file into Vector of strings
	let input = read_file("input/04.txt");
	let v: Vec<&str> = input.trim().split('\n').collect();
    let mut iter = v.iter();

    // reshuffle input into useful form
    let mut bingo_numbers: Vec<_> = iter.next().unwrap().split(',').map(|x| x.parse::<u32>().unwrap()).collect();
    let mut board_numbers = Vec::new(); // will be 1 vec with all board numbers in a row
    for _ in 0..100 { // there are 100 boards
        let mut row_as_numbers = Vec::new();
        iter.next(); // empty line
        row_as_numbers = iter.next().unwrap().split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
        board_numbers.extend_from_slice(&row_as_numbers);
        row_as_numbers = iter.next().unwrap().split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
        board_numbers.extend_from_slice(&row_as_numbers);
        row_as_numbers = iter.next().unwrap().split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
        board_numbers.extend_from_slice(&row_as_numbers);
        row_as_numbers = iter.next().unwrap().split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
        board_numbers.extend_from_slice(&row_as_numbers);
        row_as_numbers = iter.next().unwrap().split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
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
    let mut bingo_count = vec![vec![vec![0;5];2];100]; // 100 boards of 5 rows & columns
    let mut numbers_not_called = vec![1;2500];
    let mut boards_won = HashSet::new();
    'outer: for x in bingo_numbers {
        let positions = board_lookup.get(&x).unwrap();
        for pos in positions {
            let (board, row, col) = *pos;
            bingo_count[board][0][row] += 1;
            bingo_count[board][1][col] += 1;
            numbers_not_called[25*board+5*row+col] = 0;
            if bingo_count[board][0][row] == 5 || bingo_count[board][1][col] == 5 {
                boards_won.insert(board);
                if boards_won.len() == 100 {
                    println!("Board {} has won last with {} on position {:?}!", board, x, (row, col));
                    println!("{:?}", bingo_count[board]);
                    println!("{:?}", &numbers_not_called[25*board..25*(board+1)]);
                    let this_board = &board_numbers[25*board..25*(board+1)];
                    println!("{:?}", this_board);
                    let sum: u32 = numbers_not_called[25*board..25*(board+1)].iter().zip(this_board).map(|(x, y)| x * y).sum();
                    println!("{:?} * {} is {}", sum, x, sum * x);
                    break 'outer;
                }
            }
        }
    }
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
