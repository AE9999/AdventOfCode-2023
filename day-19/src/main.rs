use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;
use std::ops::Range;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let problem = read_input(input)?;

    println!("What do you get if you add together all of the rating numbers for all of the parts that ultimately get accepted {:?}",
             solve1(&problem));

    println!("How many distinct combinations of ratings will be accepted by the Elves' workflows? {:?}",
             solve2(&problem));

    Ok(())
}

fn solve2(problem: &Problem) -> usize {
    let mut problem = problem.clone();

    let workflow_a = Workflow {
        name: String::from("A"),
        rules: Vec::new()
    };

    let workflow_r = Workflow {
        name: String::from("R"),
        rules: Vec::new()
    };

    problem.workflows.insert(String::from("A"), workflow_a);

    problem.workflows.insert(String::from("R"), workflow_r);


    let known_parts: Vec<char> = vec!['x', 'm', 'a', 's'];
    let mut ranges: HashMap<char, Range<usize>> = HashMap::new();
    for known_part in &known_parts {
        ranges.insert(*known_part, 1..4001);
    }

    let workflow = problem.workflows.get("in").unwrap();

    let state = State {
        ranges,
        workflow,
        rule_index: 0,
    };

    let mut queue: VecDeque<State> = VecDeque::new();
    queue.push_back(state);

    let mut amount_of_possible_state: usize = 0;

    while let Some(state) = queue.pop_front() {

        if state.workflow.name == "R" {
            // all rejected
            continue
        } else if state.workflow.name == "A" {
            // all accepted
            let newly_discovered_states =
                known_parts.iter()
                           .map(|c| state.ranges.get(c).unwrap().len())
                           .fold(1, |acc, x| acc * x);
            amount_of_possible_state += newly_discovered_states
        } else {
            let rule = state.workflow.rules.get(state.rule_index).unwrap();

            if rule.condition.is_empty() {
                let state = State {
                    ranges: state.ranges,
                    workflow: problem.workflows.get(rule.result.as_str()).unwrap() ,
                    rule_index: 0,
                };
                queue.push_back(state);
            } else {
                let part = rule.condition.chars().next().unwrap();

                let operator: char = rule.condition.chars().skip(1).next().unwrap();

                let amount: usize = rule.condition.chars().skip(2).collect::<String>().parse::<usize>().unwrap();

                let effected_range = state.ranges.get(&part).unwrap();

                let ranges =
                    match operator {
                        '<' =>  (get_subrange_that_is_lower(effected_range,amount, false),
                                 get_subrange_that_is_higher(effected_range,amount, true)),
                        '>' => (get_subrange_that_is_higher(effected_range,amount, false),
                                get_subrange_that_is_lower(effected_range,amount, true)),
                        _  => panic!("unexpected state"),
                    };

                if ranges.0.is_some() {
                    // handle matching stuff
                    let mut next_ranges = state.ranges.clone();
                    next_ranges.insert(part, ranges.0.unwrap());
                    let state = State {
                        ranges: next_ranges,
                        workflow: problem.workflows.get(rule.result.as_str()).unwrap(),
                        rule_index: 0,
                    };
                    queue.push_back(state);
                }

                if ranges.1.is_some() {
                    // handle non-matching stuff.
                    let mut next_ranges = state.ranges.clone();
                    next_ranges.insert(part, ranges.1.unwrap());
                    let state = State {
                        ranges: next_ranges,
                        workflow: state.workflow,
                        rule_index: state.rule_index + 1,
                    };
                    queue.push_back(state);
                }
            }
        }

    }

    amount_of_possible_state
}

fn get_subrange_that_is_higher(range: &Range<usize>, bound: usize, inclusive: bool) -> Option<Range<usize>> {
    let offset = if inclusive { 0 } else { 1 };
    let r = (bound + offset)..range.end;
    if r.is_empty() {
        None
    } else {
        Some(r)
    }
}

fn get_subrange_that_is_lower(range: &Range<usize>, bound: usize, inclusive: bool) -> Option<Range<usize>> {
    let bound = if inclusive { bound + 1} else { bound };
    let r =
        if bound <= range.end {
            range.start..bound
        } else {
            range.clone()
        };

    if r.is_empty() {
        None
    } else {
        Some(r)
    }
}

