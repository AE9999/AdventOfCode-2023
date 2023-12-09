use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;
use std::collections::VecDeque;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let problem = read_input(input)?;

    println!("{:?} is the lowest location number that corresponds to any of the initial seed numbers",
            solve1(&problem));

    println!("{:?} is the lowest location number that corresponds to any of the initial seed numbers",
             solve2(&problem));

    Ok(())
}

fn solve1(problem: &Problem) -> usize {
    let translations = [
        ("seed", "soil"),
        ("soil", "fertilizer"),
        ("fertilizer", "water"),
        ("water", "light"),
        ("light", "temperature"),
        ("temperature", "humidity"),
        ("humidity", "location"),
    ];

    problem.seeds.iter()
                 .map(|seed| {
                     let mut source = seed.clone();
                     for translation in translations {
                        let key = (String::from(translation.0),
                                              String::from(translation.1));
                        let problem_range_map = &(problem.maps.get(&key).unwrap());
                        let range = problem_range_map.ranges.iter().find(|range| {
                            &source >= &range.source && &range.source + &range.range >= source
                        });
                        if range.is_some() {
                            let range = range.unwrap();
                            source = (source - &range.source) +  &range.destination;
                        }
                     }
                     source
                 }).min()
                   .unwrap()
}

fn solve2(problem: &Problem) -> usize {

    #[derive(Debug, Clone)]
    struct State {
        lb: usize,
        ub: usize,
        translation_index: usize,
    }

    fn calculate_next_states(state: &State,
                             mut ranges: Vec<Range>) -> Vec<State> {

        let mut next_states: Vec<State> = Vec::new();

        let mut chunks_removed: Vec<(usize, usize)> = Vec::new();

        ranges.sort_by(|x,y|x.source.cmp(&y.source));

        for range in ranges {

            let lb = range.source.clone();

            let ub = range.source.clone() + range.range.clone();

            if (lb >= state.lb && lb <= state.ub)
               || (ub >= state.lb && ub <= state.ub)
               || (state.lb.clone() >= lb &&  state.lb.clone() <= ub)
               || (state.ub.clone() >= lb &&  state.ub.clone() <= ub) {

                let new_lb = max(lb, state.lb.clone());
                let new_ub = min(ub, state.ub.clone());
                let lb_diff = new_lb - lb.clone();

                chunks_removed.push((new_lb.clone(), new_ub.clone()));

                next_states.push(State {
                    lb: range.destination.clone() + lb_diff,
                    ub: range.destination.clone() + lb_diff.clone() + (new_ub - new_lb.clone()),
                    translation_index: state.translation_index.clone() + 1
                });
            }
        }

        let mut current_lb = state.lb.clone();
        for chunk_removed in chunks_removed {
            if chunk_removed.0 > current_lb {
                next_states.push(State {
                    lb: current_lb,
                    ub: chunk_removed.0 - 1,
                    translation_index: state.translation_index.clone() + 1
                });

            }

            current_lb = chunk_removed.1 + 1;
        }

        if current_lb < state.ub {
            next_states.push(State {
                lb: current_lb,
                ub: state.ub.clone(),
                translation_index: state.translation_index.clone() + 1
            })
        }

        next_states
    }

    let translations = [
        ("seed", "soil"),
        ("soil", "fertilizer"),
        ("fertilizer", "water"),
        ("water", "light"),
        ("light", "temperature"),
        ("temperature", "humidity"),
        ("humidity", "location"),
    ];

    let mut lowest_location_number = usize::MAX;

    let mut deque: VecDeque<State> = VecDeque::new();

    for state in problem.seeds
                              .chunks(2)
                              .map(|p|{
                                State {
                                    lb: (&p[0]).clone(),
                                    ub: (&p[0]).clone()+ (&p[1]).clone(),
                                    translation_index: 0
                                }}) {
        deque.push_back(state)
    }

    while !deque.is_empty() {

        let state = deque.pop_front().unwrap();

        if state.translation_index == translations.len() {
            lowest_location_number = min(lowest_location_number, state.lb)
        }  else {

            let translation = translations[state.translation_index];

            let key = (String::from(translation.0),
                                   String::from(translation.1));

            let ranges = (problem.maps.get(&key).unwrap()).ranges.clone();

            for state in calculate_next_states(&state, ranges) {
                deque.push_back(state)
            }
        }
    }

    lowest_location_number
}

#[derive(Debug, Clone)]
struct Problem {
    seeds: Vec<usize>,

    maps: HashMap<(String, String), ProblemRangeMap>
}

#[derive(Debug, Clone)]
struct ProblemRangeMap {
    ranges: Vec<Range>
}

#[derive(Debug, Clone)]
struct Range {
    destination: usize,
    source: usize,
    range: usize
}

fn read_input(filename: &String) ->  io::Result<Problem> {
    let file_in = File::open(filename)?;

    let mut it=
        BufReader::new(file_in).lines().map(|x|x.unwrap()).into_iter();

    let seeds_line: String = it.next().unwrap();

    let mut it_ = seeds_line.split(":");

    it_.next().unwrap();

    let seeds: Vec<usize> =
        it_.next()
          .unwrap()
          .trim()
          .split_whitespace()
          .map(|x|x.parse::<usize>().unwrap()).collect();

    it.next();

    let mut current_map: (String, String) = (String::from(""), String::from(""));
    let mut current_ranges: Vec<Range> = Vec::new();
    let mut maps: HashMap<(String, String), ProblemRangeMap> = HashMap::new();

    for line in it {

        if line.contains("-to-") {
            let s = line.split_whitespace().next().unwrap();
            let mut it = s.split("-to-");
            current_map.0 = String::from(it.next().unwrap());
            current_map.1 = String::from(it.next().unwrap());
        } else if line.is_empty() {
            maps.insert(current_map.clone(),
                        ProblemRangeMap {
                            ranges: current_ranges.clone()
                        });
            current_ranges.clear();
        } else {
            let mut s = line.split_whitespace();
            current_ranges.push( Range {
                                         destination: s.next().unwrap().parse::<usize>().unwrap(),
                                         source: s.next().unwrap().parse::<usize>().unwrap(),
                                         range: s.next().unwrap().parse::<usize>().unwrap()
                                     });
        }
    }

    maps.insert(current_map.clone(),
                ProblemRangeMap {
                    ranges: current_ranges.clone()
                });
    current_ranges.clear();

    Ok(Problem {
        seeds,
        maps
    })
}
