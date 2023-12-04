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
        "1" => result = get_sum_calibration(&input, part_num).to_string(),
        _ => println!("Invalid day number"),
    }
    println!("Result: {}", result);
}

fn get_sum_calibration(input: &str, part_num: &str) -> i32 {
    let mut sum = 0;
    let spelled_digits = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for line in input.lines() {
        let mut calibration_string = String::new();
        let mut first_digit = line.chars().find(|c| c.is_digit(10)).unwrap();
        let mut second_digit = line.chars().rev().find(|c| c.is_digit(10)).unwrap();
        println!("First digit: {}, Second digit: {}", first_digit, second_digit);
        if part_num == "2" {
            let mut first_digit_index = line.chars().position(|c| c == first_digit as char).unwrap();
            let mut second_digit_index = line.len() - line.chars().rev().position(|c| c == second_digit as char).unwrap() - 1;
            println!("First digit index: {}, Second digit index: {}", first_digit_index, second_digit_index);
            for number in spelled_digits.iter() {
                if !line.contains(number) {
                    continue;
                }
                let temp_index = line.find(number).unwrap();
                println!("FirstNumber: {}, Index: {}", number, temp_index);
                if temp_index < first_digit_index {
                    first_digit = char::from_digit(spelled_digits.iter().position(|r| r == number).unwrap() as u32 + 1, 10).unwrap();
                    first_digit_index = temp_index;
                }

                let temp_rev_index = line.rfind(number).unwrap();
                println!("SecondNumber: {}, Index: {}", number, temp_rev_index);
                if temp_rev_index > second_digit_index {
                    second_digit = char::from_digit(spelled_digits.iter().position(|r| r == number).unwrap() as u32 + 1, 10).unwrap();
                    second_digit_index = temp_rev_index;
                }
            }
        }
        calibration_string = first_digit.to_string();
        calibration_string.push(second_digit);
        println!("Calibration string: {}", calibration_string);
        sum += calibration_string.parse::<i32>().unwrap();
    }

    sum
}