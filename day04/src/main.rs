#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    str::Lines,
};

fn main() {
    // println!("==== Part 1 output ====");
    // part_1(read_to_string("day04/part1.input.txt").unwrap().lines());
    println!("\n==== Part 2 output ====");
    part_2(read_to_string("day04/part1.input.txt").unwrap().lines());
}

fn calculate_score(matching_numbers: usize) -> usize {
    if matching_numbers == 0 {
        0
    } else {
        // 2^0=1, 2^1=2, 2^2=4, 2^3=8, ...
        2_usize.pow((matching_numbers - 1) as u32)
    }
}

fn part_1(lines: Lines) {
    let mut total = 0;

    for line in lines {
        let mut numbers = line.split(" | ").map(|part| {
            part.split(" ")
                .filter(|c| return *c != "Card" && *c != "" && !c.ends_with(':'))
                .collect::<Vec<&str>>()
        });
        let winning_numbers: HashSet<&str> = HashSet::from_iter(numbers.next().unwrap());
        let our_numbers: HashSet<&str> = HashSet::from_iter(numbers.next().unwrap());
        let winning_number_count = our_numbers.intersection(&winning_numbers).count();
        let score = calculate_score(winning_number_count);
        total += score;
        // println!("{winning_numbers:?}\t{our_numbers:?}\t{winning_number_count} -> {score}");
    }

    println!("total: {total}");
}

fn part_2(lines: Lines) {
    let mut card_matches: HashMap<usize, usize> = HashMap::new();
    let mut cards_to_process: HashMap<usize, usize> = HashMap::new();
    let mut line_amount = 0;

    for (idx, line) in lines.enumerate() {
        line_amount += 1;

        let mut numbers = line.split(" | ").map(|part| {
            part.split(" ")
                .filter(|c| return *c != "Card" && *c != "" && !c.ends_with(':'))
                .collect::<Vec<&str>>()
        });

        let winning_numbers: HashSet<&str> = HashSet::from_iter(numbers.next().unwrap());
        let our_numbers: HashSet<&str> = HashSet::from_iter(numbers.next().unwrap());
        let winning_number_count = our_numbers.intersection(&winning_numbers).count();
        card_matches.entry(idx).or_insert(winning_number_count);
        println!("[{idx}]\t{winning_number_count}");

        let additional = match cards_to_process.get(&idx) {
            Some(amount) => *amount,
            None => 0,
        };
        println!("we should process {additional} additional entries of {idx}");

        println!("we got {winning_number_count} cards after {idx}");
        let bonus = 1 * (additional + 1);
        for card_to_process in (idx + 1)..(idx + winning_number_count + 1) {
            *cards_to_process.entry(card_to_process).or_default() += bonus;
            println!("{card_to_process}++");
        }

        println!();
    }

    // Total cards that were processed + amount of initial cards
    let to_process_sum = cards_to_process.values().sum::<usize>() + line_amount;
    println!("to process: {cards_to_process:?} -> {to_process_sum}");
}
