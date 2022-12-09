use crate::filereader;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Move {
    direction: Direction,
    distance: i32,
}

impl Move {
    fn break_into_single_steps(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        for _ in 0..self.distance {
            moves.push(Move {
                direction: self.direction,
                distance: 1,
            });
        }
        moves
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn new() -> Location {
        Location { x: 0, y: 0 }
    }

    fn new_location(&self, m: &Move) -> Location {
        let (mut x, mut y) = (self.x, self.y);

        match m.direction {
            Direction::Up => y += m.distance,
            Direction::Down => y -= m.distance,
            Direction::Left => x -= m.distance,
            Direction::Right => x += m.distance,
        }

        Location { x, y }
    }

    #[inline(always)]
    fn delta(&self, other: &Location) -> (i32, i32) {
        (self.x - other.x, self.y - other.y)
    }
}

fn get_trailing_knot_location(head: &Location, tail: &Location) -> Option<Location> {
    let (mut x, mut y) = head.delta(tail);

    let (x_abs, y_abs) = (x.abs(), y.abs());

    if x_abs <= 1 && y_abs <= 1 {
        None
    } else {
        if x != 0 {
            x /= x_abs;
        }
        if y != 0 {
            y /= y_abs;
        }

        Some(Location {
            x: tail.x + x,
            y: tail.y + y,
        })
    }
}

fn moves(s: &[String]) -> Vec<Move> {
    s.iter()
        .map(|x| {
            let words = x.split_whitespace().collect::<Vec<&str>>();

            let direction = match words[0].chars().next().unwrap() {
                'U' => Direction::Up,
                'D' => Direction::Down,
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Invalid direction"),
            };
            let distance = words[1].parse::<i32>().unwrap();
            Move {
                direction,
                distance,
            }
        })
        .collect()
}

fn get_following_knot_locations(head_locations: Vec<Location>) -> Vec<Location> {
    let mut tail_locations: Vec<Location> = Vec::new();
    tail_locations.push(*head_locations.first().unwrap());

    for head in head_locations.iter().skip(1) {
        let tail = tail_locations.last().unwrap();
        let new_tail = get_trailing_knot_location(head, tail);
        if let Some(tail) = new_tail {
            tail_locations.push(tail);
        }
    }

    tail_locations
}

fn get_head_locations(moves: &[Move]) -> Vec<Location> {
    let mut head_locations: Vec<Location> = Vec::new();
    let start = Location::new();

    head_locations.push(start);

    for m in moves {
        for single_step in m.break_into_single_steps() {
            let last_location = head_locations.last().unwrap();
            let new_location = last_location.new_location(&single_step);
            head_locations.push(new_location);
        }
    }
    head_locations
}

pub fn day_9(knots: usize) -> usize {
    let moves = moves(&filereader::lines(9));

    let mut knot_locations = get_head_locations(&moves);

    for _ in 0..knots {
        knot_locations = get_following_knot_locations(knot_locations);
    }

    knot_locations.sort();
    knot_locations.dedup();
    knot_locations.len()
}
