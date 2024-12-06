use std::io;
use std::fs;
use ndarray::{Array, Array2, s};

const FILE: &str = "./index.txt";

fn main() {
    let content = read_file_to_vect(FILE);
    let arr = convert_to_array(content);
    println!("{:?}", arr);
    // let match_indexes = find_char(&arr, &'X');
    // println!("{:?}", match_indexes);
    // let sum = check_xmas(&arr, match_indexes);
    let match_a_indexes = find_char(&arr, &'A');
    println!("{:?}", match_a_indexes);
    let sum = check_x_mas(&arr, match_a_indexes);
    println!("{}", sum);
}

fn read_file_to_vect(filepath: &str) -> Vec<String> {
    let contents: Result<String, io::Error> = fs::read_to_string(filepath);
    contents.expect("Didn't load correctly").lines().map(|line| line.to_string()).collect()
}

fn convert_to_array(content: Vec<String>) -> Array2<char> {
    let height: usize = content.len();
    let width: usize = content[0].len();
    // println!("height: {}, width: {}", height, width);
    let mut arr: Array2::<char> = Array2::from_shape_vec((height, width), vec![' '; height * width]).expect("Didn't work");
    // println!("{:?}", arr);
    for (i, c) in content.iter().enumerate() {
        let c_chars: Vec<char> = c.chars().collect();
        let c_arr = Array::from_shape_vec(width, c_chars).expect("Error");
        // println!("{:?}", c_arr);
        arr.row_mut(i).assign(&c_arr);
    }
    arr
}

fn find_char(arr: &Array2<char>, character: &char) -> Vec<(usize, usize)> {
    let matches: Vec<(usize, usize)> = arr.indexed_iter().filter(|&((_, _), &val)| val == *character).map(|((i, j), _)| (i, j)).collect();
    matches
}

fn check_xmas(arr: &Array2<char>, indexes: Vec<(usize, usize)>) -> i32 {
    let mut sum: i32 = 0;

    let (height, width): (usize, usize) = (arr.shape()[0], arr.shape()[1]);

    let xmas: Array<char, _> = Array::from_vec(vec!['X', 'M', 'A', 'S']);
    let samx: Array<char, _> = Array::from_vec(vec!['S', 'A', 'M', 'X']);
    for (i,j) in indexes {
        // N
        if i >= 3 {
            let n_slice = arr.slice(s![i-3..i+1, j]);
            if n_slice == xmas || n_slice == samx {
                sum += 1;
                println!("{:?}", n_slice);
            }
            // println!("{:?}", n_slice);
        }
        // NE
        if i >= 3 && j+4 <= width {
            let temp_ne_slice = arr.slice(s![i-3..i+1, j..j+4]);
            let ne_slice_vec: Vec<char> = temp_ne_slice.indexed_iter().filter(|((i, j), _)| i + j == 3).map(|(_, &val)| val).collect();
            let ne_slice = Array::from_vec(ne_slice_vec);
            if ne_slice == xmas || ne_slice == samx {
                println!("{:?}", temp_ne_slice);
                sum += 1;
                println!("{:?}", ne_slice);
            }
        }
        // E
        if j+4 <= width {
            let e_slice = arr.slice(s![i, j..j+4]);
            if e_slice == xmas || e_slice == samx {
                sum += 1;
                println!("{:?}", e_slice);
            }
        }
        // SE
        if i+4 <= height && j+4 <= width {
            let temp_se_slice = arr.slice(s![i..i+4, j..j+4]);
            println!("{:?}", temp_se_slice);
            let se_slice_vec: Vec<char> = temp_se_slice.indexed_iter().filter(|((i, j), _)| i == j).map(|(_, &val)| val).collect();
            let se_slice = Array::from_vec(se_slice_vec);
            if se_slice == xmas || se_slice == samx {
                println!("{:?}", temp_se_slice);
                sum += 1;
                println!("{:?}", se_slice);
            }
        }
        // S
        if i+4 <= height {
            let s_slice = arr.slice(s![i..i+4, j]);
            if s_slice == xmas || s_slice == samx {
                sum += 1;
                println!("{:?}", s_slice);
            }
        }
        // SW
        if i+4 <= height && j >= 3 {
            let temp_sw_slice = arr.slice(s![i..i+4, j-3..j+1]);
            println!("{:?}", temp_sw_slice);
            let sw_slice_vec: Vec<char> = temp_sw_slice.indexed_iter().filter(|((i, j), _)| i + j == 3).map(|(_, &val)| val).collect();
            let sw_slice = Array::from_vec(sw_slice_vec);
            if sw_slice == xmas || sw_slice == samx {
                println!("{:?}", temp_sw_slice);
                sum += 1;
                println!("{:?}", sw_slice);
            }
        }
        // W
        if j >= 3 {
            let w_slice = arr.slice(s![i, j-3..j+1]);
            // println!("{:?}", w_slice);
            if w_slice == xmas || w_slice == samx {
                sum += 1;
                println!("{:?}", w_slice);
            }
        }
        // NW
        if i >= 3 && j >= 3 {
            let temp_nw_slice = arr.slice(s![i-3..i+1, j-3..j+1]);
            println!("{:?}", temp_nw_slice);
            let nw_slice_vec: Vec<char> = temp_nw_slice.indexed_iter().filter(|((i, j), _)| i == j).map(|(_, &val)| val).collect();
            let nw_slice = Array::from_vec(nw_slice_vec);
            if nw_slice == xmas || nw_slice == samx {
                println!("{:?}", temp_nw_slice);
                sum += 1;
                println!("{:?}", nw_slice);
            }
        }
    }
    sum
}

fn check_x_mas(arr: &Array2<char>, indexes: Vec<(usize, usize)>) -> i32 {
    let mut sum: i32 = 0;
    let (height, width): (usize, usize) = (arr.shape()[0], arr.shape()[1]);

    let sam: Vec<char> = vec!['S', 'A', 'M'];
    let mas: Vec<char> = vec!['M', 'A', 'S'];

    for (i,j) in indexes {
        if i >= 1 && j >= 1 && i+1 < height && j+1 < width {
            let slice = arr.slice(s![i-1..i+2, j-1..j+2]);
            println!("{:?}", slice);
            
            let diag: Vec<char> = slice.indexed_iter().filter(|((i, j), _)| i == j).map(|(_, &val)| val).collect();
            let rev_diag: Vec<char> = slice.indexed_iter().filter(|((i, j), _)| i + j == 2).map(|(_, &val)| val).collect();

            println!("{:?}", diag);
            println!("{:?}", rev_diag);

            if diag == mas && rev_diag == mas {
                sum += 1;
            }
            if diag == mas && rev_diag == sam {
                sum += 1;
            }
            if diag == sam && rev_diag == mas {
                sum += 1;
            }
            if diag == sam && rev_diag == sam {
                sum += 1;
            }
        }
    }
    sum
}


