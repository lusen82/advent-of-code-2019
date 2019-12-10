use std::io::{BufReader, BufRead};
use std::fs::File;
use core::fmt;
use std::error::Error;

pub fn day_3() -> std::result::Result<(), MyError>{
    let mut file = File::open("inp3.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let first = lines.next();
    let second = lines.next();
    let mut coordinates_1 = get_coordinates(&first.unwrap().unwrap());
    coordinates_1.sort_by(|c, d| manhattan_distance(c).cmp(&manhattan_distance(d)));

    let mut coordinates_2 = get_coordinates(&second.unwrap().unwrap());
    coordinates_2.sort_by(|c, d| manhattan_distance(c).cmp(&manhattan_distance(d)));

    for iter in 1..coordinates_1.len() {
        for jiter in 1..coordinates_2.len() {
            let c1 = coordinates_1.get(iter).unwrap();
            let c2 = coordinates_2.get(jiter).unwrap();
            if  c1.0 == c2.0 && c1.1 == c2.1 {
                println!("Found smallest coordinate {}, {}", c1.0, c1.1);
                return Ok(());
            };
        }
    }
    println!("Found nothing..");
    return Err(MyError::new(""));
}


pub fn day_3_b() -> std::result::Result<(), MyError>{
    let mut file = File::open("inp3.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let first = lines.next();
    let second = lines.next();
    let coordinates_1 = get_coordinates(&first.unwrap().unwrap());
    let coordinates_2 = get_coordinates(&second.unwrap().unwrap());
    let mut intersections = vec![];
    let mut steps_1 = 0;

    for coord1 in &coordinates_1 {
        steps_1 += 1;
        let mut steps_2 = 0;
        for coord2 in &coordinates_2 {
            steps_2 += 1;
            if  coord1.0 == coord2.0 && coord1.1 == coord2.1 {
                println!("Found equal coordinate {}, {}", coord1.0, coord1.1);
                intersections.push((coord1, steps_1 + steps_2));

            };
        }
    }
    println!("Found nothing.. {:?}", intersections);
    intersections.sort_by(|i,j|i.1.cmp(&j.1));
    println!("Found sorted.. {:?}", intersections);
    return Err(MyError::new(""));
}


#[derive(Debug)]
pub struct MyError {
    details: String
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError{details: msg.to_string()}
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}


fn manhattan_distance(coord: &(i32, i32)) -> i32{
    coord.0.abs() + coord.1.abs()
}



enum Dir {
    RIGHT,
    UP,
    LEFT,
    DOWN
}

fn get_coordinates(line: &String) -> Vec<(i32, i32)>{
    let split: Vec<&str> = line.split(',').collect();
    let mut current_coordinate = (0,0);
    let mut all_coordinates: Vec<(i32, i32)> = vec![];
    let coordinates = split.into_iter().for_each(|instr| {
        let (head, tail) = instr.split_at(1);
        let dir = match head {
            "R" => Dir::RIGHT,
            "U" => Dir::UP,
            "L" => Dir::LEFT,
            "D" => Dir::DOWN,
            _ => panic!("No direction")
        };
        let steps = tail.parse::<i32>().unwrap();
        let old_curr_coord = current_coordinate;
        current_coordinate = match dir
            {
                Dir::RIGHT => {
                    // println!("RIGHT {}", steps);
                    for st in 1..steps + 1 {
                        all_coordinates.push((old_curr_coord.0 + st, old_curr_coord.1));
                    }
                    (old_curr_coord.0 + steps, old_curr_coord.1 )
                },
                Dir::UP => {
                    // println!("UP {}", steps);
                    for st in 1..steps + 1 {
                        all_coordinates.push((old_curr_coord.0 , old_curr_coord.1 + st));
                    }
                    (old_curr_coord.0 , old_curr_coord.1 + steps)
                },
                Dir::LEFT => {
                    // println!("LEFT {}", steps);
                    for st in 1..steps + 1 {
                        all_coordinates.push((old_curr_coord.0 - st, old_curr_coord.1 ));
                    }
                    (old_curr_coord.0 - steps, old_curr_coord.1)
                },
                Dir::DOWN => {
                    //  println!("DOWN {}", steps);
                    for st in 1..steps + 1 {
                        all_coordinates.push((old_curr_coord.0, old_curr_coord.1 - st));
                    }
                    (old_curr_coord.0, old_curr_coord.1 - steps)
                },
                _ => panic!("No direction")
            };
        //  println!("Current coordinate {:#?}", current_coordinate);
    });
    all_coordinates
}

