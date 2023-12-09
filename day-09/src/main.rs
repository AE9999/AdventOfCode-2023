use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let problem = read_input(input)?;

    println!("{:?} is the sum of these extrapolated values",
             solve1(&problem));

    println!("{:?} is the sum of these extrapolated values",
             solve2(&problem));

    Ok(())
}

fn solve2(problem: &crate::Problem) -> i64 {
    fn solve2_h(vec: Vec<i64>) -> i64 {
        let next_values: Vec<i64> =
            vec.iter()
                .enumerate()
                .skip(1)
                .map(|(index, value)| value - vec.get(index - 1).unwrap() )
                .collect();
        let answer =
            if next_values.iter().find(|x| x != &&0).is_some() {
                let p = solve2_h(next_values);
                *vec.first().unwrap() - p
            } else {
                *vec.first().unwrap()
            };
        answer
    }

    problem.histories.iter().map(|x| {
        solve2_h(x.clone())
    }).sum()
}

fn solve1(problem: &Problem) -> i64 {
    fn solve1_h(vec: Vec<i64>) -> i64 {
        let next_values: Vec<i64> =
            vec.iter()
                .enumerate()
                .skip(1)
                .map(|(index, value)| value - vec.get(index - 1).unwrap() )
                .collect();
        let answer =
            if next_values.iter().find(|x| x != &&0).is_some() {
                let p = solve1_h(next_values);
                *vec.last().unwrap() + p
            } else {
                *vec.last().unwrap()
            };
        answer
    }

    problem.histories.iter().map(|x| {
        solve1_h(x.clone())
    }).sum()
}

struct Problem {
    histories: Vec<Vec<i64>>,
}


fn read_input(filename: &String) ->  io::Result<Problem> {
    let file_in = File::open(filename)?;
    let histories: Vec<Vec<i64>> =
        BufReader::new(file_in).lines().map(|x|{
            x.unwrap().split_whitespace().map(|x|{x.parse::<i64>().unwrap()}).collect()
        }).collect();
    Ok(Problem {
        histories
    })
}
