use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let input = read_input(input)?;

    println!("What is the sum of the results? {:?}", solve1(&input));

    println!("What is the focusing power of the resulting lens configuration? {:?}",
             solve2(&input));

    Ok(())
}

fn solve1(input: &String) -> u32 {
    input.split(',').map(|str| decode(str)).sum()
}

fn solve2(input: &String) -> usize {
    let mut hash_map: HashMap<u32, Vec<(String, usize)>> = HashMap::new();
    (0..256).for_each(|i|{ hash_map.insert(i, Vec::new()); });

    for instruction in input.split(',') {

        if instruction.contains('-') {
            let label = String::from(instruction.split('-').next().unwrap());
            let box_nr = decode(label.as_str());
            let my_box: &mut Vec<(String, usize)> = hash_map.get_mut(&box_nr).unwrap();
            let index_of_lens =
                my_box.iter()
                      .enumerate()
                      .find(|(_index, (l, _))| &label == l)
                      .map(|(index,_)| index);
            if index_of_lens.is_some() {
                my_box.remove(index_of_lens.unwrap());
            }
        } else {
            assert!(instruction.contains('='));
            let mut it = instruction.split('=');
            let label = String::from(it.next().unwrap());
            let focal = it.next().unwrap().parse::<usize>().unwrap();
            let box_nr = decode(label.as_str());
            let my_box: &mut Vec<(String, usize)> = hash_map.get_mut(&box_nr).unwrap();
            let index_of_lens =
                my_box.iter()
                      .enumerate()
                      .find(|(_index, (l, _))| &label == l)
                      .map(|(index,_)| index);
            if index_of_lens.is_some() {
                my_box[index_of_lens.unwrap()] = (label, focal)
            } else {
                my_box.push((label, focal))
            }
        }
    }


    (0..256).map(|i| {
        let my_box : &Vec<(String, usize)> = hash_map.get(&i).unwrap();
        my_box.iter().enumerate().map(|(slot, (_label, focus))| ((i+1) as usize) * (slot+1) * focus).sum::<usize>()
    }).sum::<usize>()
}

fn decode(str: &str) -> u32 {
    let mut current_value = 0;

    for c in str.chars() {
        current_value  += c as u32;
        current_value  *= 17;
        current_value  =  current_value - ((current_value / 256) * 256);
    }
    current_value
}

fn read_input(filename: &String) ->  io::Result<String> {
    let file_in = File::open(filename)?;
    Ok(BufReader::new(file_in).lines().next().unwrap().unwrap())
}
