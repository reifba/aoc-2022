use crate::filereader;

enum Entry {
    File { size: u32 },
    Dir { entries: Vec<Entry> },
}

impl Entry {
    fn nested_size(&self) -> u32 {
        match self {
            Entry::File { size, .. } => *size,
            Entry::Dir { entries, .. } => entries.iter().map(|e| e.nested_size()).sum::<u32>(),
        }
    }

    fn flatten(&self) -> Vec<&Entry> {
        match self {
            Entry::File { .. } => vec![self],
            Entry::Dir { entries, .. } => {
                let mut vec = vec![self];
                for e in entries {
                    vec.extend(e.flatten());
                }
                vec
            }
        }
    }
}

fn process(v: &Vec<String>, start_index: usize) -> (Entry, usize) {
    let mut map: Vec<Entry> = Vec::new();

    let mut i = start_index;

    while i < v.len() {
        let line = v[i].trim();
        i += 1;

        if line.is_empty() || line == "$ ls" || line.starts_with("dir") {
            continue;
        } else if line == "$ cd .." {
            break;
        } else if line.starts_with("$ cd") {
            let (entry_, j) = process(v, i); // rec happens here
            map.push(entry_);
            i = j;
        } else {
            let size = line
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            map.push(Entry::File { size });
        }
    }

    (Entry::Dir { entries: map }, i)
}

pub fn day_7_1() -> u32 {
    let input = filereader::lines(7);

    let (entries, _) = process(&input, 0);

    entries
        .flatten()
        .iter()
        .filter(|x| match x {
            Entry::File { size: _ } => false,
            Entry::Dir { entries: _ } => true,
        })
        .map(|x| x.nested_size())
        .filter(|x| *x <= 100000)
        .sum::<u32>()
}

const DISK_SIZE: u32 = 70000000;
const NEEDED_FREE: u32 = 30000000;

pub fn day_7_2() -> u32 {
    let input = filereader::lines(7);

    let (entries, _) = process(&input, 0);

    let flat_list = entries.flatten();

    let current_size = flat_list.iter().map(|x| x.nested_size()).max().unwrap();

    let current_free = DISK_SIZE - current_size;
    let need_to_free = NEEDED_FREE - current_free;

    flat_list
        .iter()
        .filter(|x| match x {
            Entry::File { size: _ } => false,
            Entry::Dir { entries: _ } => true,
        })
        .map(|x| x.nested_size())
        .filter(|x| *x >= need_to_free)
        .min()
        .unwrap()
}
