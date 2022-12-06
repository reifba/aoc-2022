use std::{cmp::Reverse, fs};

pub const PATH_1: &str = "/Users/Barak/Documents/aoc/input.txt";

fn bins() -> Vec<u32> {
    let contents = fs::read_to_string(PATH_1).expect("Should have been able to read the file");

    let mut vec: Vec<u32> = Vec::new();

    let mut acc: u32 = 0;

    for line in contents.lines() {
        if line.is_empty() {
            vec.push(acc);
            acc = 0;
        } else {
            acc += line.parse::<u32>().unwrap();
        }
    }

    vec.push(acc);
    vec.sort_by_key(|w| Reverse(*w));

    vec
}

pub fn day_1_1() -> u32 {
    let vec = bins();

    vec.iter().take(3).sum::<u32>()
}

pub fn day_1_2() -> u32 {
    let vec = bins();

    *vec.iter().max().unwrap()
}