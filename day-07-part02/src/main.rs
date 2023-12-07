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
    cards: Vec<Card>,
    best_hand: HandType,
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
        let ordering = self.best_hand.partial_cmp(&other.best_hand); // Example: compare based on `field1`
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

fn get_best_type(cards: Vec<Card>) -> HandType {

    let joker_index =
        cards.iter()
             .enumerate()
             .find(|(_i, card)| card == &&Card::Joker)
            .map(|(i, _card)| { i });

    if joker_index.is_none() {
        return get_type(&cards)
    }

    let joker_index = joker_index.unwrap();

    let other_cards = [
        Card::Ace,
        Card::King,
        Card::Queen,
        Card::Ten,
        Card::Nine,
        Card::Eight,
        Card::Seven,
        Card::Six,
        Card::Five,
        Card::Four,
        Card::Three,
        Card::Two
    ];

    other_cards.iter().map(|other_card| {
        let mut next_cards = cards.clone();
        next_cards[joker_index] = other_card.clone();
        get_best_type(next_cards)
    }).min()
      .unwrap()
}


fn get_type(cards: &Vec<Card>) -> HandType {
    let mut card2amount: BTreeMap<Card, u32> = BTreeMap::new();

    for c in cards {
        *card2amount.entry(c.clone()).or_insert(0) += 1;
    }

    if card2amount.len() == 1 {
        HandType::FiveOfAKind
    } else if card2amount.len() == 2
        && card2amount.iter()
        .map(|(_k, v)| { if v == &4 { 1 } else { 0 } }).sum::<u32>() > 0 {
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
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
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
                            'J' => Card::Joker,
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
                let best_hand = get_best_type(cards.clone());
                Player {
                    hand: Hand {
                        cards,
                        best_hand
                    },
                    bet
                }
            })
            .collect();
    Ok(Problem {
        players
    })
}