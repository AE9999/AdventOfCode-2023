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


    println!("How many cubic meters of lava could the lagoon hold? {:?}",
             solve2(&problem));

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
        postion = postion.add(&direction.mul(instruction.amount as i64));
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

fn solve2(problem: &Problem) -> usize {

    let mut start: Point = Point {x:0, y:0};

    let mut spans: Vec<Span> = Vec::new();

    for instruction in &problem.instructions {
        let (length, direction) = code2length_and_direction(instruction.code.as_str());

        let end = start.add(&direction.mul(length as i64));


        let up = Point { x: 0, y: -1 };
        let down = Point { x: 0, y: 1 };
        let left = Point { x: -1, y: 0 };
        let right = Point { x: 1, y: 0 };

        let span =
            match direction {
                _ if direction == up => { Span { start: end.clone() , end: start.clone() } },
                _ if direction == down => { Span { start: start.clone() , end: end.clone() } },
                _ if direction == left => { Span { start: end.clone() , end: start.clone() } },
                _ if direction == right => { Span { start: start.clone(), end: end.clone() } },
                _ => panic!("Unexpected state")
            };

        spans.push(span);

        start = end;
    }

    let involved_points =
        spans.iter()
             .map(|span| vec![span.start.clone(), span.end.clone()].clone())
             .flatten()
             .collect::<Vec<Point>>();

    let maxy = involved_points.iter().map(|point| point.y).max().unwrap();
    let miny = involved_points.iter().map(|point| point.y).min().unwrap();

    let mut amount_filled_additionally_filled: usize = 0;
    for y in maxy..(miny-1) {

        let intersections: Vec<Span> = spans.iter()
                                            .map(|span| span.intersection(y))
                                            .filter(|span| span.is_some())
                                            .map(|span| span.unwrap())
                                            .collect();

        let startx = intersections.iter().map(|span|span.start.x).min().unwrap();
        let endx = intersections.iter().map(|span|span.end.x).max().unwrap();

        let amount_filled_on_this_line: usize =
            ((endx - startx) as usize) - intersections.iter().map(|span| span.amount()).sum::<usize>();
        amount_filled_additionally_filled += amount_filled_on_this_line;


    }

    // TODO add other stuff already added
    amount_filled_additionally_filled
}

fn code2length_and_direction(code: &str) -> (usize, Point) {
    let up = Point { x: 0, y: -1 };
    let down = Point { x: 0, y: 1 };
    let left = Point { x: -1, y: 0 };
    let right = Point { x: 1, y: 0 };


    let hexstr = code.chars().skip(1).take(5).collect::<String>();
    let length = usize::from_str_radix(hexstr.as_str(),
                                       16).unwrap();
    let direction =
        match code.chars().last().unwrap() {
            '0' => right,
            '1' => down,
            '2' => left,
            '3' => up,
            _ => panic!("unexpected char")
        };

    (length, direction)
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
struct Span {
    start: Point,
    end: Point,
}

impl Span {

    fn intersection(&self, y: i64) -> Option<Self> {
        if y >= self.start.y && y <= self.end.y {

            let start = Point {
              x: self.start.x,
              y,
            };

            let end = Point {
                x: self.end.x,
                y,
            };

            Some(Span {
                start,
                end
            })
        } else {
            None
        }
    }

    fn amount(&self) -> usize {
        ((self.end.x - self.start.x).abs() + (self.end.y - self.start.y).abs()) as usize
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Hash, Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {

    fn add(&self, point: &Point) -> Self {
        Point {
            x: self.x + point.x,
            y: self.y + point.y,
        }
    }

    fn mul(&self, amount: i64) -> Self {
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
        (1..(amount + 1)).map(|i| self.add(&direction.mul(i as i64))).collect()
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