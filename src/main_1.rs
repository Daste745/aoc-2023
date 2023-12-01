use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/1_input.txt").unwrap();
    let lines = input.lines();
    // let lines = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
    let mut first_digit: Option<u32> = None;
    let mut last_digit: Option<u32> = None;

    let mut total = 0;

    for line in lines {
        // println!("{line}");
        for c in line.chars() {
            if let Some(digit) = c.to_digit(10) {
                first_digit = Some(digit);
                break;
            }
        }
        for c in line.chars().rev() {
            if let Some(digit) = c.to_digit(10) {
                last_digit = Some(digit);
                break;
            }
        }

        let first = first_digit.unwrap();
        let last = last_digit.unwrap();
        let res = first * 10 + last;
        total += res
        // println!("first:{first}, last:{last}, res:{res}");
    }

    println!("total: {total}");
}
