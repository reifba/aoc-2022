use crate::filereader;

#[inline(always)]
fn is_left_in_right(a: u32, b: u32, x: u32, y: u32) -> bool {
    (x..=y).contains(&a) && (x..=y).contains(&b)
}

#[inline(always)]
fn _any_overlap(a: u32, b: u32, x: u32, y: u32) -> bool {
    (x..=y).contains(&a) || (x..=y).contains(&b)
}

#[inline(always)]
fn any_overlaps(arr: &[u32; 4]) -> bool {
    let (a, b, x, y) = (arr[0], arr[1], arr[2], arr[3]);
    _any_overlap(a, b, x, y) || _any_overlap(x, y, a, b)
}

#[inline(always)]
fn exact_overlaps(arr: &[u32; 4]) -> bool {
    is_left_in_right(arr[0], arr[1], arr[2], arr[3])
        || is_left_in_right(arr[2], arr[3], arr[0], arr[1])
}

fn parse_line(s: &str) -> [u32; 4] {
    let v2: Vec<u32> = s
        .split(&[',', '-'])
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    [v2[0], v2[1], v2[2], v2[3]]
}

pub fn day_4_1() -> usize {
    filereader::lines(4)
        .iter()
        .filter(|x| !x.is_empty())
        .map(|x| parse_line(x))
        .filter(exact_overlaps)
        .count()
}

pub fn day_4_2() -> usize {
    filereader::lines(4)
        .iter()
        .filter(|x| !x.is_empty())
        .map(|x| parse_line(x))
        .filter(any_overlaps)
        .count()
}
