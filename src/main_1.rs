use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/1_input.txt").unwrap();
    let lines = input.lines();

    let mut total = 0;

    for line in lines {
        // println!("{line}");

        // FIXME: How can we unwrap all Some(d) without doing .filter().map()?
        let digits = line
            .chars()
            .map(|c| c.to_digit(10))
            .filter(|d| d.is_some())
            .map(|d| d.unwrap());

        let first_digit = digits.clone().next().unwrap();
        let last_digit = digits.clone().rev().next().unwrap();

        total += first_digit * 10 + last_digit;
        // println!("first:{first_digit}, last:{last_digit}");
    }

    println!("total: {total}");
}
