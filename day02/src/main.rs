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

fn part_1(lines: Lines) {
    let mut total = 0;

    for line in lines {
        let parts: Vec<&str> = line.split(':').collect();

        let game_number = parts
            .first()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        // ["3 blue, 4 red", "1 red, 2 green, 6 blue", "2 green"]
        let cube_sets = parts
            .last()
            .unwrap()
            .split(';')
            .map(|set| set.strip_prefix(' ').unwrap())
            .collect::<Vec<&str>>();

        let mut game_possible = true;
        for set in cube_sets {
            // println!("{set}");
            let colors = set.split(", ").collect::<Vec<&str>>();
            for color in colors {
                let color_amount = color.split(' ').next().unwrap().parse::<u32>().unwrap();
                let color_name = color.split(' ').last().unwrap();
                let max = match color_name {
                    "red" => MAX_RED,
                    "green" => MAX_GREEN,
                    "blue" => MAX_BLUE,
                    _ => panic!(),
                };
                if color_amount > max {
                    game_possible = false;
                    break;
                }
            }
            if !game_possible {
                break;
            };
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
        let parts: Vec<&str> = line.split(": ").collect();

        // ["3 blue, 4 red", "1 red, 2 green, 6 blue", "2 green"]
        let cube_sets = parts
            .last()
            .unwrap()
            .split("; ")
            // [["3 blue", "4 red"], ["1 red", "2 green", "6 blue"], ["2 green"]]
            .map(|set| set.split(", ").collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();

        let mut max_red: u32 = 0;
        let mut max_green: u32 = 0;
        let mut max_blue: u32 = 0;

        for set in cube_sets {
            // println!("{set}");
            for color in set {
                // "10 red"
                let color_amount = color.split(' ').next().unwrap().parse::<u32>().unwrap();
                let color_name = color.split(' ').last().unwrap();
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
