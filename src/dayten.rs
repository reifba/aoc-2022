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

const INDEXED_LOCATIONS: [usize; 6] = [20, 60, 100, 140, 180, 220];
const LINE_LENGTH: i32 = 40;

fn get_operation_list() -> Vec<Operation> {
    filereader::lines(10)
        .iter()
        .filter(|x| !x.is_empty())
        .map(|x| Operation::from_string(x))
        .collect()
}

fn time_line(ops: &[Operation], defualt: i32) -> Vec<i32> {
    let mut time_line = Vec::new();

    for op in ops {
        let last: i32 = *time_line.last().unwrap_or(&defualt);
        time_line.push(last);

        if let Operation::Addx(x) = op {
            time_line.push(last + x);
        }
    }

    time_line
}

pub fn day_10_1() -> i32 {
    let ops = get_operation_list();

    let time_line = time_line(&ops, 1);

    INDEXED_LOCATIONS
        .iter()
        .map(|x| time_line[*x - 2] * (*x as i32))
        .sum()
}

fn print_out(time_line: &[i32], offset: usize) {
    print!("#");
    let forty_cycle_iter = (0..LINE_LENGTH).cycle();
    for (sprite_location, pixel) in time_line.iter().zip(forty_cycle_iter.skip(offset)) {
        if pixel % LINE_LENGTH == 0 {
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
    let time_line = time_line(&ops, 1);

    print_out(&time_line, 1);
}
