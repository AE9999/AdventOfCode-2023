use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let problem = read_input(input)?;

    println!("What is the sum of those counts? {:?}", solve1(&problem));

    println!("What is the new sum of possible arrangement counts? {:?}", solve2(&problem));

    Ok(())
}

fn solve1(problem: &Problem) -> usize {
    problem.configurations.iter()
                          .map(|x| { x.possible_configurations() } ).sum()
}

fn solve2(problem: &Problem) -> usize {
    problem.configurations.iter()
        .map(|x| { x.possible_fixed_configurations() } ).sum()
}

struct Problem {
    configurations: Vec<Configuration>
}

struct Configuration {
    chars: Vec<char>,
    groups: Vec<usize>
}

impl Configuration {
    fn possible_configurations(&self) -> usize {
        let x = Configuration::possible_configurations_h(self.chars.clone(),
                                                                     &self.groups);
        // println!("x: {:?}", x);
        x
    }

    fn possible_fixed_configurations(&self) -> usize {
        let mut chars: Vec<char> = Vec::new();
        let mut groups: Vec<usize> =  Vec::new();

        chars.extend(self.chars.iter());
        groups.extend(self.groups.iter());

        (0..4).for_each(|_|{
            chars.push('?');
            chars.extend(self.chars.iter());
            groups.extend(self.groups.iter());
        });

        let s : String = chars.iter().collect();
        // println!("chars: {:?}, groups: {:?}", s, groups);

        let x = Configuration::possible_configurations_h(chars,
                                                               &groups);
        println!("x: {:?}", x);
        x
    }

    fn possible_configurations_h(chars: Vec<char>,
                                 groups: &Vec<usize>) -> usize {

        if !Configuration::is_still_feasible(&chars,
                                             groups) {
            let s : String = chars.iter().collect();
            println!("{:?} is no longer feasible", s);
            return 0
        }

        if Configuration::is_fixed(&chars) {
            let s : String = chars.iter().collect();
            println!("{:?} is feasible", s);
            return 1
        }

        chars.iter().enumerate()
                    .find(|(_, char)| {
                        char == &&'?'
                    })
                    .map(|(index, _)| {
                        // You need to take a greedy position based on where you are ..
                        let mut with_point = chars.clone();
                        let mut with_block = chars.clone();
                        with_point[index] = '.';
                        with_block[index] = '#';
                        Configuration::possible_configurations_h(with_block, groups) +
                        Configuration::possible_configurations_h(with_point, groups)
                    }).unwrap()
    }

    fn is_still_feasible(chars: &Vec<char>,
                         groups: &Vec<usize>) -> bool {
        let mut counts: Vec<usize> = Vec::new();
        let mut count: usize = 0;
        let mut indeterminate_state_found = false;

        for ch in chars {
            if ch == &'#' {
                count += 1;
            } else if ch == &'.'  {
                if count > 0 {
                    counts.push(count);
                    count = 0;
                }
            } else {
                assert!(ch == &'?');
                indeterminate_state_found = true;
                break;
            }
        }

        // println!("chars: {:?}, counts: {:?}, groups: {:?}, count: {:?}, indeterminate_state_found:{:?}",
        //          chars, counts, groups, count, indeterminate_state_found);

        for (index, cnt) in counts.iter().enumerate() {
            if groups.get(index).is_none()
                || cnt != groups.get(index).unwrap() {
                return false
            }
        }

        let i: usize = counts.len();

        if indeterminate_state_found {
            if count > 0 {
                groups.get(i).is_some() && groups[i] >= count
            }
            else {
                true
            }
        } else {
            &Configuration::map_hash_groups(chars) == groups
        }
    }

    fn map_hash_groups(chars: &Vec<char>) -> Vec<usize> {
        let mut counts = Vec::new();
        let mut count = 0;

        for ch in chars {
            if ch == &'#' {
                count += 1;
            } else if count > 0 {
                counts.push(count);
                count = 0;
            }
        }

        if count > 0 {
            counts.push(count);
        }

        counts
    }

    fn is_fixed(chars: &Vec<char>) -> bool {
        chars.iter().find(|c| c == &&'?').is_none()
    }
}

fn read_input(filename: &String) ->  io::Result<Problem> {
    let file_in = File::open(filename)?;

    let configurations: Vec<Configuration> =
        BufReader::new(file_in).lines()
                                     .map(|x| { x.unwrap() })
                                     .map(|line|{
                                        let mut it = line.split_whitespace();
                                         let chars: Vec<char> = it.next().unwrap().chars().collect();
                                         let groups : Vec<usize> = it.next().unwrap().split(",").map(|x| x.parse::<usize>().unwrap()).collect();
                                         Configuration {
                                             chars,
                                             groups
                                         }
                                      })
                                     .collect();


    Ok(Problem {
        configurations
    })
}
