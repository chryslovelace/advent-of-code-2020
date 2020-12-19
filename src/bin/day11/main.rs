use lazy_static::lazy_static;
use std::collections::{hash_map::DefaultHasher, BTreeMap};
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Hash)]
enum Seat {
    Empty,
    Occupied,
}

#[derive(Clone)]
struct Seats {
    seats: BTreeMap<(i32, i32), Seat>,
    width: usize,
    height: usize,
}

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

impl Seats {
    fn is_occupied(&self, x: i32, y: i32) -> bool {
        matches!(self.seats.get(&(x, y)), Some(Seat::Occupied))
    }

    fn occupied_adjacent(&self, x: i32, y: i32) -> usize {
        DIRECTIONS
            .iter()
            .filter(|(dx, dy)| self.is_occupied(x + dx, y + dy))
            .count()
    }

    fn next_seats_v1(&self) -> Self {
        let seats = self
            .seats
            .iter()
            .map(|(&(x, y), &seat)| {
                (
                    (x, y),
                    match seat {
                        Seat::Empty if self.occupied_adjacent(x, y) == 0 => Seat::Occupied,
                        Seat::Occupied if self.occupied_adjacent(x, y) >= 4 => Seat::Empty,
                        _ => seat,
                    },
                )
            })
            .collect();
        Self {
            seats,
            width: self.width,
            height: self.height,
        }
    }

    fn occupied_visible(&self, x: i32, y: i32) -> usize {
        DIRECTIONS
            .iter()
            .filter(|(dx, dy)| {
                matches!(
                    std::iter::successors(Some((x + dx, y + dy)), |(x, y)| Some((x + dx, y + dy)))
                        .take_while(|(x, y)| {
                            (0..self.width as i32).contains(x)
                                && (0..self.height as i32).contains(y)
                        })
                        .find_map(|pos| self.seats.get(&pos)),
                    Some(Seat::Occupied)
                )
            })
            .count()
    }

    fn next_seats_v2(&self) -> Self {
        let seats = self
            .seats
            .iter()
            .map(|(&(x, y), &seat)| {
                (
                    (x, y),
                    match seat {
                        Seat::Empty if self.occupied_visible(x, y) == 0 => Seat::Occupied,
                        Seat::Occupied if self.occupied_visible(x, y) >= 5 => Seat::Empty,
                        _ => seat,
                    },
                )
            })
            .collect();
        Self {
            seats,
            width: self.width,
            height: self.height,
        }
    }

    fn count_occupied(&self) -> usize {
        self.seats
            .values()
            .filter(|seat| matches!(seat, Seat::Occupied))
            .count()
    }

    fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.seats.hash(&mut hasher);
        hasher.finish()
    }
}

impl fmt::Display for Seats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.seats.get(&(x as i32, y as i32)) {
                    Some(Seat::Empty) => write!(f, "L")?,
                    Some(Seat::Occupied) => write!(f, "#")?,
                    None => write!(f, ".")?,
                };
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

lazy_static! {
    static ref INITIAL_SEATS: Seats = {
        let input = include_str!("input.txt");
        let seats = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| match c {
                    'L' => Some(((x as i32, y as i32), Seat::Empty)),
                    _ => None,
                })
            })
            .collect();
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        Seats {
            seats,
            width,
            height,
        }
    };
}

fn part1() {
    let mut seats = INITIAL_SEATS.clone();
    let mut last_seats_hash = seats.hash();
    loop {
        seats = seats.next_seats_v1();
        let seats_hash = seats.hash();
        if seats_hash == last_seats_hash {
            break;
        }
        last_seats_hash = seats_hash;
    }
    println!("*   {}", seats.count_occupied());
}

fn part2() {
    let mut seats = INITIAL_SEATS.clone();
    let mut last_seats_hash = seats.hash();
    loop {
        seats = seats.next_seats_v2();
        let seats_hash = seats.hash();
        if seats_hash == last_seats_hash {
            break;
        }
        last_seats_hash = seats_hash;
    }
    println!("**  {}", seats.count_occupied());
}

fn main() {
    part1();
    part2();
}
