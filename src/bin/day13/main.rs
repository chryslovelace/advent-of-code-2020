use lazy_static::lazy_static;

lazy_static! {
    static ref INPUT: (u32, Vec<Option<u32>>) = {
        let mut lines = include_str!("input.txt").lines();
        let timestamp = lines.next().unwrap().parse().unwrap();
        let buses = lines
            .next()
            .unwrap()
            .split(',')
            .map(|id| id.parse().ok())
            .collect();
        (timestamp, buses)
    };
}

fn part1() {
    let (timestamp, buses) = &*INPUT;
    let (bus, time) = buses
        .iter()
        .flatten()
        .map(|&bus| (bus, bus - (timestamp % bus)))
        .min_by_key(|&(bus, time)| if bus == time { 0 } else { time })
        .unwrap();
    println!("*   {} ({} * {})", bus * time, bus, time);
}

fn part2() {
    let (_, buses) = &*INPUT;
    let mut buses = buses
        .iter()
        .enumerate()
        .flat_map(|(i, bus)| bus.map(|bus| (i as i128, bus as i128)));
    let (idx, bus) = buses.next().unwrap();
    let mut timestamp = bus - idx;
    let mut step = bus;
    for (idx, bus) in buses {
        while (timestamp + idx) % bus != 0 {
            timestamp += step;
        }
        step = num::integer::lcm(step, bus);
    }
    println!("**  {}", timestamp);
}

fn main() {
    part1();
    part2();
}
