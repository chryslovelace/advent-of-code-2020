use lazy_static::lazy_static;
use std::collections::{hash_map::DefaultHasher, BTreeMap};
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Hash)]
enum Seat {
    Empty,
    Occupied,
}

#[derive(Clone, Hash)]
struct Seats {
    seats: BTreeMap<(i32, i32), Seat>,
    width: usize,
    height: usize,
}

impl Seats {
    fn is_occupied(&self, x: i32, y: i32) -> bool {
        matches!(self.seats.get(&(x, y)), Some(Seat::Occupied))
    }

    fn occupied_adjacent(&self, x: i32, y: i32) -> u8 {
        self.is_occupied(x - 1, y - 1) as u8
            + self.is_occupied(x, y - 1) as u8
            + self.is_occupied(x + 1, y - 1) as u8
            + self.is_occupied(x + 1, y) as u8
            + self.is_occupied(x + 1, y + 1) as u8
            + self.is_occupied(x, y + 1) as u8
            + self.is_occupied(x - 1, y + 1) as u8
            + self.is_occupied(x - 1, y) as u8
    }

    fn next_seats(&self) -> Self {
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

    fn count_occupied(&self) -> usize {
        self.seats
            .values()
            .filter(|seat| match seat {
                Seat::Empty => false,
                Seat::Occupied => true,
            })
            .count()
    }

    fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        <Self as Hash>::hash(self, &mut hasher);
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
        seats = seats.next_seats();
        let seats_hash = seats.hash();
        if seats_hash == last_seats_hash {
            break;
        }
        last_seats_hash = seats_hash;
    }
    println!("*   {}", seats.count_occupied());
}

fn main() {
    part1();
}
