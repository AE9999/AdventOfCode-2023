use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let     problem = read_input(input)?;

    println!("What is the total load on the north support beams?, {:?}",
             solve1(&mut problem.clone()));

    println!("Afterwards, what is the total load on the north support beams?, {:?}",
             solve2(&mut problem.clone()));

    Ok(())
}

fn solve1(problem: &mut Problem) -> usize {
    let north = Point { x: 0, y: -1 };
    problem.step(&north);
    problem.total_load()
}

fn solve2(problem: &mut  Problem) -> usize {
    let total_steps = 1000000000;

    let mut step_till_cycle: HashMap<Problem, usize> = HashMap::new();

    for step in 1..total_steps + 1 {

        problem.cycle();

        if step_till_cycle.contains_key(&problem) {
            let previous_steps = step_till_cycle.get(&problem).unwrap();
            let steps_till_equalibrium = step - previous_steps;

            let remaining_steps = 1000000000 - step;
            let remainder_of_remaining_step = remaining_steps % steps_till_equalibrium;

            (0..remainder_of_remaining_step).for_each(|_|problem.cycle());

            return problem.total_load()
        }

        step_till_cycle.insert(problem.clone(), step);

    }

    problem.total_load()
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Hash, Debug)]
struct Problem {
    maps: Vec<Vec<char>>
}

impl Problem {
    fn height(&self) -> usize {
        self.maps.len()
    }

    fn width(&self) -> usize {
        self.maps.get(0).unwrap().len()
    }

    fn char_at(&self, point: &Point) -> char {
        let x: &Vec<char> = self.maps.get(point.y as usize).unwrap();
        x.get(point.x as usize).unwrap( ).clone()
    }

    fn is_on_map(&self, point: &Point) -> bool {
        point.x >= 0
        && (point.x as usize) < self.width()
        && point.y >= 0
        && (point.y as usize) < self.height()
    }

    fn is_free(&self,
               point: &Point) -> bool {
        self.is_on_map(point) && self.char_at(point) == '.'
    }

    fn set_chat_at_position(&mut self,
                            point: &Point,
                            value: char) {
        let row = self.maps.get_mut(point.y as usize).unwrap();
        row[point.x as usize] = value
    }

    fn total_load(&self) -> usize {
        self.maps.iter()
                 .enumerate()
                 .map(|(index, line)|{
                     line.iter().map(|c| if c == &'O' { self.height() - index  } else { 0 })
                                .sum::<usize>()
                  }).sum::<usize>()
    }

    fn handle_step(&mut self,
                   position: &Point,
                   direction: &Point) {

        if self.char_at(&position) != 'O' {
            return;
        }

        let mut next_postion = position.clone();

        loop {
            let test = next_postion.add(direction);
            if !self.is_free(&test) {
                break;
            }
            next_postion = test
        }

        self.set_chat_at_position(&position, '.');

        self.set_chat_at_position(&next_postion, 'O');
    }

    fn cycle(&mut self) {
        let north = Point { x: 0, y: -1 };
        let west = Point { x: -1, y: 0 };
        let south = Point { x: 0, y: 1 };
        let east = Point { x: 1, y: 0 };

        for direction in [north, west, south, east] {
            self.step(&direction);
        }
    }

    fn step(&mut self, direction: &Point) {

        let north = Point { x: 0, y: -1 };
        let west = Point { x: -1, y: 0 };
        let south = Point { x: 0, y: 1 };
        let east = Point { x: 1, y: 0 };

        if direction == &north  {
            for y in 0..self.height() {
                for x in 0..self.width() {
                    let position = Point {
                        x: x as i32,
                        y: y as i32,
                    };
                    self.handle_step(&position, direction);
                }
            }
        } else if direction == &west {
            for y in 0..self.height() {
                for x in 0..self.width() {
                    let position = Point {
                        x: x as i32,
                        y: y as i32,
                    };
                    self.handle_step(&position, direction);
                }
            }
        } else if direction == &south {
            for y in (0..self.height()).rev() {
                for x in 0..self.width() {
                    let position = Point {
                        x: x as i32,
                        y: y as i32,
                    };
                    self.handle_step(&position, direction);
                }
            }
        } else if direction == &east {
            for y in 0..self.height() {
                for x in (0..self.width()).rev() {
                    let position = Point {
                        x: x as i32,
                        y: y as i32,
                    };
                    self.handle_step(&position, direction);
                }
            }
        } else {
            panic!("Not supported")
        }
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn add(&self, other: &Point) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn read_input(filename: &String) ->  io::Result<Problem> {
    let file_in = File::open(filename)?;
    let maps: Vec<Vec<char>>  =
        BufReader::new(file_in).lines()
                                     .map(|line|line.unwrap().chars().collect())
                                     .collect();

    Ok(Problem {
        maps
    })
}
