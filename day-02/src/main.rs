use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;
use std::collections::HashMap;


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let input = read_input(input)?;

    let configuration = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let answer: usize =
        input.iter()
             .filter(|game| {
                 !is_possible(game, &configuration)
             })
             .map(|game| game.id).sum();

    println!("{:?}  is the sum of the IDs of those games", answer);

    let answer: i32  =
        input.iter()
            .map(|game| power_of_game(&game)).sum();

    println!("{:?}  is the sum of the power of these sets", answer);

    Ok(())
}

fn is_possible(game: &Game,
               configuration: &HashMap<&str, i32>) -> bool {
    game.draws.iter()
        .find(|draw| draw.keys()
            .find(|color|
                !configuration.contains_key(color.as_str())
                    || configuration.get(color.as_str()).unwrap()
                    < draw.get(color.as_str()).unwrap())
            .is_some())
        .is_some()
}

fn power_of_game(game: &Game) -> i32 {
    min_amount_of_color_needed(game, "red")
    * min_amount_of_color_needed(game, "blue")
    * min_amount_of_color_needed(game, "green")
}

fn min_amount_of_color_needed(game: &Game, color: &str) -> i32 {
    game.draws
        .iter()
        .map(|draw| draw.get(color).or(Some(&0)).unwrap())
        .max().unwrap().clone()
}

fn read_input(filename: &String) ->  io::Result<Vec<Game>> {
    let file_in = File::open(filename)?;
    Ok(BufReader::new(file_in).lines().map(|x| x.unwrap()).map(|x| {
        let mut main = x.split(":");
        let game_info_str = main.next().unwrap().to_string();
        let tmp = main.next().unwrap().to_string();
        let draws_str = tmp.split(";");

        let mut id = game_info_str.split(" ");
        id.next();
        let id  = id.next().unwrap().parse::<usize>().unwrap();

        let draws = draws_str.into_iter().map(|x| {
            let mut map: HashMap<String, i32> = HashMap::new();
            for (key, value) in  x.trim().split(",")
                                                    .map(|y|{
                                                        let mut z = y.trim().split(" ");
                                                        let value = z.next().unwrap().parse::<i32>().unwrap();
                                                        let key = String::from(z.next().unwrap());
                                                        (key, value)
                                                    }) {
                    map.insert(key, value);
            }
            map
        }).collect();
        Game {
            id,
            draws
        }
    }).collect())
}

struct Game {
    id: usize,
    draws: Vec<HashMap<String, i32>>,
}