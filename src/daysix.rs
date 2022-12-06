use std::fs;

const PATH: &str = "/Users/Barak/Documents/aoc/input6.txt";

fn parse_file() -> Vec<char> {
    let contents = fs::read_to_string(PATH).expect("Should have been able to read the file");

    let line = contents.lines().next().unwrap();

    line.chars().collect()
}

fn four_are_unique(a: char, b: char, c: char, d: char) -> bool {
    a != b && a != c && a != d && b != c && b != d && c != d
}

pub fn day_6_1() -> usize {
    let vec = parse_file();

    for (i, c) in vec.windows(4).enumerate() {
        if four_are_unique(c[0], c[1], c[2], c[3]) {
            return i + 4;
        }
    }

    panic!("Should have found a match")
}

fn fourteen_are_unique(chars: &[char]) -> bool {
    let mut set = std::collections::HashSet::new();

    for c in chars {
        if set.contains(c) {
            return false;
        } else {
            set.insert(c);
        }
    }

    true
}

pub fn day_6_2() -> usize {
    let vec = parse_file();

    for (i, c) in vec.windows(14).enumerate() {
        if fourteen_are_unique(c) {
            return i + 14;
        }
    }

    panic!("Should have found a match")
}