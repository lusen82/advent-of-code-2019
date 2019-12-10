use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn day_2() {
    let mut file = File::open("inp2.txt").unwrap();
    let reader = BufReader::new(file);
    let inp: Vec<String> = reader.lines().into_iter().map(|d| {
        let numbers = d.unwrap();
        numbers
    }).collect();
    let first_line = inp.first().unwrap();

    for noun in (0..100) {
        for verb in (0..100) {
            let numbers_i: Vec<&str> = first_line.split(',').collect();
            //println!("n {:?}", &numbers_i);
            let numbers: Vec<usize> = numbers_i.into_iter().map(|integ| integ.parse::<usize>().unwrap()).collect();
            let mut iter = 0;
            let mut result_vec: Vec<usize> = numbers.clone();
            result_vec[1] = noun;
            result_vec[2] = verb;
            while iter < numbers.len() {
                let option = result_vec.get(iter).unwrap();
                match option {
                    1 => {
                        let first_val_ind = *result_vec.get(iter + 1).unwrap();
                        let second_val_ind = *result_vec.get(iter + 2).unwrap();
                        let to_repl_index = *result_vec.get(iter + 3).unwrap();
                        result_vec[to_repl_index] = result_vec.get(first_val_ind).unwrap() + result_vec.get(second_val_ind).unwrap();
                    },
                    2 => {
                        let first_val_ind = *result_vec.get(iter + 1).unwrap();
                        let second_val_ind = *result_vec.get(iter + 2).unwrap();
                        let to_repl_index = *result_vec.get(iter + 3).unwrap();
                        result_vec[to_repl_index] = result_vec.get(first_val_ind).unwrap() * result_vec.get(second_val_ind).unwrap();
                    },
                    99 => {
                        iter = numbers.len();
                    },
                    _ => panic!("Misunderstood")
                }
                iter += 4;
            }
            //println!("Vec res {:?}", &result_vec);
            let zero_after = result_vec[0];
            if zero_after == 19690720 {
                println!("Noun is {} and verb is {}", noun, verb);
                break;
            }
        }
    }
}
