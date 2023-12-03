#![allow(dead_code)]
#![allow(unused_variables)]

use std::{collections::HashSet, fs::read_to_string, str::Lines};

fn main() {
    println!("==== Part 1 output ====");
    part_1(read_to_string("day03/part1.input.txt").unwrap().lines());
    println!("\n==== Part 2 output ====");
    part_2(read_to_string("day03/part1.input.txt").unwrap().lines());
}

/// Find all numbers and their indexes in a string
fn find_numbers(line: &&str) -> Vec<(usize, u32)> {
    let mut parsed_numbers: Vec<(usize, u32)> = Vec::new();
    let mut digits: Vec<u32> = Vec::new();

    fn parse_digit(digits: &Vec<u32>) -> u32 {
        let mut parsed_number = 0;
        for digit in digits {
            parsed_number *= 10;
            parsed_number += digit;
        }
        parsed_number
    }

    for (idx, c) in line.char_indices() {
        if let Some(digit) = c.to_digit(10) {
            digits.push(digit);
        } else if digits.len() > 0 {
            parsed_numbers.push((idx - digits.len(), parse_digit(&digits)));
            digits.clear();
        }
    }
    if digits.len() > 0 {
        parsed_numbers.push((line.len() - digits.len(), parse_digit(&digits)));
        digits.clear();
    }

    parsed_numbers
}

fn part_1(lines: Lines) {
    let engine: Vec<&str> = lines.collect();
    let mut total = 0;

    for (row, line) in engine.iter().enumerate() {
        for (col, number) in find_numbers(line) {
            // println!("row:{row} number:{number} at:{col}");

            let mut lines_to_check: Vec<&str> = Vec::new();
            lines_to_check.push(line);
            if row > 0 {
                if let Some(line_above) = engine.iter().skip(row - 1).next() {
                    lines_to_check.push(line_above);
                }
            }
            if row < engine.len() {
                if let Some(line_below) = engine.iter().skip(row + 1).next() {
                    lines_to_check.push(line_below);
                }
            }

            let mut adjacent_chars: HashSet<char> = HashSet::new();
            let left_bound = if col > 0 { col - 1 } else { col };
            let number_chars = number.to_string().chars().collect::<Vec<char>>();
            for l in lines_to_check {
                l.chars()
                    .skip(left_bound)
                    .take(number_chars.len() + 2)
                    .filter(|c| !number_chars.contains(c) && *c != '.')
                    .for_each(|c| {
                        adjacent_chars.insert(c);
                    });
            }

            // println!("adjacent: {adjacent_chars:?}");
            if adjacent_chars.len() > 0 {
                total += number
            }
        }
    }

    println!("total: {total}");
}

#[derive(Debug)]
struct Gear {
    pos_y: usize,
    pos_x: usize,
    numbers: Vec<u32>,
}

impl Gear {
    fn new(pos_y: usize, pos_x: usize, number: u32) -> Self {
        Self {
            pos_y,
            pos_x,
            numbers: vec![number],
        }
    }

    fn push_number(&mut self, number: u32) {
        self.numbers.push(number);
    }

    fn is_valid(&self) -> bool {
        self.numbers.len() == 2
    }

    fn ratio(&self) -> u32 {
        self.numbers.iter().product()
    }
}

fn part_2(lines: Lines) {
    let engine: Vec<&str> = lines.collect();
    let mut gears: Vec<Gear> = Vec::new();

    for (row, line) in engine.iter().enumerate() {
        for (col, number) in find_numbers(line) {
            // println!("row:{row} number:{number} at:{col}");

            // Save lines with their indexes so we know positions of Gears
            let mut lines_to_check: Vec<(usize, &str)> = Vec::new();
            lines_to_check.push((row, line));
            if row > 0 {
                if let Some(line_above) = engine.iter().skip(row - 1).next() {
                    lines_to_check.push((row - 1, line_above));
                }
            }
            if row < engine.len() {
                if let Some(line_below) = engine.iter().skip(row + 1).next() {
                    lines_to_check.push((row + 1, line_below));
                }
            }

            let mut adjacent_chars: HashSet<char> = HashSet::new();
            let left_bound = if col > 0 { col - 1 } else { col };
            let number_chars = number.to_string().chars().collect::<Vec<char>>();
            for (i, l) in lines_to_check {
                let chars_to_check = l
                    .chars()
                    .enumerate()
                    .skip(left_bound)
                    .take(number_chars.len() + 2);
                for (j, c) in chars_to_check {
                    if number_chars.contains(&c) || c == '.' {
                        continue;
                    }
                    adjacent_chars.insert(c);
                    if c == '*' {
                        if let Some(gear) = gears.iter_mut().find(|c| c.pos_y == i && c.pos_x == j)
                        {
                            gear.push_number(number);
                        } else {
                            gears.push(Gear::new(i, j, number));
                        }
                    }
                }
            }

            // println!("adjacent: {adjacent_chars:?}");
        }
    }

    let total: u32 = gears.iter().filter(|c| c.is_valid()).map(Gear::ratio).sum();
    println!("total: {total}");
}
