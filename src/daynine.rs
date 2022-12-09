use std::collections::HashSet;

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

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn new() -> Location {
        Location { x: 0, y: 0 }
    }

    fn new_location(&self, m: &Move) -> Location {
        let mut new_location = Location::new();
        new_location.x = self.x;
        new_location.y = self.y;

        match m.direction {
            Direction::Up => new_location.y += m.distance,
            Direction::Down => new_location.y -= m.distance,
            Direction::Left => new_location.x -= m.distance,
            Direction::Right => new_location.x += m.distance,
        }

        new_location
    }

    fn are_touching(&self, other: &Location) -> bool {
        let (x, y) = self.delta(other);
        x.abs() <=1 && y.abs() <= 1
    }

    fn delta(&self, other: &Location) -> (i32, i32) {
        (self.x - other.x, self.y - other.y)
    }
}

fn get_next_tail_location(head: &Location, tail: &Location) -> Option<Location> {
    if head.are_touching(tail) {
        return None;
    }

    if head.x != tail.x && head.y != tail.y {
        //make diagonal move
        let (mut x, mut y) = head.delta(tail);
        x /= x.abs();
        y /= y.abs();
        Some(Location {
            x: tail.x + x,
            y: tail.y + y,
        })
    } else {
        //make horizontal or vertical move
        let (mut x, mut y) = head.delta(tail);
        if x != 0 {
            x /= x.abs();
        } else {
            y /= y.abs();
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

fn get_following_knot_locations(head_locations: Vec<Location> ) -> Vec<Location> {
    let mut tail_locations: Vec<Location> = Vec::new();
    tail_locations.push(*head_locations.first().unwrap());

    for h in head_locations {
        let new_tail = get_next_tail_location(&h, tail_locations.last().unwrap());
        if let Some(tail) = new_tail {
            tail_locations.push(tail);
        }
    }

    tail_locations
}

pub fn day_9_1(knots: usize) -> usize {
    let moves = moves(&filereader::lines(9));

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

    let mut tail_locations: Vec<Location> = get_following_knot_locations(head_locations);
    
    for _ in 0..(knots-1) {
        tail_locations = get_following_knot_locations(tail_locations);

    }
    // tail_locations.push(start);

    // for h in head_locations {
    //     let new_tail = get_next_tail_location(&h, tail_locations.last().unwrap());
    //     if let Some(tail) = new_tail {
    //         tail_locations.push(tail.clone());
    //     }
    // }

    let h: HashSet<Location> = HashSet::from_iter(tail_locations);

    h.len()
}


