use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let mut input = read_input(input)?;

    Ok(())
}


fn read_input(filename: &String) ->  io::Result<()> {
    let file_in = File::open(filename)?;
    let file_reader = BufReader::new(file_in).lines();

    for line in file_reader {
        let line = line.unwrap();
    }

    Ok(elves)
}
