use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let puzzle = read_input(input)?;

    println!("{:?} is the sum of all of the part numbers in the engine schematic",
             part_1(&puzzle));

    println!("{:?} is the sum of all of the gear ratios in your engine schematic?",
            part_2(&puzzle));

    Ok(())
}

fn part_1(input: &Vec<Vec<char>>) -> i32 {

    read_spans(input).iter().filter(|number_span| {
        // Find any digit in the range
        let y = number_span.2.clone();
        let x1 = number_span.0.clone();
        let x2 = number_span.1.clone();

        for y_ in (y.clone()-1)..(y.clone()+2) {
            if y_ < 0 || y_ >= (input.len() as i32) {
                continue;
            }
            for x_ in (x1.clone()-1)..(x2.clone()+2) {
                let y_ = y_.clone() as usize;
                if x_ < 0 || x_ >= (input[0].len() as i32) {
                    continue;
                }
                let symbol_candidate = &(&input[y_])[x_ as usize];
                if !symbol_candidate.is_digit(10)
                    && symbol_candidate != &'.' {
                    return true;
                }
            }
        }
        false
    }).map(|number_span| {
        span2number(input, number_span)
    }).sum()
}

fn part_2(input: &Vec<Vec<char>>) -> i32 {

    let number_spans = read_spans(input);

    let mut gear_ratios = 0;

    for y in 0..input.len() {
        let row = &input[y];
        for x in 0..row.len() {

            if (&input[y.clone()])[x] != '*' {
                continue
            }

            let matching_spans : Vec<(i32, i32, i32)> =
                number_spans.iter()
                            .filter(|span| {
                                let x_cor = x.clone() as i32;
                                let y_cor = y.clone() as i32;

                                y_cor >= (span.2.clone() - 1)
                                && y_cor <= (span.2.clone() + 1)
                                && x_cor >= (span.0.clone()) - 1
                                && x_cor <= (span.1.clone()) + 1

                            })
                            .map(|span| span.clone()).collect();

            if matching_spans.len() == 2 {
                gear_ratios += span2number(input, &matching_spans[0])
                               * span2number(input, &matching_spans[1])
            }
        }
    }

    gear_ratios
}

fn read_spans(input: &Vec<Vec<char>>) -> Vec<(i32, i32, i32)> {
    let mut number_spans: Vec<(i32, i32, i32)> = Vec::new();

    for y in 0..input.len() {
        let row = &input[y];
        let mut last_processing_number_index  = if row[0].is_digit(10) { 0 } else { -1 } ;

        // Fix this
        for x in 1..row.len() {
            if !row[x.clone()].is_digit(10) && last_processing_number_index > -1 {
                number_spans.push((last_processing_number_index,
                                   (x.clone() - 1) as i32,
                                   y.clone() as i32));
                last_processing_number_index = -1
            } else if row[x.clone()].is_digit(10)  && last_processing_number_index == -1 {
                last_processing_number_index = x.clone() as i32
            }
        }

        if last_processing_number_index > 0 {
            number_spans.push((last_processing_number_index,
                               (row.len() -1) as i32,
                               y.clone() as i32));
        }
    }

    return number_spans
}

fn span2number(input: &Vec<Vec<char>>,
               number_span: &(i32, i32, i32)) -> i32 {
    let y = number_span.2.clone() as usize;
    let x1: usize = number_span.0.clone() as usize;
    let x2: usize = (number_span.1.clone()  as usize) + 1;
    let value : String = (&input[y])[x1..x2].iter().collect();
    value.parse::<i32>().unwrap()
}

fn read_input(filename: &String) ->  io::Result<Vec<Vec<char>>> {
    let file_in = File::open(filename)?;
    Ok(BufReader::new(file_in).lines().map(|x| x.unwrap().chars().collect()).collect())
}
