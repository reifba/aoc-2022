use crate::filereader;

enum Operation {
    Noop,
    Addx(i32),
}

impl Operation {
    fn from_string(s: &str) -> Operation {
        let mut iter = s.split_whitespace();
        let op = iter.next().unwrap();
        match op {
            "noop" => Operation::Noop,
            "addx" => Operation::Addx(iter.next().unwrap().parse::<i32>().unwrap()),
            _ => panic!("Unknown operation"),
        }
    }
}

const INDEX_LOCATIONS: [usize; 6] = [20, 60, 100, 140, 180, 220];

fn get_operation_list() -> Vec<Operation> {
    filereader::lines(10)
        .iter()
        .filter(|x| !x.is_empty())
        .map(|x| Operation::from_string(x))
        .collect()
}

fn time_line(ops: &[Operation]) -> Vec<i32> {
    let mut time_line = Vec::new();

    for op in ops {
        let last: i32 = *time_line.last().unwrap_or(&1);
        match op {
            Operation::Noop => time_line.push(last),
            Operation::Addx(x) => {
                time_line.push(last);
                time_line.push(last + x);
            }
        };
    }

    time_line
}

pub fn day_10_1() -> i32 {
    let ops = get_operation_list();

    let time_line = time_line(&ops);

    let mut acc = 0;

    for location in INDEX_LOCATIONS {
        acc += time_line[location - 2] * (location as i32);
    }

    acc
}

fn print_out(time_line: &[i32], offset: usize) {
    print!(".");
    let forty_cycle_iter = (0..40).cycle();
    for (sprite_location, pixel) in time_line.iter().zip(forty_cycle_iter.skip(offset)) {
        if pixel % 40 == 0 {
            println!();
        }
        if (*sprite_location - pixel).abs() <= 1 {
            print!("#");
        } else {
            print!(".");
        }
    }

    println!();
}

// .##..###..###...##..###...##...##..####.
// #..#.#..#.#..#.#..#.#..#.#..#.#..#.#....
// #..#.###..#..#.#..#.#..#.#..#.#....###..
// ###..#..#.###..####.###..####.#.##.#....
// #.#..#..#.#....#..#.#.#..#..#.#..#.#....
// #..#.###..#....#..#.#..#.#..#..###.#....

pub fn day_10_2() {
    let ops = get_operation_list();
    let time_line = time_line(&ops);

    print_out(&time_line, 1);
}
