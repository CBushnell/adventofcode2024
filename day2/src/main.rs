use std::fs;
use std::io;
use regex::Regex;
use std::collections::HashSet;

const FILE: &str = "./input2.txt";

fn main() {
    let elements = read_file_to_vect(FILE);
    let sum: i32 = process_reports(elements);
    println!("{}", sum);
}

fn read_file_to_vect(filepath: &str) -> Vec<String> {
    let contents: Result<String, io::Error> = fs::read_to_string(filepath);
    contents.expect("Didn't load correctly").lines().map(|line| line.to_string()).collect()
}

fn process_reports(reports: Vec<String>) -> i32 {
    let r_value: Regex = Regex::new(r"\d+").unwrap();
    let mut sum: i32 = 0;

    for r in reports {
        let values: Vec<i32> = r_value.find_iter(&r).map(|x| x.as_str().parse().unwrap()).collect();
        println!("{:?}", values);
        let valid = brute_force_assess(values);
        println!("{}", valid);
        sum += valid;
    }
    sum
}

// Attempted to use recursion but kept coming up with the wrong answers
// After solving the challenge, I don't think the approach was wrong but I instead missed the edge case of the number of increasing and decreasing elements being equivalent
//
// fn assess_report(report: Vec<i32>, dampened: bool) -> i32 {
//     //All inputs verified to be n>2 in length
//     let report_len: usize = report.len();
//     let mut distances: Vec<i32> = vec![0; report_len - 1];

//     let mut bad_levels: HashSet<usize> = HashSet::new();

//     for i in 1..report_len {
//         let distance: i32 = report[i] - report[i-1];
//         //increase/decrease by at least 1 but less than 3 - terminate early if not
//         if (distance.abs() > 3) || (distance == 0) {
//             if dampened {
//                 return 0
//             } 
//             bad_levels.extend(&vec![i - 1, i]);
//         }
//         distances[i - 1] = distance;
//     }
//     //all values are increasing or decreasing
//     let inc_idx: Vec<usize> = distances.iter().enumerate().filter(|(_, x)| **x > 0).map(|(i, _)| i).collect();
//     let dec_idx: Vec<usize> = distances.iter().enumerate().filter(|(_, x)| **x < 0).map(|(i, _)| i).collect();

//     let inc_len = inc_idx.len();
//     let dec_len = dec_idx.len();

//     // increasing, checking for decreasing elements 
//     if inc_len > dec_len {
//         if dec_idx.len() > 0 {
//             //there is another issue after removing a problematic index
//             if dampened {
//                 return 0
//             }
//             let dec_bad_levels: Vec<usize> = dec_idx.into_iter().flat_map(|i| vec![i, i+1]).collect();
//             bad_levels.extend(dec_bad_levels);
//         }
//     }
//     // decreasing, checking for increasing elements
//     if inc_len < dec_len {
//         if inc_len > 0 {
//             //there is another issue after removing a problematic index
//             if dampened {
//                 return 0
//             }
//             let inc_bad_levels: Vec<usize> = inc_idx.into_iter().flat_map(|i| vec![i, i+1]).collect();
//             bad_levels.extend(inc_bad_levels);
//         }
//     }
//     // bad levels should = 1 if we're in a passable dampened state
//     if bad_levels.len() > 0 {
//         if dampened {
//             return 0
//         }
//         let mut damepend_attempt_sum: i32 = 0;
//         for i in bad_levels {
//             let mut dampened_report: Vec<i32> = report.clone();
//             dampened_report.remove(i);
//             damepend_attempt_sum += assess_report(dampened_report, true);
//         }
//         if damepend_attempt_sum > 0 {
//             return 1
//         }
//         return 0
//     }
//     return 1
// }

fn brute_force_assess(report: Vec<i32>) -> i32 {
    let report_len: usize = report.len();

    //get all iterations of the report with a record removed
    let mut vec_of_reports: Vec<Vec<i32>> = Vec::new();
    vec_of_reports.push(report.clone());
    for i in 0..report_len {
        let mut vec_w_remove = report.clone();
        vec_w_remove.remove(i);
        vec_of_reports.push(vec_w_remove);
    }
    for r in vec_of_reports {
        println!("{:?}", r);
        let report_len: usize = r.len();
        let mut distances: Vec<i32> = vec![0; report_len - 1];
        let mut invalid = false;
        for i in 1..report_len {
            let distance: i32 = r[i] - r[i-1];
            //increase/decrease by at least 1 but less than 3 - terminate early if not
            if (distance.abs() > 3) || (distance == 0) {
                println!{"{}, {} greater than 3 or == 0", r[i-1], r[i]};
                invalid = true;
                break
            }
            distances[i - 1] = distance;
        }
        if invalid {
            continue;
        }
          //all values are increasing or decreasing
        let inc_idx: Vec<usize> = distances.iter().enumerate().filter(|(_, x)| **x > 0).map(|(i, _)| i).collect();
        let dec_idx: Vec<usize> = distances.iter().enumerate().filter(|(_, x)| **x < 0).map(|(i, _)| i).collect();

        let inc_len = inc_idx.len();
        let dec_len = dec_idx.len();

        // increasing, checking for decreasing elements 
        if inc_len > dec_len {
            println!("increasing");
            if dec_len > 0 {
                continue;
            }
        }
        // decreasing, checking for increasing elements
        if inc_len < dec_len {
            println!("decreasing");
            if inc_len > 0 {
               continue;
            }
        }
        if inc_len == dec_len {
            println!("equiv");
            continue;
        }
        return 1
    }
    return 0
}