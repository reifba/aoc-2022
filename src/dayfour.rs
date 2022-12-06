use crate::filereader;

fn is_left_in_right(a: u32, b: u32, x: u32, y: u32) -> bool {
    (x..=y).contains(&a) && (x..=y).contains(&b)
}

fn _any_overlap(a: u32, b: u32, x: u32, y: u32) -> bool {
    (x..=y).contains(&a) || (x..=y).contains(&b)
}

fn any_overlaps(a: u32, b: u32, x: u32, y: u32) -> bool {
    _any_overlap(a, b, x, y) || _any_overlap(x, y, a, b)
}

fn exact_overlaps(a: u32, b: u32, x: u32, y: u32) -> bool {
    is_left_in_right(a, b, x, y) || is_left_in_right(x, y, a, b)
}

fn parse_line(s: &String) -> (u32, u32, u32, u32) {
    let mut iter = s.split(',');
    let mut iter2 = iter.next().unwrap().split('-');
    let a = iter2.next().unwrap().parse::<u32>().unwrap();
    let b = iter2.next().unwrap().parse::<u32>().unwrap();
    let mut iter3 = iter.next().unwrap().split('-');
    let x = iter3.next().unwrap().parse::<u32>().unwrap();
    let y = iter3.next().unwrap().parse::<u32>().unwrap();
    (a, b, x, y)
}

pub fn day_4_1() -> usize {
    filereader::lines(4)
        .iter()
        .filter(|x| !x.is_empty())
        .map(parse_line)
        .filter(|(a, b, x, y)| exact_overlaps(*a, *b, *x, *y))
        .count()
}

pub fn day_4_2() -> usize {
    filereader::lines(4)
        .iter()
        .filter(|x| !x.is_empty())
        .map(parse_line)
        .filter(|(a, b, x, y)| any_overlaps(*a, *b, *x, *y))
        .count()
}
