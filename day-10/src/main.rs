use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let problem = read_input(input)?;

    let solutions  = solve1(&problem);

    println!("How many steps along the loop does it take to get from the starting position to the point farthest from the starting position? {:?}",
            solutions.0);

    println!("How many tiles are enclosed by the loop? {:?}",
             solutions.1);

    Ok(())
}

fn solve1(problem: &Problem) -> (usize, usize) {

    let mut visited_positions: HashMap<Point, usize> = HashMap::new();

    let mut deque: VecDeque<Point> = VecDeque::new();

    let mut dxdy :  HashMap<Point, String> = HashMap::new();

    dxdy.insert(Point { x:0, y:1 }, String::from("|LJ")); // North
    dxdy.insert(Point { x:0, y:-1 }, String::from("|7F")); // South
    dxdy.insert(Point { x: 1, y: 0 }, String::from("-J7")); // East
    dxdy.insert(Point { x: -1, y: 0 }, String::from("-LF")); // West

    let y =
        problem.maze.iter()
                    .enumerate()
                    .find(|(_y, line)|
                              {
                                  line.iter().enumerate().find(|(_x, c)| { c == &&'S' }).is_some()
                              }).map(|(y, _line)| y).unwrap();

    let x = problem.maze.iter().nth(y).map(|line| {
        let x = line.iter().enumerate().find(|(_x, c)| { c == &&'S' }).map(|(x,_c)| x);
        x.unwrap()
    }).unwrap();

    let start = Point {
        x: x as i64,
        y: y as i64,
    };

    visited_positions.insert(start.clone(), 0);

    deque.push_back(start.clone());

    while !deque.is_empty() {

        let point = deque.pop_front().unwrap();

        dxdy.iter()
            .for_each(|(dxdy, allowed_chars)| {

                let next_point = point.add(dxdy);

                if next_point.y >= 0 && next_point.y < problem.height()
                    && next_point.x >= 0 && next_point.x < problem.width()
                    && allowed_chars.contains(problem.char_at(&next_point))
                    && (!visited_positions.contains_key(&next_point)
                    || visited_positions.get(&point).unwrap()
                    < visited_positions.get(&next_point).unwrap()) {

                    visited_positions.insert(next_point.clone(),
                                             visited_positions.get(&point).unwrap() + 1);
                    deque.push_back(next_point);
                }
            });
    }

    //problem.debug(&visited_positions);
    let sol1 = *visited_positions.iter().map(|(_k,v)| v).max().unwrap();

    let mut intersection_x : HashSet<Point> = HashSet::new();
    let mut inside = false;

    for y in problem.height() {
        for x in problem.width() {
            if () {
                inside = !inside
            } else if (inside) {
                intersection_x.insert(Point {
                    x,y
                })
            }
        }
    }

    inside = false;
    let mut intersection_y : HashSet<Point> = HashSet::new();
    for x in problem.height() {
        for y in problem.width() {
            else if (inside) {
                intersection_x.insert(Point {
                    x,y
                })
            }
        }
    }

    let sol2 = 0;

    (sol1, sol2)
}


struct Problem {
    maze: Vec<Vec<char>>
}

impl Problem {
    fn height(&self) -> i64 {
        self.maze.len() as i64
    }

    fn width(&self) -> i64 {
        self.maze.get(0).unwrap().len() as i64
    }

    fn char_at(&self, point: &Point) -> char {
        *self.maze.get(point.y as usize).unwrap().get(point.x as usize).unwrap()
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Hash, Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn add(&self, other: &Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}


fn read_input(filename: &String) ->  io::Result<Problem> {
    let file_in = File::open(filename)?;
    let maze: Vec<Vec<char>> = BufReader::new(file_in).lines().map(|x|x.unwrap().chars().collect()).collect();
    Ok(Problem{
        maze
    })

}
