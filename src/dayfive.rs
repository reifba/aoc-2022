use crate::filereader;

fn pile_to_column(i: usize) -> usize {
    if !(1..=9).contains(&i) {
        panic!("Invalid column number");
    }
    1 + 4 * (i - 1)
}

struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn new(amount: usize, from: usize, to: usize) -> Move {
        Move { amount, from, to }
    }

    fn from_string(s: &str) -> Move {
        let word_vec: Vec<&str> = s.split_whitespace().collect();
        let amount = word_vec[1].parse::<usize>().unwrap();
        let from: usize = word_vec[3].parse::<usize>().unwrap();
        let to: usize = word_vec[5].parse::<usize>().unwrap();
        Move::new(amount, from, to)
    }
}

struct FileInput {
    moves: Vec<Move>,
    piles: Vec<Vec<char>>,
}
impl FileInput {
    fn extract_top_line(piles: Vec<Vec<char>>) -> String {
        piles
            .iter()
            .skip(1)
            .map(|x| x.last().unwrap())
            .cloned()
            .collect::<String>()
    }

    fn from_file() -> FileInput {
        let input = filereader::lines(5);

        let mut cargo_lines: Vec<String> = Vec::new();
        let mut moves: Vec<String> = Vec::new();
        let mut column_line: String = String::new();

        for line in input {
            if line.is_empty() {
                continue;
            } else if line.starts_with(' ') {
                column_line = line;
            } else if line.starts_with("move") {
                moves.push(line);
            } else {
                cargo_lines.push(line);
            }
        }

        let max: usize = column_line
            .chars()
            .filter(|x| x.is_ascii_digit())
            .map(|x| x.to_digit(10).unwrap() as usize)
            .max()
            .unwrap();

        let mut cargo: Vec<Vec<char>> = Vec::with_capacity(max + 1);

        for _ in 0..=max {
            cargo.push(Vec::with_capacity(cargo_lines.len()));
        }

        for lines in cargo_lines.iter().rev() {
            let char_vec: Vec<char> = lines.chars().collect();

            for (i, pile) in cargo.iter_mut().enumerate().take(max + 1).skip(1) {
                let c = char_vec[pile_to_column(i)];
                if c != ' ' {
                    pile.push(c);
                }
            }
        }
        FileInput {
            moves: moves.iter().map(|x| Move::from_string(x)).collect(),
            piles: cargo,
        }
    }
}

pub fn day_5_1() -> String {
    let mut input = FileInput::from_file();

    for m in input.moves {
        for _ in 0..m.amount {
            let c = input.piles[m.from].pop().unwrap();
            input.piles[m.to].push(c);
        }
    }

    FileInput::extract_top_line(input.piles)
}

pub fn day_5_2() -> String {
    let mut input = FileInput::from_file();

    for m in input.moves {
        let length = input.piles[m.from].len();
        let mut temp_vec: Vec<char> = input.piles[m.from].split_off(length - m.amount);
        input.piles[m.to].append(&mut temp_vec);
    }

    FileInput::extract_top_line(input.piles)
}
