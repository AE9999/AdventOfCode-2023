use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let problem = read_input(input)?;

    println!("how many garden plots could the Elf reach in exactly 64 steps? {:?}",
             solve1(&problem));

    Ok(())
}

fn solve1(problem: &Problem) -> usize {
    let mut visited_plots = HashSet::<(Point, usize)>::new();
    let mut queue = VecDeque::<(Point, usize)>::new();

    let start = (problem.find_s(), 0);
    queue.push_back(start.clone());
    visited_plots.insert(start.clone());

    while let Some((point, steps)) = queue.pop_front() {

        if steps < 64 {
            point.neighbours().iter()
                              .filter(|neighbour| problem.char_at(neighbour) == '.'
                                                           || problem.char_at(neighbour) == 'S')
                              .for_each(|neighbour| {
                                    let next = (neighbour.clone(), steps + 1);
                                    if !visited_plots.contains(&next) {
                                        visited_plots.insert(next.clone());
                                        queue.push_back(next.clone());
                                    }
                              });
        }
    }
    visited_plots.iter().filter(|(point, step)| step == &64).collect::<HashSet<_>>().len()
}

struct Problem {
    map: Vec<Vec<char>>
}

impl Problem {
    fn find_s(&self) -> Point {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let point = Point {x: x as i64, y: y as i64};
                if self.char_at(&point) == 'S' {
                    return point;
                }
            }
        }
        panic!("unexpected state")
    }

    fn char_at(&self, point: &Point) -> char {
        *self.map.get(point.y as usize).unwrap().get(point.x as usize).unwrap()
    }

    fn height(&self) -> usize {
        self.map.len()
    }

    fn width(&self) -> usize {
        self.map.get(0).unwrap().len()
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
}


fn read_input(filename: &String) ->  io::Result<Problem> {
    let file_in = File::open(filename)?;
    let file_reader = BufReader::new(file_in).lines();
    let map: Vec<Vec<char>> = file_reader.map(|x|x.unwrap().chars()
                                                                        .collect::<Vec<char>>())
                                         .collect();
    Ok(Problem {
        map
    })
}