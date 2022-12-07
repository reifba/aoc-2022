use std::collections::HashSet;

use crate::filereader;

fn parse_file() -> Vec<char> {
    let input = filereader::lines(6);

    let line = input.first().unwrap();

    let s = line.clone();

    s.chars().collect()
}

fn unique_n2(chars: &[char]) -> bool {
    for i in 0..chars.len() {
        for j in (i + 1)..chars.len() {
            if chars[i] == chars[j] {
                return false;
            }
        }
    }

    true
}

fn is_unique(chars: &[char]) -> bool {
    if chars.len() <= 10 {
        return unique_n2(chars);
    }

    let mut set = HashSet::with_capacity(chars.len() + 1);

    for c in chars {
        if set.contains(c) {
            return false;
        } else {
            set.insert(c);
        }
    }

    true
}

pub fn day_6(size: usize) -> usize {
    let vec = parse_file();

    for (i, c) in vec.windows(size).enumerate() {
        if is_unique(c) {
            return i + size;
        }
    }

    panic!("Should have found a match")
}
