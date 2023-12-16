use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let problem = read_input(input)?;

    println!("How many tiles end up being energized? {:?}",
             solve1(&problem));

    Ok(())
}

fn solve1(problem: &Problem) -> usize {

    let state: State = State {
        position: Point {
            x:0,
            y:0,
        },
        direction: Point {
            x:1,
            y:0,
        },
    };

    let mut to_process: VecDeque<State> = VecDeque::new();
    to_process.push_back(state);

    let mut seen_states: HashSet<State> = HashSet::new();

    while !to_process.is_empty() {
        let state = to_process.pop_back().unwrap();
        seen_states.insert(state.clone());
        problem.calculate_next_state(&state)
               .into_iter()
               .for_each(|next_state|
                   if !seen_states.contains(&next_state) {
                        to_process.push_back(next_state.clone());
                   }
               )
    }

    //problem.debug(&seen_states);
    let energized_tiles: HashSet<Point> =
        HashSet::from_iter(seen_states.iter().map(|x| x.position.clone()));

    energized_tiles.len()
}

struct Problem {
    map: Vec<Vec<char>>
}

impl Problem {

    fn width(&self) -> usize {
        self.map.get(0).unwrap().len()
    }

    fn height(&self) -> usize {
        self.map.len()
    }

    fn is_on_map(&self, point: &Point) -> bool {
        point.x >= 0
        && point.y >= 0
        && (point.x as usize) < self.width()
        && (point.y as usize) < self.height()
    }

    fn char_at(&self, point: &Point) -> char {
        *(self.map.get(point.y as usize).unwrap().get(point.x as usize).unwrap())
    }

    fn calculate_next_state(&self, state: &State) -> Vec<State> {
        let going_right = Point { x: 1, y:0 };
        let going_left = Point { x: -1, y:0 };
        let going_up = Point { x: 0, y: -1 };
        let going_down = Point { x: 0, y: 1 };

        let mut next_states : Vec<State> = Vec::new();
        match self.char_at(&state.position) {
            '.' => {
                next_states.push(state.clone())
            },
            '/' => {
                let next_state =
                    if state.direction == going_right {
                        state.change_direction(&going_up)
                    } else if state.direction == going_left {
                        state.change_direction(&going_down)
                    } else if state.direction == going_up {
                        state.change_direction(&going_right)
                    } else if state.direction == going_down {
                        state.change_direction(&going_left)
                    } else {
                        panic!("Unexpected stated")
                    };
                next_states.push(next_state)
            },
            '\\' => {
                let next_state =
                    if state.direction == going_right {
                        state.change_direction(&going_down)
                    } else if state.direction == going_left {
                        state.change_direction(&going_up)
                    } else if state.direction == going_up {
                        state.change_direction(&going_left)
                    } else if state.direction == going_down {
                        state.change_direction(&going_right)
                    } else {
                        panic!("Unexpected stated")
                    };
                next_states.push(next_state)
            },
            '|' => {
                if state.direction == going_right {
                    next_states.push(state.change_direction(&going_up));
                    next_states.push(state.change_direction(&going_down));
                } else if state.direction == going_left {
                    next_states.push(state.change_direction(&going_up));
                    next_states.push(state.change_direction(&going_down));
                } else if state.direction == going_up {
                    next_states.push(state.clone());
                } else if state.direction ==  going_down {
                    next_states.push(state.clone());
                } else {
                    panic!("Unexpected stated")
                }
            },
            '-' => {
                if state.direction == going_right {
                    next_states.push(state.clone());
                } else if state.direction == going_left {
                    next_states.push(state.clone());
                } else if state.direction == going_up {
                    next_states.push(state.change_direction(&going_left));
                    next_states.push(state.change_direction(&going_right));
                } else if state.direction ==  going_down {
                    next_states.push(state.change_direction(&going_left));
                    next_states.push(state.change_direction(&going_right));
                } else {
                    panic!("Unexpected stated")
                }
            },
            _ => panic!("Unexpected input")

        }
        next_states.iter()
                   .map(|x| x.continue_onwards())
                   .filter(|x|self.is_on_map(&x.position))
                   .collect()
    }

    // fn debug(&self, energized: &HashSet<Point>) {
    //     for y in 0..self.height() {
    //         let  row: String =
    //             self.map.get(y)
    //                     .unwrap()
    //                     .clone()
    //                     .into_iter()
    //                     .enumerate()
    //                     .map(|(x,c)|{
    //                         let  point = Point {x: x as i64,
    //                                             y: y as i64 };
    //                         if energized.contains( &point) { '#' }
    //                         else { c }
    //                     })
    //                     .collect();
    //         println!("{:?}", row);
    //
    //     }
    // }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Hash, Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn add(&self, other: &Point) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Hash, Debug)]
struct State {
    position: Point,
    direction: Point,
}

impl State {
    fn continue_onwards(&self) -> Self {
        State {
            position: self.position.add(&self.direction),
            direction: self.direction.clone()
        }
    }

    fn change_direction(&self, direction: &Point) -> Self {
        State {
            position: self.position.clone(),
            direction: direction.clone()
        }
    }
}


fn read_input(filename: &String) ->  io::Result<Problem> {
    let file_in = File::open(filename)?;
    let map: Vec<Vec<char>> =
        BufReader::new(file_in).lines()
                                     .map(|x|x.unwrap().chars().collect())
                                     .collect();
    Ok(Problem {
        map
    })
}