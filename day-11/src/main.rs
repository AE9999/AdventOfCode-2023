extern crate core;

use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;
use itertools::Itertools;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let problem = read_input(input)?;

    println!("What is the sum of these lengths? {:?}",
             problem.solve1());

    println!("What is the sum of these lengths? {:?}",
             problem.solve2());

    Ok(())
}

struct Problem {
    nodes: HashSet<Point>,
    empty_rows: HashSet<usize>,
    empty_columns: HashSet<usize>,
}

impl Problem {

    fn solve1(&self) -> i64 {

        self.nodes.iter()
                  .tuple_combinations()
                  .map(|(l,r)|  self.distance(l.clone(), r.clone(), 2))
                  .sum()
    }

    fn solve2(&self) -> i64 {

        self.nodes.iter()
            .tuple_combinations()
            .map(|(l,r)|  self.distance(l.clone(), r.clone(), 1000000))
            .sum()
    }

    fn distance(&self, start: Point, end: Point, space: i64) -> i64 {
        
        let minx = min(start.x, end.x);
        let maxx = max(start.x, end.x);

        let miny = min(start.y, end.y);
        let maxy = max(start.y, end.y);


        let extra_y: i64 = self.empty_rows.iter().map(|y| *y as i64).map(|y| if y >= miny && y <= maxy { space - 1 } else { 0 }).sum();
        let extra_x: i64 = self.empty_columns.iter().map(|x| *x as i64).map(|x| if x >= minx && x <= maxx { space - 1 } else { 0 }).sum();

        (maxx - minx + extra_x) +  (maxy - miny + extra_y)
    }

    fn new(map: Vec<Vec<char>>) -> Self {

        let mut nodes: HashSet<Point> = HashSet::new();

        let empty_rows: HashSet<usize> = HashSet::from_iter(
            map.iter().enumerate()
                          .filter(|(_index, row)| {
                              row.iter().find(|c| c == &&'#' ).is_none()
                          })
                         .map(|(index, _row)| index)
        );

        let empty_columns : HashSet<usize> =
            HashSet::from_iter(
                (0..(map.get(0).unwrap().len())).filter(|x| {
                    (0..(map.len())).find(|y|
                        map.get(*y).unwrap().get(*x).unwrap() == &'#'
                    ).is_none()
                })
            );

        for y in 0..map.len() {

            let row = map.get(y).unwrap();

            for x in 0..row.len() {

                let point =  Point {
                    x:x as i64,
                    y:y as i64
                };

                if row.get(x).unwrap() == &'#' {
                    nodes.insert(point.clone());
                }
            }
        }

        Problem {
            nodes,
            empty_rows,
            empty_columns,
        }
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Hash, Debug)]
struct Point {
    x: i64,
    y: i64,
}

fn read_input(filename: &String) ->  io::Result<Problem> {
    let file_in = File::open(filename)?;
    let map: Vec<Vec<char>> =
        BufReader::new(file_in).lines()
                                     .map(|line|{line.unwrap().chars().collect()})
                                     .collect();



    Ok(Problem::new(map))
}
