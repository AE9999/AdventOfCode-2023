use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let problem = read_input(input)?;

    println!("{:?} is what you get if you multiply these numbers together", solve1(&problem));

    let problem = read_input_part2(input)?;

    println!("{:?} is in how many ways you can solve the longer race", amount_of_ways_to_win(&problem));

    Ok(())
}

fn solve1(problem: &Problem) -> usize {

    problem.races.iter()
                 .map(|race| amount_of_ways_to_win(race))
                 .fold(1, |acc, x| acc * x)
}

fn wins(race: &Race, press_time: usize) -> bool {
    press_time < race.time
        && (&race.time - press_time) * press_time > race.distance
}

fn amount_of_ways_to_win(race: &Race) -> usize {
    (1..race.time).map(|press_time| if wins(race, press_time) { 1 } else { 0 })
        .sum()
}

struct Problem {
    races: Vec<Race>,
}

struct Race {
    time: usize,
    distance: usize
}

fn read_input(filename: &String) ->  io::Result<Problem> {
    let file_in = File::open(filename)?;

    let lines : Vec<String> =
        BufReader::new(file_in).lines().map(|x|x.unwrap()).collect();

    let mut it = lines.get(0).unwrap().split(":");
    it.next().unwrap();
    let times : Vec<usize> =
        it.next().unwrap().trim().split_whitespace().map(|x|x.parse::<usize>().unwrap()).collect();

    let mut it = lines.get(1).unwrap().split(":");
    it.next().unwrap();
    let distances : Vec<usize> =
        it.next().unwrap().trim().split_whitespace().map(|x|x.parse::<usize>().unwrap()).collect();

    let races : Vec<Race> = (0..(times.len())).into_iter().map(|i| Race {
        time: times[i],
        distance: distances[i]
    }).collect();

    Ok(Problem {
        races
    })
}

fn read_input_part2(filename: &String) ->  io::Result<Race> {

    let file_in = File::open(filename)?;

    let lines : Vec<String> =
        BufReader::new(file_in).lines().map(|x|x.unwrap()).collect();

    let mut it = lines.get(0).unwrap().split(":");
    it.next().unwrap();
    let time : usize =
        it.next().unwrap().trim().replace(" ", "").parse::<usize>().unwrap();

    let mut it = lines.get(1).unwrap().split(":");
    it.next().unwrap();
    let distance : usize =
        it.next().unwrap().trim().replace(" ", "").parse::<usize>().unwrap();

    Ok(Race {
        time,
        distance
    })
}
