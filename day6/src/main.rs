use std::io;
use std::fs;
use ndarray::{Array2, s};

const FILE: &str = "./input.txt";

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let content = read_file_to_vect(FILE);
    let (world, starting_pos) = create_world(&content, None);
    println!("{:?}", world);
    let (sum, coord_vec) = act(world, starting_pos);
    println!("Normal sum is: {:?}", sum);

    let mut sum_loops: i32 = 0;
    for coord in &coord_vec {
        let (new_world, new_starting_pos) = create_world(&content, Some(*coord));
        let returned: (i32, Vec<(usize, usize)>) = act(new_world, new_starting_pos);
        if returned.0 == -1 {
            sum_loops += 1;
            println!("{}", sum_loops);
        }
    }
    println!("{} loops are possible", sum_loops);
}

fn read_file_to_vect(filepath: &str) -> Vec<String> {
    let contents: Result<String, io::Error> = fs::read_to_string(filepath);
    contents.expect("Didn't load correctly").lines().map(|line| line.to_string()).collect()
}


// 0 = Nothing
// -1 = Obstacle
// 1 = Traversed
fn create_world(content: &Vec<String>, new_obstacle: Option<(usize, usize)>) -> (Array2<i32>, (usize, usize)) {
    let (height, width) = (content.len(), content.get(0).unwrap().len());
    let mut world: Array2<i32> = Array2::zeros((height, width));
    let mut starting_pos: (usize, usize) = (0, 0);

    for (i, c_row) in content.iter().enumerate() { 
        let c_row_chars: Vec<char> = c_row.chars().collect();
        for (j, c) in c_row_chars.iter().enumerate() {
            if *c == '#' {
                world[[i ,j]] = -1;
            } else if *c == '^' {
                starting_pos = (i, j);
                world[[i ,j]] = 1;
            } else {
                world[[i ,j]] = 0;
            }
        }
    }
    match new_obstacle {
        Some(new_obstacle) => world[[new_obstacle.0, new_obstacle.1]] = -1,
        None => return (world, starting_pos),
    }
    (world, starting_pos)
}

fn act(mut world: Array2<i32>, starting_pos: (usize, usize)) -> (i32, Vec<(usize, usize)>) {
    let mut direction: Direction = Direction::North;
    let (height, width) = (world.shape()[0], world.shape()[1]);
    let (mut i, mut j) = starting_pos;
    // let (mut target_i, mut target_j) = starting_pos;
    let mut out_of_bounds: bool = false;
    let mut old_sum: usize = 0;
    let mut counter: i32 = 0;
    let mut coord_vec: Vec<(usize, usize)> = Vec::new();

    while !out_of_bounds {
        match direction {
            Direction::North => {
                let path: ndarray::ArrayBase<ndarray::ViewRepr<&i32>, ndarray::Dim<[usize; 1]>> = world.slice(s![0..i, j]);
                let obstacle_positions: Vec<usize> = path.iter().enumerate().filter_map(|(idx, x)| if *x == -1 { Some(idx) } else { None }).collect();
                let closest_pos_option: Option<usize> = obstacle_positions.iter().filter_map(|&x| Some(x)).max();
                if closest_pos_option == None {
                    out_of_bounds = true;
                    world.slice_mut(s![0..i+1, j]).fill(1);
                } else {
                    let closest_pos = closest_pos_option.unwrap();
                    world.slice_mut(s![closest_pos+1..i+1, j]).fill(1);
                    (i, j) = (closest_pos+1, j);
                    direction = Direction::East;
                }
            },
            Direction::East => {
                let path: ndarray::ArrayBase<ndarray::ViewRepr<&i32>, ndarray::Dim<[usize; 1]>> = world.slice(s![i, j..width]);
                let obstacle_positions: Vec<usize> = path.iter().enumerate().filter_map(|(idx, x)| if *x == -1 { Some(idx) } else { None }).collect();
                let closest_pos_option: Option<usize> = obstacle_positions.iter().filter_map(|&x| Some(x)).min();
                // println!("{:?}", closest_pos_option);
                if closest_pos_option == None {
                    out_of_bounds = true;
                    world.slice_mut(s![i, j..width]).fill(1);
                } else {
                    let closest_pos = closest_pos_option.unwrap();
                    world.slice_mut(s![i, j..j+closest_pos]).fill(1);
                    (i, j) = (i, j+closest_pos-1);
                    direction = Direction::South;
                }
            },
            Direction::South => {
                let path: ndarray::ArrayBase<ndarray::ViewRepr<&i32>, ndarray::Dim<[usize; 1]>> = world.slice(s![i..height, j]);
                let obstacle_positions: Vec<usize> = path.iter().enumerate().filter_map(|(idx, x)| if *x == -1 { Some(idx) } else { None }).collect();
                let closest_pos_option: Option<usize> = obstacle_positions.iter().filter_map(|&x| Some(x)).min();
                if closest_pos_option == None {
                    out_of_bounds = true;
                    world.slice_mut(s![i..height, j]).fill(1);
                } else {
                    let closest_pos = closest_pos_option.unwrap();
                    world.slice_mut(s![i..i+closest_pos, j]).fill(1);
                    (i, j) = (i+closest_pos-1, j);
                    direction = Direction::West;
                }
            },
            Direction::West => {
                let path: ndarray::ArrayBase<ndarray::ViewRepr<&i32>, ndarray::Dim<[usize; 1]>> = world.slice(s![i, 0..j]);
                let obstacle_positions: Vec<usize> = path.iter().enumerate().filter_map(|(idx, x)| if *x == -1 { Some(idx) } else { None }).collect();
                let closest_pos_option: Option<usize> = obstacle_positions.iter().filter_map(|&x| Some(x)).max();
                if closest_pos_option == None {
                    out_of_bounds = true;
                    world.slice_mut(s![i, 0..j+1]).fill(1);
                } else {
                    let closest_pos = closest_pos_option.unwrap();
                    world.slice_mut(s![i, closest_pos+1..j+1]).fill(1);
                    (i, j) = (i, closest_pos+1);
                    direction = Direction::North;
                }
            }
        }
        let running_sum: usize = world.iter().filter(|x| **x == 1).count();
        // println!("Turn! Going {:?}", direction);
        // println!("{:?}", &world);
        // println!("{:?}", running_sum);
        if running_sum == old_sum {
            counter += 1;
        }
        if counter > 4 {
            return (-1, coord_vec)
        }
        old_sum = running_sum.clone();
    }
    // println!("{:?}", world);
    let sum: usize = world.iter().filter(|x| **x == 1).count();
    let flatened_world:ndarray::iter::IndexedIter<'_, i32, ndarray::Dim<[usize; 2]>> = world.indexed_iter();
    for coordinate in flatened_world {
        let (index, value) = coordinate;
        if value == &1 {
            coord_vec.push(index);
        }
    }
    return (sum as i32, coord_vec);
}