#[derive(Clone, Debug)]
struct State<'a> {
    ranges: HashMap<char, Range<usize>>,
    workflow: &'a Workflow,
    rule_index: usize,
}

fn solve1(problem: &Problem) -> usize {

    let known_parts: Vec<char> = vec!['x', 'm', 'a', 's'];
    let mut accepted_parts: HashMap<char, usize> =  HashMap::new();
    for known_part in &known_parts {
        accepted_parts.insert(*known_part, 0);
    }

    for input in &problem.inputs {

        let mut workflow = problem.workflows.get("in");
        assert!(workflow.is_some());
        loop {

            if workflow.is_none() {
                break;
            }

            let workflow_ = workflow.unwrap();
            for rule in &workflow_.rules {
                if rule.matches(input) {

                    match rule.result.as_str() {
                        "A" => {
                            for known_part in &known_parts {
                                let mut amount_of_parts = *accepted_parts.get(known_part).unwrap();
                                amount_of_parts += *input.parts.get(known_part).unwrap();
                                accepted_parts.insert(*known_part, amount_of_parts);
                            }
                            workflow = None;
                        },
                        "R" => {
                            workflow = None;
                        }
                        _ => {
                            workflow = problem.workflows.get(rule.result.as_str());
                            assert!(workflow.is_some())
                        }
                    }
                    break;
                }
            }
        }
    }

    accepted_parts.iter().map(|(_,v)| v).sum()
}

#[derive(Clone, Debug)]
struct Problem {
    workflows: HashMap<String , Workflow>,
    inputs: Vec<Input>,
}

#[derive(Clone, Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

#[derive(Clone, Debug)]
struct Rule {
    condition: String,
    result: String,
}

impl Rule {
    fn matches(&self, input: &Input) -> bool {

        if self.condition.is_empty() {
            return true
        }

        let part = self.condition.chars().next().unwrap();
        let operator: char = self.condition.chars().skip(1).next().unwrap();
        let amount: usize = self.condition.chars().skip(2).collect::<String>().parse::<usize>().unwrap();

        let amount_of_part_in_input = *(input.parts.get(&part).unwrap());

        match operator {
            '<' =>  amount_of_part_in_input < amount,
            '>' => amount_of_part_in_input > amount,
            _  => panic!("unexpected state"),
        }
    }
    
}

#[derive(Clone, Debug)]
struct Input {
    parts: HashMap<char, usize>
}

fn read_input(filename: &String) ->  io::Result<Problem> {
    let file_in = File::open(filename)?;
    let file_reader = BufReader::new(file_in).lines();

    let mut parsing_workflows: bool = true;

    let mut workflows: HashMap<String , Workflow> = HashMap::new();
    let mut inputs: Vec<Input> = Vec::new();

    for line in file_reader {
        let line = line.unwrap();

        if line.is_empty() {
            parsing_workflows = false;
        } else if parsing_workflows {
            let mut it = line.split('{');
            let name: String = String::from(it.next().unwrap());
            let rest: String = String::from(it.next().unwrap().replace('}', ""));
            let it = rest.split(",");
            let mut rules: Vec<Rule> = Vec::new();
            for x in it {
                if x.contains(':') {
                    let mut it = x.split(':');
                    rules.push(Rule {
                        condition: String::from(it.next().unwrap()),
                        result: String::from(it.next().unwrap()),
                    });
                } else {

                    rules.push(Rule {
                        condition: String::from(""),
                        result: String::from(x),
                    });

                    let workflow = Workflow {
                        name: name.clone(),
                        rules,
                    };
                    workflows.insert(name, workflow);
                    break; // Guaranteed to be the last iteration
                }
            }
        } else {
            let mut parts: HashMap<char, usize> = HashMap::new();
            line.replace('{', "").replace('}', "")
                .split(',')
                .map(|s|{
                    let mut it = s.split('=');
                    let c: char = it.next().unwrap().chars().next().unwrap();
                    let amount: usize = it.next().unwrap().parse::<usize>().unwrap();
                    (c, amount)
                })
                .for_each(|(c, amount)| {
                    parts.insert(c, amount);
                });
            inputs.push(Input {
                parts
            })
        }
    }

    Ok(Problem {
        workflows,
        inputs
    })
}