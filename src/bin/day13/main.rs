use lazy_static::lazy_static;

lazy_static! {
    static ref INPUT: (u32, Vec<u32>) = {
        let mut lines = include_str!("input.txt").lines();
        let timestamp = lines.next().unwrap().parse().unwrap();
        let buses = lines
            .next()
            .unwrap()
            .split(',')
            .flat_map(|id| id.parse())
            .collect();
        (timestamp, buses)
    };
}

fn part1() {
    let (timestamp, buses) = &*INPUT;
    let (bus, time) = buses
        .iter()
        .map(|&bus| (bus, bus - (timestamp % bus)))
        .min_by_key(|&(bus, time)| if bus == time { 0 } else { time })
        .unwrap();
    println!("*   {} ({} * {})", bus * time, bus, time);
}

fn main() {
    part1();
}
