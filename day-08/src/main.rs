use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let problem = read_input(input)?;
    println!("{:?} steps are required to reach ZZZ", solve1(&problem));

    println!("{:?} steps does it take before you're only on nodes that end with Z", solve2(&problem));
    Ok(())
}

fn solve1(problem: &Problem) -> usize {
    let mut step: usize = 0;

    let mut position = String::from("AAA");

    while position != String::from("ZZZ") {

        let instruction =
            problem.instructions.get(step % problem.instructions.len()).unwrap();

        let option =
            problem.options.get(&position).unwrap();

        position =
            if instruction == &'L' {
                option.0.clone()
            } else {
                option.1.clone()
            };

        step += 1
    }
    step
}

fn solve2(problem: &Problem) -> usize {

    /* Not going to lie, I did have to look this Least Common Multiple shit up */
    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    fn lcm(a: usize, b: usize) -> usize {
        a / gcd(a, b) * b
    }

    fn lcm_of_values(arr: Vec<usize>) -> usize {
        arr.iter().fold(1, |acc, &num| lcm(acc, num))
    }

    let positions: HashSet<String> =
        HashSet::from_iter(problem.options.keys()
                                              .filter(|x| {x.chars().nth(2).unwrap() == 'A'})
                                              .map(|x|x.clone()));

    /*for position in positions {
        println!("Trying to find the possible end_states for {:?}", position);
        for pes  in find_possible_end_states(problem, position) {
            println!("\tFound {:?} ..", pes)
        }
    }*/

    // This won't work if I return multiple state, but fuck it.
    let values: Vec<usize> =
        positions.iter()
                 .map(|position|
                    find_possible_end_states(problem,
                                             position.clone())).into_iter()
                                                                         .map(|x|x.iter().next().unwrap().1)
                                                                         .collect();
    lcm_of_values(values)
}

fn find_possible_end_states(problem: &Problem,
                            begin_state: String) -> Vec<(String, usize)>  {

    #[derive(Clone, Debug, Eq, PartialEq, Hash)]
    struct State {
        position: String,
        instruction_index: usize
    }

    let end_positions: HashSet<String> =
        HashSet::from_iter(problem.options.keys()
            .filter(|x| {x.chars().nth(2).unwrap() == 'Z'})
            .map(|x|x.clone()));

    let mut seen_states : HashSet<State> = HashSet::new();

    let mut possible_end_states : Vec<(String, usize)> = Vec::new();

    let mut step: usize = 0;

    let mut position = begin_state;

    loop {
        let instruction_index = step % problem.instructions.len();

        let state = State {
            position: position.clone(),
            instruction_index: instruction_index.clone()
        };

        if seen_states.contains(&state) {
            break;
        }

        seen_states.insert(state);

        let instruction =
            problem.instructions.get(instruction_index).unwrap();

        let option =
            problem.options.get(&position).unwrap();

        if end_positions.contains(&position) {
            possible_end_states.push((position, step))
        }

        position =
            if instruction == &'L' {
                option.0.clone()
            } else {
                option.1.clone()
            };

        step += 1
    }

    possible_end_states
}


#[derive(Clone, Debug)]
struct Problem {
    instructions: Vec<char>,
    options: HashMap<String, (String, String)>
}

fn read_input(filename: &String) ->  io::Result<Problem> {
    let file_in = File::open(filename)?;

    let mut lines = BufReader::new(file_in).lines().map(|x|x.unwrap());

    let instructions: Vec<char> = lines.next().unwrap().chars().collect();

    lines.next();


    let mut options: HashMap<String, (String, String)> = HashMap::new();

    for line in lines {
        let it = line.replace("=", "")
                            .replace(",", "")
                            .replace("(", "")
                            .replace(")", "");
        let mut it = it.split_whitespace();
        options.insert(String::from(it.next().unwrap()),
                     (String::from(it.next().unwrap()),
                         String::from(it.next().unwrap())));
    }

    Ok(Problem {
        instructions,
        options
    })
}
