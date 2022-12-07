
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
        let line = v[i].clone();
        i += 1;

        if line.trim().is_empty() || line.trim() == "$ ls" || line.trim().starts_with("dir") {
            continue;
        } else if line.trim() == "$ cd .." {
            break;
        } else if line.starts_with("$ cd") {
            let (entry_, j) = process(v, i); // rec happens here
            map.push(entry_);
            i = j;
        } else {
            let size = line.split_whitespace().next().unwrap().parse::<u32>().unwrap();
            map.push(Entry::File { size });
        }
    }

    (Entry::Dir { entries: map }, i)
}

pub fn day_7_1() -> u32 {
    let input = filereader::lines(7);

    let e = process(&input, 0);

    let flat_list = e.0.flatten();

    flat_list
        .iter()
        .filter(|x| match x {
            Entry::File { size: _ } => false,
            Entry::Dir { entries: _ } => true,
        })
        .map(|x| x.nested_size())
        .filter(|x| *x <= 100000)
        .sum::<u32>()
}

pub fn day_7_2() -> u32 {
    let disk_size: u32 = 70000000;
    let need_free: u32 = 30000000;

    let input = filereader::lines(7);

    let e = process(&input, 0);

    let flat_list = e.0.flatten();

    let current_size = flat_list.iter().map(|x| x.nested_size()).max().unwrap();

    let current_free = disk_size - current_size;
    let need_to_free = need_free - current_free;

    flat_list
        .iter()
        .filter(|x| match x {
            Entry::File { size: _ } => false,
            Entry::Dir { entries: _ } => true,
        })
        .map(|x| x.nested_size())
        .filter(|x| *x > need_to_free)
        .min()
        .unwrap()
}
