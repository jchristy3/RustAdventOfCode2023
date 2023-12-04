use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_num = &args[1];
    let part_num = &args[2];
    let file_path = &args[3];
    println!("Reading file: {}", file_path);
    let input = fs::read_to_string(file_path).expect("Error reading file"); 
    let mut result = String::new();
    match day_num.as_str() {
        "1" => {
            match part_num.as_str() {
                "1" => result = get_sum_calibration(&input).to_string(),
                _ => println!("Invalid part number"),
            }
        },
        _ => println!("Invalid day number"),
    }
    println!("Result: {}", result);
}

fn get_sum_calibration(input: &str) -> i32 {
    let mut sum = 0;
    let spelled_digits = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for line in input.lines() {
        let mut calibration_string = String::new();
        let first_digit = line.chars().find(|c| c.is_digit(10)).unwrap();
        let mut first_spelled_number: i32 = -1;
        let mut spelled_number_index = line.len();
        for number in spelled_digits.iter() {
            let number_found = line.find(number);
            if line.contains(number) && line.find(number).unwrap() < spelled_number_index {
                let spelled_number_index = line.find(number).unwrap();
                first_spelled_number = spelled_digits.iter().position(|r| r == number).unwrap() as i32;
            }
        }
        println!("{:?}", first_spelled_number);
        calibration_string.push(line.chars().find(|c| c.is_digit(10)).unwrap());
        calibration_string.push(line.chars().rev().find(|c| c.is_digit(10)).unwrap());
        sum += calibration_string.parse::<i32>().unwrap();
    }

    sum
}