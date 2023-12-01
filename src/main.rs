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
        println!("{line}");

        let mut first_digit_index = line.len();
        let mut first_digit: Option<u32> = None;
        for number in numbers.clone() {
            match line.find(number) {
                Some(index) => {
                    if index <= first_digit_index {
                        first_digit_index = index;
                        let a = &line[index..index + number.len()];
                        let b = to_number(a);
                        first_digit = Some(b);
                        // println!("found:{a}, parsed:{b}");
                    }
                }
                None => continue,
            }
        }

        let mut last_digit_index = 0;
        let mut last_digit: Option<u32> = None;
        for number in numbers.clone() {
            match line.rfind(number) {
                Some(index) => {
                    if index >= last_digit_index {
                        last_digit_index = index;
                        let a = &line[index..index + number.len()];
                        let b = to_number(a);
                        last_digit = Some(b);
                        // println!("found:{a}, parsed:{b}");
                    }
                }
                None => continue,
            }
        }

        let first = first_digit.unwrap();
        let last = last_digit.unwrap();
        let res = first * 10 + last;
        total += res;
        // println!("first:{first}, last:{last}, res:{res}");
    }

    println!("total: {total}");
}
