use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

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
    0
}

fn solve1(problem: &Problem) -> usize {
    println!("{:?}", problem);

    let known_parts: Vec<char> = vec!['x', 'm', 'a', 's'];
    let mut accepted_parts: HashMap<char, usize> =  HashMap::new();
    for known_part in &known_parts {
        accepted_parts.insert(*known_part, 0);
    }

    for input in &problem.inputs {
        println!("input: {:?}", input);

        let mut workflow = problem.workflows.get("in");
        assert!(workflow.is_some());
        loop {

            if workflow.is_none() {
                break;
            }

            let workflow_ = workflow.unwrap();
            println!("workflow: {:?}", workflow_);
            for rule in &workflow_.rules {
                if rule.matches(input) {
                    println!("matched: {:?}", rule.result.as_str());
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

    println!("{:?}", accepted_parts);

    accepted_parts.iter().map(|(_,v)| v).sum()
}

#[derive(Clone, Debug)]
struct Problem {
    workflows: HashMap<String , Workflow>,
    inputs: Vec<Input>,
}

#[derive(Clone, Debug)]
struct Workflow {
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