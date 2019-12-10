use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn day_1() {
    let result = read_a_file();
    match result {
        Ok(v) => println!("Sum is: {}", v),
        Err(e) => println!("error parsing header: {}", e),
    }
}

fn read_a_file() -> std::result::Result<i32, &'static str> {
    let mut file = File::open("inp1").unwrap();

    let reader = BufReader::new(file);
    let integers: Vec<i32> = reader.lines().into_iter().map(|d| {
        let number = d.unwrap().parse::<i32>().unwrap();
        let fuel = calculate_fuel(number);
        println!("number is {} fuel is {}", &number, &fuel);
        return fuel;
    }).collect();
    return Ok(integers.into_iter().sum());
}

fn calculate_fuel(number: i32) -> i32 {
    let fuel = number / 3 - 2;
    if fuel < 9 {
        return fuel;
    }
    else {
        return fuel + calculate_fuel(fuel);
    }
}
