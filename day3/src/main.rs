
use std::fs;
use std::io;
use regex::Regex;

const FILE: &str = "./input.txt";

fn main() {
    let contents = read_file_to_vect(FILE);
    let sum: i32 = get_instructions(contents);
    println!("{}", sum);
}

fn read_file_to_vect(filepath: &str) -> Vec<String> {
    let contents: Result<String, io::Error> = fs::read_to_string(filepath);
    contents.expect("Didn't load correctly").lines().map(|line| line.to_string()).collect()
}

fn get_instructions(contents: Vec<String>) -> i32 {
    let mut sum: i32 = 0;
    let re_mu = Regex::new(r"mul\((?<first>\d{1,3}),(?<second>\d{1,3})\)|(?<do>do\(\))|(?<dont>don't\(\))").unwrap();
    let mut active: bool = true;
    for c in contents {
        // println!("{:?}", c);
        for cap in re_mu.captures_iter(&c) {
            println!("{:?}", cap);
            if !cap.name("do").is_none() {
                active = true;
            }
            if !cap.name("dont").is_none() {
                active = false;
            }
            if !cap.name("first").is_none() && active{
                let first: i32 = cap.name("first").map_or("", |m| m.as_str()).parse().unwrap();
                let second: i32 = cap.name("second").map_or("", |m| m.as_str()).parse().unwrap();
                sum += first * second;
            }
        }
    }
    sum
}