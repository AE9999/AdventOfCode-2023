use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let problem = read_input(input)?;

    println!("What do you get if you multiply the total number of low pulses sent by the total number of high pulses sent? {:?}",
             solve1(&problem));

    Ok(())
}

fn solve1(problem: &Problem) -> usize {
    println!("problem: {:?}", problem);

    let mut flip_flop_states: HashMap<String, FlipFlopState> = HashMap::new();
    let mut conjunction_states: HashMap<String, HashMap<String, Pulse>> = HashMap::new();

    problem.modules.iter().filter(|(_k, v)| v == &&ModuleType::FlipFlop)
                          .for_each(|(k, _)| {
                              flip_flop_states.insert(k.clone(), FlipFlopState::Off);
                          });

    problem.modules.iter().filter(|(_, v)| v == &&ModuleType::Conjunction)
                           .for_each(|(conjunction_name, _)| {
                               conjunction_states.insert(conjunction_name.clone(), HashMap::new());
                               problem.connections.iter()
                                                  .filter(|(_, outgoing)| outgoing.contains(conjunction_name))
                                                  .for_each(|(name, _)| {
                                                      conjunction_states.get_mut(conjunction_name).unwrap().insert(name.clone(), Pulse::Low);
                                                  });

                           });

    let mut send_low: usize = 0;
    let mut send_high: usize = 0;

    for _ in 0..1000 {
        let mut queue: VecDeque<(String, String, Pulse)> = VecDeque::new();
        queue.push_back((String::from("button"), String::from("broadcaster"), Pulse::Low));
        while let Some((source, destination, pulse) ) = queue.pop_front() {
            println!("source: {:?}, destination: {:?}, pulse: {:?}", source, destination, pulse);
            match pulse {
                Pulse::Low => {
                    send_low += 1;
                },
                Pulse::High => {
                    send_high += 1;
                },
            }

            let connections = problem.connections.get(destination.as_str());
            if connections.is_none() {
                continue // output has no stuff
            }

            let connections = connections.unwrap();

            match problem.modules.get(destination.as_str()).unwrap() {
                ModuleType::Broadcaster => {
                    for connection in connections {
                        queue.push_back((destination.clone(),
                                               connection.clone(),
                                               pulse.clone()));
                    }
                },
                ModuleType::FlipFlop => {
                    match pulse {
                        Pulse::Low => {
                            match flip_flop_states.get(destination.as_str()).unwrap() {
                                FlipFlopState::On => {
                                    flip_flop_states.insert(destination.clone(), FlipFlopState::Off);
                                    for connection in connections {
                                        queue.push_back((destination.clone(),
                                                               connection.clone(),
                                                               Pulse::Low));
                                    }
                                },
                                FlipFlopState::Off => {
                                    flip_flop_states.insert(destination.clone(), FlipFlopState::On);
                                    for connection in connections {
                                        queue.push_back((destination.clone(),
                                                               connection.clone(),
                                                               Pulse::High));
                                    }
                                }
                            }
                        },
                        Pulse::High => {
                            // Ignore
                        }
                    }
                },
                ModuleType::Conjunction => {
                    let conjunction_states =
                        conjunction_states.get_mut(destination.as_str()).unwrap();
                    conjunction_states.insert(source.clone(), pulse.clone());
                    let has_low = conjunction_states.iter().find(|(_,v)| v == &&Pulse::Low);
                    let next_pulse =
                        if has_low.is_some() {
                            Pulse::High
                        } else {
                            Pulse::Low
                        };
                    for connection in connections {
                        queue.push_back((destination.clone(),
                                         connection.clone(),
                                         next_pulse.clone()));
                    }
                },
            }
        }
    }

    println!("send_low: {:?}, send_hight: {:?}", send_low, send_high);

    send_low  * send_high // button doesn't count

}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction
}

#[derive(Clone, Debug)]
struct Problem {
    connections: HashMap<String, Vec<String>>,
    modules: HashMap<String, ModuleType>
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
enum Pulse {
    High,
    Low,
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
enum FlipFlopState {
    On,
    Off
}

// #[derive(Debug, Clone)]
// struct State{
//     flip_flop_states: HashMap<String, FlipFlopState>,
//     conjunction_states: HashMap<String, HashMap<String, Pulse>>,
//     sending: HashMap<String, Pulse>,
// }

fn read_input(filename: &String) ->  io::Result<Problem> {
    let file_in = File::open(filename)?;
    let file_reader = BufReader::new(file_in).lines();

    let mut connections: HashMap<String, Vec<String>> = HashMap::new();
    let mut modules: HashMap<String, ModuleType> = HashMap::new();

    for line in file_reader {
        let line = line.unwrap();
        let mut it = line.split("->");
        let first = it.next().unwrap().trim();

        let module_type =
            match first.chars().next().unwrap() {
                '&' => ModuleType::Conjunction,
                '%' => ModuleType::FlipFlop,
                'b' => ModuleType::Broadcaster,
                _ => panic!("unexpected input")
            };

        let module_name: String =
            match module_type {
                ModuleType::Broadcaster => first.chars().collect(),
                _ => first.chars().skip(1).collect(),
            };

        modules.insert(module_name.clone(), module_type);
        connections.insert(module_name,
                            it.next()
                                 .unwrap()
                                 .trim()
                                 .split(',')
                                 .map(|x|String::from(x.trim()))
                                 .collect::<Vec<String>>());

    }

    Ok(Problem {
        connections,
        modules
    })
}