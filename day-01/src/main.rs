use std::cmp::min;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let input = read_input(input)?;

    let answer: u32 =
        input.iter().map(|x| x.chars().filter(|x| x.is_digit(10)).next().unwrap().to_digit(10).unwrap() * 10 +
                                     x.chars().filter(|x| x.is_digit(10)).last().unwrap().to_digit(10) .unwrap()).sum();

    println!("{:?}  is the sum of all of the calibration values", answer);

    let answer: i32 =
        input.iter().map(|x| handle_input_line(x)).sum();

    println!("{:?}  is the sum of all of the calibration values", answer);

    Ok(())
}

fn handle_input_line(input_line: &String) -> i32 {
    let digit_entries = [
        ("1", "one", 1),
        ("2", "two", 2),
        ("3", "three", 3),
        ("4", "four", 4),
        ("5" , "five", 5),
        ("6", "six", 6),
        ("7", "seven", 7),
        ("8", "eight", 8),
        ("9", "nine", 9)
    ];

    let mut first: Vec<(usize, i32)> =
        digit_entries.iter()
                     .map(|digit_entry| calculate_first_digit(input_line, digit_entry)).collect();

    first.sort_by(|x,y|x.0.cmp(&y.0));

    let first = first.iter().next().unwrap().1.clone();

    let mut last: Vec<(usize, i32)>  =
        digit_entries.iter()
            .map(|digit_entry| calculate_last_digit(input_line, digit_entry)).collect();

    last.sort_by(|x,y|x.0.cmp(&y.0));

    let last = last.iter().next().unwrap().1.clone();

    (first * 10) + last
}

fn calculate_first_digit(input: &String,
                         digit_entry: &(&str, &str, i32)) -> (usize, i32) {
    let digit_location = input.find(digit_entry.0).unwrap_or(input.len());
    let string_location = input.find(digit_entry.1).unwrap_or(input.len());
    (min(digit_location, string_location), digit_entry.2.clone())
}

fn calculate_last_digit(input: &String,
                         digit_entry: &(&str, &str, i32)) -> (usize, i32) {
    let reversed_input: String = input.chars().rev().collect();
    let reversed_string: String = digit_entry.1.chars().rev().collect();

    let digit_location = reversed_input.find(digit_entry.0).unwrap_or(input.len());
    let string_location = reversed_input.find(reversed_string.as_str()).unwrap_or(input.len());

    (min(digit_location, string_location), digit_entry.2.clone())
}

fn read_input(filename: &String) ->  io::Result<Vec<String>> {
    let file_in = File::open(filename)?;
    Ok(BufReader::new(file_in).lines().map(|x| x.unwrap()).collect())
}
