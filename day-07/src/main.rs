use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;
use std::collections::BTreeMap ;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let problem = read_input(input)?;

    println!("{:?} are the total winnings", solve1(&problem));

    Ok(())
}
fn solve1(problem: &Problem) -> usize {
    let mut players = problem.players.clone();
    players.sort();

    players.iter().rev().enumerate().map(|(i, player)| {
            (i+1) * player.bet
    }).sum()
}

#[derive(Clone)]
struct Problem {
    players: Vec<Player>
}

#[derive(Clone, PartialEq, Eq, Ord)]
struct Player{
    hand: Hand,
    bet: usize
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.hand.partial_cmp(&other.hand)
    }
}

#[derive(Clone, Eq, Ord)]
struct Hand {
    cards: Vec<Card>
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        // Define how two structs are considered equal
        self.cards == other.cards // Example: compare based on `field1`
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Define how two structs are ordered
        let ordering = self.get_type().partial_cmp(&other.get_type()); // Example: compare based on `field1`
        if ordering.is_some() && ordering.unwrap() == std::cmp::Ordering::Equal {
            for i in 0..(self.cards.len()) {
                let l = self.cards.get(i).unwrap();
                let r = other.cards.get(i).unwrap();
                let o = l.cmp(r);
                if o != std::cmp::Ordering::Equal {
                    return Some(o)
                }
            }
            ordering
        } else {
            ordering
        }
    }
}

impl Hand {
    fn get_type(&self) -> HandType {
        let mut card2amount: BTreeMap<Card, u32> = BTreeMap::new();

        for c in &self.cards {
            *card2amount.entry(c.clone()).or_insert(0) += 1;
        }

        if card2amount.len() == 1 {
            HandType::FiveOfAKind
        } else if card2amount.len() == 2
            && card2amount.iter()
            .map(|(_k, v)| { if v == &4 { 1 } else { 0 } } ).sum::<u32>() > 0 {
            HandType::FourOfAKind
        } else if card2amount.len() == 2 {
            HandType::FullHouse
        } else if card2amount.iter()
            .map(|(_k, v)| if v == &3 { 1 } else { 0 }).sum::<u32>() > 0 {
            HandType::ThreeOfAKind
        } else if card2amount.iter().map(|(_k, v)| if v == &2 { 1 } else { 0 }).sum::<u32>() > 1 {
            HandType::TwoPair
        } else if card2amount.iter().map(|(_k, v)| if v == &2 { 1 } else { 0 }).sum::<u32>() > 0 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

fn read_input(filename: &String) ->  io::Result<Problem> {
    let file_in = File::open(filename)?;
    let players: Vec<Player> =
        BufReader::new(file_in).lines()
            .map(|x|x.unwrap())
            .map(|x| {
                let mut it = x.split_whitespace();
                let cards: Vec<Card> =
                    it.next().unwrap().chars().map(|c|
                        match c {
                            'A' => Card::Ace,
                            'K' => Card::King,
                            'Q' => Card::Queen,
                            'J' => Card::Jack,
                            'T' => Card::Ten,
                            '9' => Card::Nine,
                            '8' => Card::Eight,
                            '7' => Card::Seven,
                            '6' => Card::Six,
                            '5' => Card::Five,
                            '4' => Card::Four,
                            '3' => Card::Three,
                            '2' => Card::Two,
                            _ => panic!("unexpected input")
                        }
                    ).collect();
                let bet = it.next().unwrap().parse::<usize>().unwrap();
                Player {
                    hand: Hand {
                        cards
                    },
                    bet
                }
            })
            .collect();
    Ok(Problem {
        players
    })
}