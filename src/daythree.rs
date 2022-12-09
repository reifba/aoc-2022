use crate::filereader;

struct CharSet {
    bitmap: [u8; 26 * 2],
}

impl CharSet {
    fn priority(c: char) -> Option<u32> {
        let v = c as u8;

        if ('a'..='z').contains(&c) {
            Some((1 + v - b'a') as u32)
        } else if ('A'..='Z').contains(&c) {
            Some((27 + v - b'A') as u32)
        } else {
            return None;
        }
    }

    fn new() -> CharSet {
        CharSet {
            bitmap: [0; 26 * 2],
        }
    }

    fn index(c: char) -> usize {
    (CharSet::priority(c).unwrap() - 1) as usize
    }

    fn contains(&self, c: char) -> bool {
        let index = CharSet::index(c);
        self.bitmap[index] > 0
    }

    fn insert(&mut self, c: char) -> bool {
        let index = CharSet::index(c);
        let alredy_exists = self.bitmap[index] > 0;
        self.bitmap[index] += 1;
        alredy_exists
    }

    fn intersect(&self, other: &CharSet) -> CharSet {
        let mut set = CharSet::new();
        for i in 0..26 * 2 {
            if self.bitmap[i] > 0 && other.bitmap[i] > 0 {
                set.bitmap[i] = 1;
            }
        }
        set
    }

    fn first_exising(&self) -> Option<char> {
        for i in 0..26 * 2 {
            if self.bitmap[i] > 0 {
                if (0..26).contains(&i) {
                    return Some((b'a' + i as u8) as char);
                } else {
                    return Some((b'A' + i as u8) as char);
                }
            }
        }
        None
    }

    fn from_iter(iter: impl Iterator<Item = char>) -> CharSet {
        let mut set = CharSet::new();
        for c in iter {
            set.insert(c);
        }
        set
    }
}

pub fn get_item_from_split(s: &String) -> Option<char> {
    let length = s.len() / 2;
    let s: Vec<char> = s.chars().collect();
    let char_set = CharSet::from_iter(s.iter().take(length).cloned());

    for c in s.iter().skip(length) {
        if char_set.contains(*c) {
            return Some(*c);
        }
    }
    panic!("Should have found a match");
}

pub fn day_3_1() -> u32 {
    filereader::lines(3)
        .iter()
        .filter_map(get_item_from_split)
        .filter_map(CharSet::priority)
        .sum::<u32>()
}

fn multiple_interscetion(v: &[CharSet]) -> char {
    let mut char_set = v[0].intersect(&v[1]);

    for cs  in v.iter().skip(2) {
        char_set = char_set.intersect(cs);
    }

    char_set.first_exising().unwrap()
}

pub fn day_3_2() -> u32 {
    filereader::lines(3)
        .iter()
        .filter(|x| !x.is_empty())
        .map(|s| CharSet::from_iter(s.chars()))
        .collect::<Vec<CharSet>>()
        .chunks(3)
        .map(multiple_interscetion)
        .filter_map(CharSet::priority)
        .sum::<u32>()
}
