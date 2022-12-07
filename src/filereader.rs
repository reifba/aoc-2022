use std::fs;

fn path(day: usize) -> String {
    format!("/Users/Barak/Documents/aoc-2022/inputs/input{}.txt", day)
}

pub fn lines(day: usize) -> Vec<String> {
    let p = path(day);

    fs::read_to_string(p)
        .expect("Error reading file")
        .lines()
        .map(|x| x.to_string())
        .collect()
}