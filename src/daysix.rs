
use crate::filereader;

fn parse_file() -> Vec<char> {
    let input = filereader::lines(6);

    let line = input.first().unwrap();

    let s = line.clone();

    s.chars().collect()
}


fn is_unique(chars: &[char]) -> bool {

    let mut vc = chars.to_vec();
    vc.sort();
    vc.dedup();

    vc.len() == chars.len()
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
