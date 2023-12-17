use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let problem = read_input(input)?;

    println!("What is the least heat loss it can incur? {:?}",
             solve1(&problem));

    Ok(())
}

fn solve1(problem: &Problem) -> u32 {

    let mut queue: VecDeque<State> = VecDeque::new();
    queue.push_back(State { steps: Vec::new() });

    let mut seen: HashSet<State> = HashSet::new();

    let mut local_lower_bounds: HashMap<(Point, Vec<Point>), u32> = HashMap::new();

    let mut global_lb: u32 = u32::MAX;

    while let Some(state) = queue.pop_front() {

        if seen.contains(&state) {
            continue
        }

        println!("state: {:?}", state);

        seen.insert(state.clone());

        let (cost, position, options, last_two_positions) =
            state.cost_and_position_options_and_last_two_positions(problem);
        println!("cost: {:?}, position: {:?}, options: {:?}", cost, position, options);

        let key = (position.clone(), last_two_positions.clone());
        if local_lower_bounds.contains_key(&key) {
            let current_lb = local_lower_bounds.get(&key).unwrap();
            if &cost > current_lb {
                continue;
            }
        }
        local_lower_bounds.insert(key, cost);

        if cost > global_lb {
            continue;
        }

        if position == problem.end() {
            global_lb = cost
        }

        options.iter().map(|dxdy| {
            let mut steps = state.steps.clone();
            steps.push(dxdy.clone());
            State { steps }
        }).filter(|next_state| {
            !seen.contains(next_state)
        }).for_each(|next_state|{
            queue.push_back(next_state)
        })
    }

    global_lb
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Hash, Debug)]
struct Problem {
   map: Vec<Vec<u32>>
}

impl Problem {

    fn point_on_map(&self, point: &Point) -> bool {
        point.x >= 0
        && point.y >= 0
        && (point.x as usize) < self.width()
        && (point.y as usize) < self.height()
    }

    fn height(&self) -> usize {
        self.map.len()
    }

    fn width(&self) -> usize {
        self.map.get(0).unwrap().len()
    }

    fn digit_at_point(&self, point: &Point) -> u32 {
        *(self.map.get(point.y as usize).unwrap().get(point.x as usize).unwrap())
    }

    fn end(&self) -> Point {
        Point {
            x: (self.width() - 1) as i32,
            y: (self.height() - 1) as i32
        }
    }
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
}


#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Hash, Debug)]
struct State {
    steps: Vec<Point>
}

impl State {
    fn cost_and_position_options_and_last_two_positions(&self,
                                                        problem: &Problem) -> (u32, Point, Vec<Point>, Vec<Point>) {

        let mut point = Point {x: 0, y: 0};

        let mut cost: u32 = 0;

        for step in self.steps.iter() {
            point = point.add(step);
            cost = problem.digit_at_point(&point);
        }

        let last_three_positions: Vec<Point> =
            self.steps.iter().rev().take(3).map(|x|x.clone()).collect();


        let up = Point { x: 0, y: -1 };
        let down = Point { x: 0, y: 1 };
        let left = Point { x: -1, y: 0 };
        let right = Point { x: 1, y: 0 };

        let dxdys: Vec<Point> =
            if last_three_positions.len() == 3
               && last_three_positions[0] == last_three_positions[1]
               && last_three_positions[0] == last_three_positions[2]{
                // must turn left or right
                match last_three_positions[0] {
                    _ if last_three_positions[0] == up => vec![left, right],
                    _ if last_three_positions[0] == down => vec![left, right],
                    _ if last_three_positions[0] == left => vec![up, down],
                    _ if last_three_positions[0] == right => vec![up, down],
                    _ => panic!("Unrecognized point!")
                }
            } else if last_three_positions.len() > 0 {
                match last_three_positions[0] {
                    _ if last_three_positions[0] == up => vec![left, right, up],
                    _ if last_three_positions[0] == down => vec![left, right, down],
                    _ if last_three_positions[0] == left => vec![up, down, left],
                    _ if last_three_positions[0] == right => vec![up, down, right],
                    _ => panic!("Unrecognized point!")
                }
            } else {
                vec![down, right] // At 0,0
            };

        let options: Vec<Point> =
            dxdys.into_iter()
                 .filter(|dxdy| problem.point_on_map(&point.add(dxdy)))// .map(|dxdy| point.add(dxdy))
                 .collect();

        let last_two_positions = last_three_positions.into_iter().take(2).collect();

        (cost, point, options, last_two_positions)
    }
}

fn read_input(filename: &String) ->  io::Result<Problem> {
    let file_in = File::open(filename)?;
    let map: Vec<Vec<u32>> = BufReader::new(file_in).lines()
                                                          .map(|x|x.unwrap().chars().map(|x|x.to_digit(10).unwrap()).collect())
                                                          .collect();
    Ok(Problem {
        map
    })
}