use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let problem = read_input(input)?;

    println!("What number do you get after summarizing all of your notes? {:?}",
            solve1(&problem));

    Ok(())
}

fn solve1(problem: &Problem) -> usize {
    problem.patterns.iter().map(|pattern|pattern.summarize()).sum()
}

struct Problem {
    patterns: Vec<Pattern>
}

struct Pattern {
    map: Vec<Vec<char>>
}

impl Pattern {

    fn is_mirror_column(&self, index: usize) -> bool {
        for y in self.height() {

        }
        true
    }

    fn is_mirror_row(&self, index: usize) -> bool {
        true
    }

    fn summarize(&self) -> usize {
        let l : usize =
            (0..(self.width() - 1)).filter(|i| self.is_mirror_column(*i))
                                   .map(|i| i+1 )
                                   .sum();

        let r : usize =
            (0..(self.height() - 1)).filter(|i| self.is_mirror_row(*i))
                .map(|i| (i+1) * 100 )
                .sum();

        l + r
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
