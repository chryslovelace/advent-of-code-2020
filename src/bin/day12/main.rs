use either::Either;
use lazy_static::lazy_static;
use std::str::FromStr;

enum Direction {
    N,
    S,
    E,
    W,
    L,
    R,
    F,
}

struct Action {
    dir: Direction,
    value: i32,
}

#[derive(Debug)]
struct UnrecognizedAction(Option<char>);

impl FromStr for Action {
    type Err = Either<UnrecognizedAction, std::num::ParseIntError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dir = match s.chars().next() {
            Some('N') => Direction::N,
            Some('S') => Direction::S,
            Some('E') => Direction::E,
            Some('W') => Direction::W,
            Some('L') => Direction::L,
            Some('R') => Direction::R,
            Some('F') => Direction::F,
            c => return Err(Either::Left(UnrecognizedAction(c))),
        };
        let value = s[1..].parse().map_err(Either::Right)?;
        Ok(Self { dir, value })
    }
}

#[derive(Clone, Copy)]
enum Facing {
    N,
    S,
    E,
    W,
}

struct ShipV1 {
    facing: Facing,
    east: i32,
    north: i32,
}

impl ShipV1 {
    fn new() -> Self {
        Self {
            facing: Facing::E,
            east: 0,
            north: 0,
        }
    }

    fn perform(&mut self, action: &Action) {
        match action.dir {
            Direction::N => self.north += action.value,
            Direction::S => self.north -= action.value,
            Direction::E => self.east += action.value,
            Direction::W => self.east -= action.value,
            Direction::L => {
                self.facing = match (self.facing, action.value % 360) {
                    (Facing::N, 90) => Facing::W,
                    (Facing::S, 90) => Facing::E,
                    (Facing::E, 90) => Facing::N,
                    (Facing::W, 90) => Facing::S,
                    (Facing::N, 180) => Facing::S,
                    (Facing::S, 180) => Facing::N,
                    (Facing::E, 180) => Facing::W,
                    (Facing::W, 180) => Facing::E,
                    (Facing::N, 270) => Facing::E,
                    (Facing::S, 270) => Facing::W,
                    (Facing::E, 270) => Facing::S,
                    (Facing::W, 270) => Facing::N,
                    _ => unreachable!(),
                }
            }
            Direction::R => {
                self.facing = match (self.facing, action.value % 360) {
                    (Facing::N, 90) => Facing::E,
                    (Facing::S, 90) => Facing::W,
                    (Facing::E, 90) => Facing::S,
                    (Facing::W, 90) => Facing::N,
                    (Facing::N, 180) => Facing::S,
                    (Facing::S, 180) => Facing::N,
                    (Facing::E, 180) => Facing::W,
                    (Facing::W, 180) => Facing::E,
                    (Facing::N, 270) => Facing::W,
                    (Facing::S, 270) => Facing::E,
                    (Facing::E, 270) => Facing::N,
                    (Facing::W, 270) => Facing::S,
                    _ => unreachable!(),
                }
            }
            Direction::F => match self.facing {
                Facing::N => self.north += action.value,
                Facing::S => self.north -= action.value,
                Facing::E => self.east += action.value,
                Facing::W => self.east -= action.value,
            },
        }
    }
}

struct Waypoint {
    east: i32,
    north: i32,
}

impl Waypoint {
    fn rotate_right(&mut self) {
        std::mem::swap(&mut self.east, &mut self.north);
        self.north *= -1;
    }

    fn rotate_left(&mut self) {
        std::mem::swap(&mut self.east, &mut self.north);
        self.east *= -1;
    }

    fn rotate_180(&mut self) {
        self.east *= -1;
        self.north *= -1;
    }
}

struct ShipV2 {
    waypoint: Waypoint,
    east: i32,
    north: i32,
}

impl ShipV2 {
    fn new() -> Self {
        Self {
            waypoint: Waypoint { east: 10, north: 1 },
            east: 0,
            north: 0,
        }
    }

    fn perform(&mut self, action: &Action) {
        match action.dir {
            Direction::N => self.waypoint.north += action.value,
            Direction::S => self.waypoint.north -= action.value,
            Direction::E => self.waypoint.east += action.value,
            Direction::W => self.waypoint.east -= action.value,
            Direction::L => match action.value % 360 {
                90 => self.waypoint.rotate_left(),
                180 => self.waypoint.rotate_180(),
                270 => self.waypoint.rotate_right(),
                _ => unreachable!(),
            },
            Direction::R => match action.value % 360 {
                90 => self.waypoint.rotate_right(),
                180 => self.waypoint.rotate_180(),
                270 => self.waypoint.rotate_left(),
                _ => unreachable!(),
            },
            Direction::F => {
                self.east += self.waypoint.east * action.value;
                self.north += self.waypoint.north * action.value;
            }
        }
    }
}

lazy_static! {
    static ref ACTIONS: Vec<Action> = include_str!("input.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
}

fn part1() {
    let mut ship = ShipV1::new();
    for action in &*ACTIONS {
        ship.perform(action);
    }
    println!("*   {}", ship.east.abs() + ship.north.abs());
}

fn part2() {
    let mut ship = ShipV2::new();
    for action in &*ACTIONS {
        ship.perform(action);
    }
    println!("**  {}", ship.east.abs() + ship.north.abs());
}

fn main() {
    part1();
    part2();
}
