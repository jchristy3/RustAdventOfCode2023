use std::env;
use std::fs;
use std::collections::HashMap;

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
        "2" => result = get_possible_game_sum(&input, part_num).to_string(),
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
        if part_num == "2" {
            let mut first_digit_index = line.chars().position(|c| c == first_digit as char).unwrap();
            let mut second_digit_index = line.len() - line.chars().rev().position(|c| c == second_digit as char).unwrap() - 1;
            for number in spelled_digits.iter() {
                if !line.contains(number) {
                    continue;
                }
                let temp_index = line.find(number).unwrap();
                if temp_index < first_digit_index {
                    first_digit = char::from_digit(spelled_digits.iter().position(|r| r == number).unwrap() as u32 + 1, 10).unwrap();
                    first_digit_index = temp_index;
                }

                let temp_rev_index = line.rfind(number).unwrap();
                if temp_rev_index > second_digit_index {
                    second_digit = char::from_digit(spelled_digits.iter().position(|r| r == number).unwrap() as u32 + 1, 10).unwrap();
                    second_digit_index = temp_rev_index;
                }
            }
        }
        calibration_string = first_digit.to_string();
        calibration_string.push(second_digit);
        sum += calibration_string.parse::<i32>().unwrap();
    }

    sum
}

fn get_possible_game_sum(input: &str, part_num: &str) -> i32 {
    let mut game_number_sum = 0; //our resulting value
    let mut expected_sum = HashMap::new();
    expected_sum.insert(String::from("red"), 12);
    expected_sum.insert(String::from("green"), 13);
    expected_sum.insert(String::from("blue"), 14);

    let mut actual_sum = HashMap::new();
    for line in input.lines() {
        let colon_position = line.chars().find(|c| c == ':').unwrap();
        let game_number = &line[5..colon_position];
        let rounds = line.split("; ").collect::<Vec<&str>>();
        
        // get sum of each color across all rounds
        for round in rounds.iter() {
            let color_sums = round.split(", ").collect::<Vec<&str>>();
            for color_sum in color_sums.iter() {
                let space_position = color_sum.chars().position(|w| w == ' ').unwrap();
                let color = &color_sum[..space_position];
                let total = &color_sum[space_position + 1..].parse::<i32>().unwrap();

                if actual_sum.contains_key(color) {
                    actual_sum.get(color) = actual_sum + total;
                } else {
                    actual_sum.insert(color, total);
                }
            }
        }

        //compare actual to expected
        for (key, value) in &expected_sum {
            if actual_sum.contains_key(key) {
                if actual_sum.get(key) == value {
                    game_number_sum += game_number.parse::<i32>().unwrap();
                }
            }
        }

        return game_number_sum;
    }
}