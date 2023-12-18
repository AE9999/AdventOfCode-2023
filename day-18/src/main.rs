use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let problem = read_input(input)?;

    println!("how many cubic meters of lava could it hold? {:?}",
             solve1(&problem));

    Ok(())
}

fn solve1(problem: &Problem) -> usize {

    let up = Point { x: 0, y: -1 };
    let down = Point { x: 0, y: 1 };
    let left = Point { x: -1, y: 0 };
    let right = Point { x: 1, y: 0 };

    let mut postion: Point = Point {x:0, y:0};

    let mut dug_out_points: HashSet<Point> = HashSet::new();
    dug_out_points.insert(postion.clone());

    for instruction in &problem.instructions {
        let direction =
            match instruction.direction {
                'R' => &right,
                'L' => &left,
                'U' => &up,
                'D' => &down,
                _ => panic!("Unrecognized point!")
            };
        dug_out_points.extend(postion.all_points_from_me_in_direction(direction,
                                                                   instruction.amount.clone()));
        postion = postion.add(&direction.mul(instruction.amount as i32));
    }

    let maxy = dug_out_points.iter().map(|point| point.y).max().unwrap();
    let maxx = dug_out_points.iter().map(|point| point.x).max().unwrap();
    let minx = dug_out_points.iter().map(|point| point.x).min().unwrap();

    // let start: Point = (.map(|x| Point {x, y: maxy - 1})
    //                                          .find(|point| !dug_out_points.contains(point))
    //                                          .unwrap();
    #[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
    enum State {
        Entering,
        Outside,
        Inside,
    }

    let mut state: State = State::Outside;
    let mut start = Point { x: 0, y: 0 } ;
    for x in (minx - 1)..(maxx+1) {
        let point = Point { x: x, y: maxy - 1 };
        state =
            if dug_out_points.contains(&point) {
                match state {
                    State::Outside => State::Entering,
                    State::Entering => State::Entering,
                    State::Inside => panic!("Illegal state")
                }
            } else {
                match state {
                    State::Outside => State::Outside,
                    State::Entering => State::Inside,
                    State::Inside => panic!("Illegal state")
                }
            };
        if state == State::Inside {
            start = point.clone();
            break;
        }
    }

    assert!(state == State::Inside);

    let mut queue: VecDeque<Point> = VecDeque::new();
    queue.push_back(start.clone());
    dug_out_points.insert(start.clone());
    while let Some(point) = queue.pop_front() {
        point.neighbours().iter()
                          .for_each(|neighbour| {
                              if !dug_out_points.contains(neighbour) {
                                  dug_out_points.insert(neighbour.clone());
                                  queue.push_back(neighbour.clone());
                              }
                          })
    }

    dug_out_points.len()
}

#[derive(Clone, Hash, Debug)]
struct Problem {
    instructions: Vec<Instruction>
}

#[derive(Clone, Hash, Debug)]
struct Instruction {
    direction: char,
    amount: u32,
    code: String,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {

    fn add(&self, point: &Point) -> Self {
        Point {
            x: self.x + point.x,
            y: self.y + point.y,
        }
    }

    fn mul(&self, amount: i32) -> Self {
        Point {
            x: self.x * amount,
            y: self.y * amount
        }
    }

    fn neighbours(&self) -> Vec<Self> {
        let up = Point { x: 0, y: -1 };
        let down = Point { x: 0, y: 1 };
        let left = Point { x: -1, y: 0 };
        let right = Point { x: 1, y: 0 };

        vec![self.add(&up),
             self.add(&down),
             self.add(&left),
             self.add(&right)]
    }

    fn all_points_from_me_in_direction(&self, direction: &Point, amount: u32) -> Vec<Self> {
        (1..(amount + 1)).map(|i| self.add(&direction.mul(i as i32))).collect()
    }
}


fn read_input(filename: &String) ->  io::Result<Problem> {
    let file_in = File::open(filename)?;

    let instructions: Vec<Instruction> =
        BufReader::new(file_in).lines().map(|line| {
            let line = line.unwrap();
            let mut it = line.split(" ");
            let direction : char = it.next().unwrap().chars().next().unwrap();
            let amount: u32 = it.next().unwrap().parse::<u32>().unwrap();
            let code = it.next().unwrap().replace("(", "").replace(")", "");
            Instruction {
                direction,
                amount,
                code
            }
        }).collect();

    Ok(Problem {
        instructions
    })
}