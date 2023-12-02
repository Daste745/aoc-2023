use std::{fs::read_to_string, str::Lines};

fn main() {
    println!("==== Part 1 output ====");
    part_1(read_to_string("day02/part1.input.txt").unwrap().lines());
    println!("\n==== Part 2 output ====");
    part_2(read_to_string("day02/part1.input.txt").unwrap().lines());
}

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn parse_cube_sets(results_str: &str) -> Vec<Vec<(u32, &str)>> {
    results_str
        .split("; ")
        // "3 blue, 4 red"
        .map(|set| {
            set.split(", ")
                // "3 blue"
                .map(|color_str| {
                    let mut color_str_parts = color_str.split(' ');
                    let color_amount = color_str_parts.next().unwrap().parse::<u32>().unwrap();
                    let color_name = color_str_parts.next().unwrap();
                    (color_amount, color_name)
                })
                .collect()
        })
        .collect()
}

fn part_1(lines: Lines) {
    let mut total = 0;

    for line in lines {
        let line_parts: Vec<&str> = line.split(": ").collect();
        let game_number = line_parts
            .first()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let cube_sets = parse_cube_sets(line_parts.last().unwrap());

        let mut game_possible = true;
        for set in cube_sets {
            // println!("{set}");
            game_possible = set.iter().all(|(color_amount, color_name)| {
                let max = match *color_name {
                    "red" => MAX_RED,
                    "green" => MAX_GREEN,
                    "blue" => MAX_BLUE,
                    _ => panic!(),
                };
                *color_amount <= max
            });
            if !game_possible {
                break;
            }
        }

        if game_possible {
            // println!("Game {game_number} possible");
            total += game_number;
        } else {
            // println!("Game {game_number} impossible");
        }
    }

    println!("total: {total}");
}

fn part_2(lines: Lines) {
    let mut total = 0;

    for line in lines {
        let line_parts: Vec<&str> = line.split(": ").collect();
        let cube_sets = parse_cube_sets(line_parts.last().unwrap());

        let mut max_red: u32 = 0;
        let mut max_green: u32 = 0;
        let mut max_blue: u32 = 0;

        for set in cube_sets {
            // println!("{set}");
            for (color_amount, color_name) in set {
                let max_color = match color_name {
                    "red" => &mut max_red,
                    "green" => &mut max_green,
                    "blue" => &mut max_blue,
                    _ => panic!(),
                };
                if color_amount > *max_color {
                    *max_color = color_amount;
                }
            }
        }
        let minimum_set_power = max_red * max_green * max_blue;
        total += minimum_set_power;
    }

    println!("total: {total}");
}
