use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::collections::HashSet;
use std::collections::HashMap;
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let puzzle = read_input(input)?;

    println!("The cards are worth {:?} many points total", solve1(&puzzle));

    println!("{:?} many total scratchcards you end up with", solve2(&puzzle));

    Ok(())
}

fn solve1(puzzle: &Puzzle) -> usize {
    puzzle.cards.iter()
                .map(|card| {
                    let intersections = card.intersections();
                    if intersections > 0 {
                        let base: usize = 2;
                        base.pow(intersections - 1)
                    } else {
                        0
                    }
                }).sum()
}

fn solve2(puzzle: &Puzzle) -> usize {

    let mut todos: HashMap<usize, usize> = HashMap::new();

    let mut total_cards = puzzle.cards.len();

    (0..(puzzle.cards.len())).for_each(|x|{ todos.insert(x, 1); } );

    for x in 0..(puzzle.cards.len()) {

        let amount_of_this_card = todos.get(&x).unwrap().clone();

        let intersections = puzzle.cards.get(x).unwrap().intersections() as usize;

        for y in 1..(intersections.clone() + 1) {
            let card_id = x.clone() + (y as usize);
            let cards_left = todos.get(&card_id).unwrap() + amount_of_this_card.clone();
            todos.insert(card_id, cards_left);
        };

        total_cards += intersections * amount_of_this_card;

    };

    total_cards
}

fn read_input(filename: &String) ->  io::Result<Puzzle> {
    let file_in = File::open(filename)?;
    let cards: Vec<Card> =
        BufReader::new(file_in).lines()
                                     .map(|x|x.unwrap())
            .map(|x| {
                let mut it = x.split("|");

                let winning_numbers_and_card_string = it.next().unwrap().trim();
                let numbers_string = it.next().unwrap().trim();


                let mut it = winning_numbers_and_card_string.split(":");

                let id_string = it.next().unwrap();

                let winning_numbers_and_card_string = it.next().unwrap().trim();

                let mut it = id_string.split_whitespace();
                it.next();

                let _id = it.next().unwrap().trim().parse::<u32>().unwrap();

                let winning_numbers : HashSet<u32> =
                    winning_numbers_and_card_string
                                          .trim()
                                          .split_whitespace()
                                          .map(|x| {
                                              x.parse::<u32>().unwrap() })
                                          .collect();


                let numbers : HashSet<u32> =
                    numbers_string.trim()
                                  .split_whitespace()
                                  .map(|x| x.parse::<u32>().unwrap())
                                  .collect();
                Card {
                    winning_numbers,
                    numbers
                }
            }).collect();

    Ok(Puzzle {
        cards
    })
}

struct Puzzle {
    cards: Vec<Card>
}

struct Card {
    winning_numbers: HashSet<u32>,
    numbers: HashSet<u32>
}

impl Card {
    fn intersections(&self) -> u32 {
        let intersection: Vec<&u32>
            = self.winning_numbers.intersection(&self.numbers).collect();
        intersection.len() as u32
    }
}
