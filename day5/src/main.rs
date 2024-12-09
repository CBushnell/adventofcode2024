use std::io;
use std::fs;
use regex::{Regex, Captures};
use std::collections::{HashMap, HashSet};

const FILE: &str = "./input.txt";

#[derive(Clone)]
#[derive(Debug)]
struct Rule {
    first: i32,
    second: i32,
}

impl Rule {
    fn new(first: i32, second: i32) -> Self {
        Self {first, second}
    }

    fn new_from_string(string: &str) -> Self {
        let r_values: Regex = Regex::new(r"(?<first>\d+)\|(?<second>\d+)").unwrap();
        let cap: Captures<'_> = r_values.captures(string).unwrap();
        let first: i32 = cap.name("first").unwrap().as_str().parse().unwrap();
        let second: i32 = cap.name("second").unwrap().as_str().parse().unwrap();
        Self {first, second}
    }
}
fn main() {
    let content = read_file_to_vect(FILE);
    let (rules, values) = get_rules_and_values(content);
    let sum: i32 = assess_values(rules, values);
    println!("{}", sum);
}

fn read_file_to_vect(filepath: &str) -> Vec<String> {
    let contents: Result<String, io::Error> = fs::read_to_string(filepath);
    contents.expect("Didn't load correctly").lines().map(|line| line.to_string()).collect()
}

fn get_rules_and_values(content: Vec<String>) -> (HashMap<i32, Vec<Rule>>, Vec<Vec<i32>>) {
    let r_rule_or_list: Regex = Regex::new(r"(?<rule>\d+\|\d+)|(?<list>(\d+),((?:(\d+),)+)(\d+))").unwrap();
    let r_digit: Regex = Regex::new(r"(?<digit>\d+),?").unwrap();

    let mut rules: HashMap<i32, Vec<Rule>> = HashMap::new();
    let mut values: Vec<Vec<i32>> = Vec::new();

    for c in content {
        let cap: Captures<'_> = match r_rule_or_list.captures(&c) {
            Some(m) => m,
            None => continue,
        };
        if cap.name("rule") != None {
            let rule_str: &str = cap.name("rule").unwrap().as_str();
            // println!("{}", rule_str);
            let rule: Rule = Rule::new_from_string(rule_str);
            let first_rule_vec: &mut Vec<Rule> = rules.entry(rule.first).or_insert(Vec::new());
            first_rule_vec.push(rule);
        } else if cap.name("list") != None {
            let list_str: &str = cap.name("list").unwrap().as_str();
            let mut value_vec: Vec<i32> = Vec::new();
            // println!("{:?}", list_str);
            for m in r_digit.captures_iter(list_str) {
                value_vec.push(m.name("digit").unwrap().as_str().parse().unwrap());
            }
            values.push(value_vec);
        }
    }

    println!("{:?}", rules);
    println!("{:?}", values);

    (rules, values)
}

fn assess_values(rules: HashMap<i32, Vec<Rule>>, values: Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    let default_rule_vec: Vec<Rule> = Vec::new();
    let part_two = true;

    for value_vec in values {
        let mut broken_rules: Vec<Rule> = Vec::new();
        println!("For the values: {:?}", value_vec);
        let maybe_relevant_rules: Vec<_> = value_vec.iter().map(|v| rules.get(v).unwrap_or(&default_rule_vec)).flat_map(|v| v.iter()).collect();
        let mut relevant_rules: Vec<Rule> = Vec::new();
        // println!("The potentially relevant rules are: {:?}", maybe_relevant_rules);
        // cull relevant rules to the ones that only have values present
        let value_set: HashSet<i32> = value_vec.clone().into_iter().collect();
        for r in maybe_relevant_rules {
            if value_set.contains(&r.first) && value_set.contains(&r.second) {
                relevant_rules.push(Rule::new(r.first, r.second));
            }
        }
        // println!("The actually relevant rules are: {:?}", relevant_rules);
        // iterate on relevant rules and determine whether the value_vec adheres
        let mut valid: bool = true;
        // apologies for this one
        let relevant_rules_clone = relevant_rules.clone();
        for r in &relevant_rules {
            let first_idx: usize = value_vec.iter().position(|x| *x == r.first).unwrap();
            let second_idx: usize = value_vec.iter().position(|x| *x == r.second).unwrap();
            if first_idx > second_idx {
                println!("{:?} breaks the validity", r);
                if part_two {
                    valid = false;
                    broken_rules.push(r.clone());
                } else {
                    valid = false;
                    break
                }
            } 
        }
        if valid && !part_two {
            let middle_idx = value_vec.len() / 2;
            sum += value_vec[middle_idx];
            println!("Added {}", value_vec[middle_idx]);
        } else if !valid && part_two {
            let mut fixed_value_vec: Vec<i32> = value_vec.clone();
            let mut rule_map: HashMap<usize, Vec<usize>> = HashMap::new();
            for r in &broken_rules {
                let rule_entry: &mut Vec<usize> = rule_map.entry(fixed_value_vec.iter().position(|x| *x == r.first).unwrap()).or_insert(Vec::new());
                rule_entry.push(fixed_value_vec.iter().position(|x| *x == r.second).unwrap());
            }
            for first_i in rule_map.keys() {
                let second_i = rule_map.get(first_i).unwrap().iter().min().unwrap();
                let hold_first = fixed_value_vec[*first_i];
                fixed_value_vec[*first_i] = fixed_value_vec[*second_i];
                fixed_value_vec[*second_i] = hold_first;
            }
            for r in &relevant_rules_clone {
                let first_idx: usize = fixed_value_vec.iter().position(|x| *x == r.first).unwrap();
                let second_idx: usize = fixed_value_vec.iter().position(|x| *x == r.second).unwrap();
                if first_idx > second_idx {
                    // println!("{:?} breaks the validity", r);
                }
            }
            println!("Adjusted to {:?}", fixed_value_vec);
            let middle_idx = fixed_value_vec.len() / 2;
            sum += fixed_value_vec[middle_idx];
            println!("Added {}", fixed_value_vec[middle_idx]);
        }
    }
    sum
}


