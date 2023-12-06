#![allow(dead_code)]
#![allow(unused_variables)]

use std::{fs::read_to_string, ops::Range, str::Lines};

fn main() {
    // println!("==== Part 1 output ====");
    // part_1(read_to_string("day06/part1.input.txt").unwrap().lines());
    println!("\n==== Part 2 output ====");
    part_2(read_to_string("day06/part1.input.txt").unwrap().lines());
}

#[derive(Debug)]
struct Race {
    time: usize,
    record: usize,
}

impl Race {
    fn hold_times(&self) -> Range<usize> {
        0..(self.time + 1)
    }

    fn calculate_distance(&self, hold_for: usize) -> usize {
        let time_left = self.time - hold_for;
        time_left * hold_for
    }
}

fn part_1(mut lines: Lines) {
    let times = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|it| it.parse::<usize>())
        .filter_map(|num| num.ok());
    let records = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|it| it.parse::<usize>())
        .filter_map(|num| num.ok());
    // println!("{:?}", times.clone().collect::<Vec<_>>());
    // println!("{:?}", records.clone().collect::<Vec<_>>());

    let races = times
        .zip(records)
        .map(|(time, record)| Race { time, record });

    let result = races
        .map(|r| {
            r.hold_times()
                .map(|hold_for| r.calculate_distance(hold_for))
                // Find hold times that will result in a distance greater than the record
                .filter(|distance| *distance > r.record)
                .count()
        })
        // Multiply all amounts of possible hold times together
        .product::<usize>();
    println!("{result}");
}

fn part_2(mut lines: Lines) {
    let time = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .flat_map(|p| p.chars())
        .collect::<String>()
        .parse()
        .expect("Time should be a valid number");
    let record = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .flat_map(|p| p.chars())
        .collect::<String>()
        .parse()
        .expect("Record should be a valid number");
    // println!("{:?}", time);
    // println!("{:?}", record);

    let race = Race { time, record };
    let result = race
        .hold_times()
        .map(|hold_for| race.calculate_distance(hold_for))
        .filter(|distance| *distance > race.record)
        .count();
    println!("{result}");
}
