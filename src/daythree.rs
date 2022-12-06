use std::collections::HashSet;

use crate::filereader;

pub fn priority(c: char) -> Option<u32> {
    let v = c as u8;

    if ('a'..='z').contains(&c) {
        Some((1 + v - b'a') as u32)
    } else if ('A'..='Z').contains(&c) {
        Some((27 + v - b'A') as u32)
    } else {
        return None;
    }
}

pub fn get_item_from_split(s: &String) -> Option<char> {
    let length = s.len() / 2;
    let chars: Vec<char> = s.chars().collect();
    let h1: HashSet<char> = chars.iter().take(length).cloned().collect();
    for c in chars.iter().skip(length) {
        if h1.contains(c) {
            return Some(*c);
        }
    }
    panic!("Should have found a match");
}

pub fn day_3_1() -> u32 {
    filereader::lines(3)
        .iter()
        .filter_map(get_item_from_split)
        .filter_map(priority)
        .sum::<u32>()
}

fn multiple_interscetion(v: &[HashSet<char>]) -> char {
    v[0].iter()
        .filter(|c| v[1].contains(c) && v[2].contains(c))
        .cloned()
        .next()
        .unwrap()
}

pub fn day_3_2() -> u32 {
    filereader::lines(3)
        .iter()
        .filter(|x| !x.is_empty())
        .map(|s| HashSet::from_iter(s.chars()))
        .collect::<Vec<HashSet<char>>>()
        .chunks(3)
        .map(multiple_interscetion)
        .filter_map(priority)
        .sum::<u32>()
}
