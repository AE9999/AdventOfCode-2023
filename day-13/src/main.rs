extern crate core;

use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let mut problem = read_input(input)?;

    println!("What number do you get after summarizing all of your notes? {:?}",
            solve1(&problem));

    println!("What number do you get after summarizing the new reflection line in each pattern in your notes? {:?}",
            solve2(&mut problem));

    Ok(())
}

fn solve2(problem: &mut Problem) -> usize {
    problem.patterns.iter()
                    .map(|pattern| {
                        let row = pattern.get_mirror_row();
                        let column =pattern.get_mirror_column();

                        for x in 0..pattern.width() {
                            for y in 0..pattern.height() {
                                let other_pattern = pattern.flip(x, y);
                                let other_row = other_pattern.get_mirror_row_excluding(&row);
                                let other_column =other_pattern.get_mirror_column_excluding(&column);

                                if other_row.is_some()
                                   || other_column.is_some() {
                                    let row_value =  if row != other_row { other_row.unwrap_or(0) } else { 0};
                                    let column_value = if column != other_column { other_column.unwrap_or(0) } else {0};
                                    return row_value + (column_value * 100)
                                }
                            }
                        }
                        panic!("Unexpected state");
                    }).sum()
}

fn solve1(problem: &Problem) -> usize {
    problem.patterns.iter()
                     .map(|pattern| {
                         let row = pattern.get_mirror_row().unwrap_or(0);
                         let column =pattern.get_mirror_column().unwrap_or(0) * 100;
                         row + column
                     }).sum()
}

struct Problem {
    patterns: Vec<Pattern>
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Hash, Debug)]
struct Pattern {
    map: Vec<Vec<char>>
}

impl Pattern {

    fn is_mirror_column(&self, line: f32) -> bool {


        for y in 0..self.height() {
            let mut left_index = line.floor() as i32;
            let mut right_index = line.ceil()  as i32;
            let row = self.map.get(y).unwrap();

            while left_index >= 0
                  && right_index < (self.width() as i32) {

                let l = row.get(left_index as usize).unwrap();
                let r = row.get(right_index as usize).unwrap();
                if l != r {
                    return false
                }

                left_index -= 1;
                right_index += 1;
            }
        }
        true
    }

    fn is_mirror_row(&self, line: f32) -> bool {

        let mut left_index = line.floor() as i32;
        let mut right_index = line.ceil()  as i32;

        while left_index >= 0
            && right_index < (self.height() as i32) {

            let lrow = self.map.get(left_index as usize).unwrap();
            let rrow = self.map.get(right_index as usize).unwrap();

            for x in 0..self.width() {
                let l = lrow.get(x).unwrap();
                let r = rrow.get(x).unwrap();
                if l != r {
                    return false
                }
            }

            left_index -= 1;
            right_index += 1;
        }

        true
    }

    fn flip(&self, x: usize, y: usize) -> Self {
        let mut new_map: Vec<Vec<char>> = self.map.clone();
        let  row = new_map.get_mut(y).unwrap();
        row[x] = if self.map.get(y).unwrap().get(x).unwrap() == &'#' { '.' } else { '#' };
        Pattern {
            map: new_map
        }
    }

    fn get_mirror_row(&self) -> Option<usize> {
            (0..(self.width() - 1))
                .find(|i| self.is_mirror_column((*i as f32)  + 0.5))
                .map(|i| i + 1)
    }

    fn get_mirror_row_excluding(&self, exclusion: &Option<usize>) -> Option<usize> {

        if exclusion.is_none() {
            return self.get_mirror_row()
        }

        let exclusion = exclusion.unwrap();

        (0..(self.width() - 1))
            .find(|i| i + 1 != exclusion && self.is_mirror_column((*i as f32)  + 0.5))
            .map(|i| i + 1)
    }

    fn get_mirror_column(&self) -> Option<usize> {
        (0..(self.height() - 1)).find(|i| self.is_mirror_row((*i as f32) + 0.5))
                                .map(|i| i+1 )
    }

    fn get_mirror_column_excluding(&self, exclusion: &Option<usize>) -> Option<usize> {

        if exclusion.is_none() {
            return self.get_mirror_column()
        }

        let exclusion = exclusion.unwrap();

        (0..(self.height() - 1))
            .find(|i| i + 1 != exclusion && self.is_mirror_row((*i as f32) + 0.5))
            .map(|i| i+1 )
    }

    fn width(&self) -> usize {
        self.map.get(0).unwrap().len()
    }

    fn height(&self) -> usize {
        self.map.len()
    }
}


fn read_input(filename: &String) ->  io::Result<Problem> {
    let file_in = File::open(filename)?;

    let mut map : Vec<Vec<char>> = Vec::new();

    let mut patterns: Vec<Pattern> = Vec::new();

    for line in BufReader::new(file_in).lines().map(|x| { x.unwrap() }) {

        if line.is_empty() {
            patterns.push(Pattern {
                map: map.clone()
            });
            map.clear();
        } else {
            map.push(line.chars().collect())
        }
    }

    patterns.push(Pattern {
        map: map.clone()
    });

    Ok(Problem {
        patterns
    })

}
