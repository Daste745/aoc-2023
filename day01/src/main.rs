use std::{fs::read_to_string, str::Lines};

fn main() {
    println!("==== Part 1 output ====");
    part_1(read_to_string("day01/part1.input.txt").unwrap().lines());
    println!("\n==== Part 2 output ====");
    part_2(read_to_string("day01/part2.input.txt").unwrap().lines());
}

fn to_number(s: &str) -> u32 {
    match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => {
            return s.chars().next().unwrap().to_digit(10).unwrap();
        }
    }
}

fn part_1(lines: Lines) {
    let mut total = 0;

    for line in lines {
        // println!("{line}");

        let digits = line.chars().map(|c| c.to_digit(10)).filter_map(|d| d);
        let first_digit = digits.clone().next().unwrap();
        let last_digit = digits.clone().rev().next().unwrap();

        total += first_digit * 10 + last_digit;
        // println!("first:{first_digit}, last:{last_digit}");
    }

    println!("total: {total}");
}

fn part_2(lines: Lines) {
    let mut total = 0;

    let numbers = vec![
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine",
    ];

    for line in lines {
        // println!("{line}");

        let (first_digit_index, first_digit) = numbers
            .iter()
            .map(|num| (line.find(num), num))
            .filter_map(|(idx, num)| idx.map(|idx| (idx, num)))
            .min_by_key(|(idx, _)| *idx)
            .unwrap();
        let first_digit =
            to_number(&line[first_digit_index..first_digit_index + first_digit.len()]);

        let (last_digit_index, last_digit) = numbers
            .iter()
            .map(|num| (line.rfind(num), num))
            .filter_map(|(idx, num)| idx.map(|idx| (idx, num)))
            .max_by_key(|(idx, _)| *idx)
            .unwrap();
        let last_digit = to_number(&line[last_digit_index..last_digit_index + last_digit.len()]);

        total += first_digit * 10 + last_digit;
        // println!("first:{first_digit}, last:{last_digit}");
    }

    println!("total: {total}");
}
