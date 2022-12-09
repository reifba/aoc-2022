use crate::filereader;

const MIN_CHAR: char = '/';

fn get_column_from_matrix(matrix: &[Vec<char>], column: usize) -> Vec<char> {
    matrix.iter().map(|x| x[column]).collect()
}

fn visable_tree_indices(tree_line: &[char]) -> Vec<usize> {
    let mut visable_indices: Vec<usize> = Vec::new();
    let mut max = MIN_CHAR;

    for (index, tree) in tree_line.iter().enumerate() {
        if *tree > max {
            max = *tree;
            visable_indices.push(index);
        }
    }

    max = MIN_CHAR;
    for (index, tree) in tree_line.iter().enumerate().rev() {
        if *tree > max {
            max = *tree;
            visable_indices.push(index);
        }
    }

    visable_indices
}

pub fn day_8_1() -> usize {
    let input = filereader::lines(8);
    let forest: Vec<Vec<char>> = input.iter().map(|x| x.chars().collect()).collect();

    let columns = forest[0].len();

    let mut set: Vec<(usize, usize)> = Vec::new();

    for (row, tree_line) in forest.iter().enumerate() {
        let visable_indices = visable_tree_indices(tree_line);

        for column in visable_indices {
            set.push((row, column));
        }
    }

    set.sort();
    set.dedup();

    for column in 0..columns {
        let tree_line = get_column_from_matrix(&forest, column);
        let visable_indices = visable_tree_indices(&tree_line);

        for row in visable_indices {
            set.push((row, column));
        }
    }

    set.sort();
    set.dedup();

    set.len()
}

fn brute_force_scenic_score(
    forest: &[Vec<char>],
    row: usize,
    column: usize,
    max_so_far: usize,
) -> usize {
    let tree_height = forest[row][column];

    let rows = forest.len();
    let columns = forest[0].len();

    let mut left: usize = 0;
    let mut down: usize = 0;
    let mut up: usize = 0;

    let max_right = columns - column;
    let max_left = column + 1;
    let max_up = row + 1;
    let max_down = rows - row;

    if max_right * max_left * max_down * max_up < max_so_far {
        return 0;
    }

    let right = {
        let mut counter: usize = 0;
        for i in (column + 1)..columns {
            counter += 1;
            if forest[row][i] >= tree_height {
                break;
            }
        }
        counter
    };

    if right * max_left * max_down * max_up < max_so_far {
        return 0;
    }

    for i in (0..column).rev() {
        left += 1;
        if forest[row][i] >= tree_height {
            break;
        }
    }

    if right * left * max_down * max_up < max_so_far {
        return 0;
    }

    for tree_line in forest.iter().skip(row + 1) {
        down += 1;
        if tree_line[column] >= tree_height {
            break;
        }
    }

    if right * left * down * max_up < max_so_far {
        return 0;
    }

    for i in (0..row).rev() {
        up += 1;
        if forest[i][column] >= tree_height {
            break;
        }
    }

    left * right * up * down
}

pub fn day_8_2() -> usize {
    let input = filereader::lines(8);
    let forest: Vec<Vec<char>> = input.iter().map(|x| x.chars().collect()).collect();

    let rows = forest.len();
    let columns = forest[0].len();

    let mut max = 0;

    for row in 1..(rows - 1) {
        for column in 1..(columns - 1) {
            let score = brute_force_scenic_score(&forest, row, column, max);
            if score > max {
                max = score;
            }
        }
    }

    max
}
