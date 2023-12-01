use std::fs::read_to_string;

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

fn main() {
    let input = read_to_string("src/1_2_input.txt").unwrap();
    let lines = input.lines();

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
            .filter(|(idx, _)| idx.is_some())
            .map(|(idx, num)| (idx.unwrap(), num))
            .min_by_key(|(idx, _)| *idx)
            .unwrap();
        let first_digit =
            to_number(&line[first_digit_index..first_digit_index + first_digit.len()]);

        let (last_digit_index, last_digit) = numbers
            .iter()
            .map(|num| (line.rfind(num), num))
            .filter(|(idx, _)| idx.is_some())
            .map(|(idx, num)| (idx.unwrap(), num))
            .max_by_key(|(idx, _)| *idx)
            .unwrap();
        let last_digit = to_number(&line[last_digit_index..last_digit_index + last_digit.len()]);

        total += first_digit * 10 + last_digit;
        // println!("first:{first_digit}, last:{last_digit}");
    }

    println!("total: {total}");
}
