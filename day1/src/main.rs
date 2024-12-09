use std::collections::HashMap;
use std::fs;
use std::io;
use regex::Regex;

const FILE: &str = "./index2.txt";
fn main() {

    let items: Vec<String> = read_file_to_vect(FILE);
    let (vec1, vec2) = create_vects(items);
    println!("{:?}", vec1);
    println!("{:?}", vec2);
    let sum: i32 = get_counts(vec1, vec2);
    // for i in 0..vec1.len() {
    //     let vec1_num: i32 = vec1[i];
    //     let vec2_num: i32 = vec2[i];
    //     let distance: i32 = vec1_num - vec2_num;
    //     sum += distance.abs();
    // }
    println!("{}", sum.to_string())
}

fn read_file_to_vect(filepath: &str) -> Vec<String> {
    let contents: Result<String, io::Error> = fs::read_to_string(filepath);
    contents.expect("Didn't load correctly").lines().map(|line| line.to_string()).collect()
}

fn get_counts(vec_source: Vec<i32>, vec_target: Vec<i32>) -> i32 {
    let mut target_counts: HashMap<i32, usize> = HashMap::new();
    for e in vec_target {
        *target_counts.entry(e).or_default() += 1;
    }
    let mut sum = 0;
    for e in vec_source {
        let occurances: &usize = target_counts.get(&e).unwrap_or(&0);
        sum += e * *occurances as i32;
    }
    sum
}

fn create_vects(items: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let r_numbers: Regex = Regex::new(r"\d+").unwrap();
    let items_len: usize = items.len();

    let mut vec1: Vec<i32> = vec![0; items_len];
    let mut vec2: Vec<i32> = vec![0; items_len];
    
    for (i, e) in items.iter().enumerate() {
        let numbers: Vec<_> = r_numbers.find_iter(e).map(|m| m.as_str()).collect();

        vec1[i] = numbers[0].parse().unwrap();
        vec2[i] = numbers[1].parse().unwrap();
    }

    vec1.sort();
    vec2.sort();

    (vec1, vec2)
}




