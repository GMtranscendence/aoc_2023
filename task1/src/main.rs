use std::fs::{read_to_string, File};
use std::io::{BufReader, BufRead};
use std::vec::Vec;
use std::collections::HashMap;

fn part1() -> u32 {
    let mut calibration_values: Vec<u32> = Vec::new();
    let contents = read_to_string("./input")
        .expect("Error with reading the input file");
    let mut first_num: char = '0';
    let mut second_num: char = '0';
    let mut found_first_num: bool = false;
    for char in contents.chars() {
        if char.is_digit(10){
            second_num = char;
            if !found_first_num {
                first_num = char;
                found_first_num = true;
            }
        }
        else if char == '\n'{
            found_first_num = false;
            let mut concatenated = String::new();
            concatenated.push(first_num);
            concatenated.push(second_num);
            let concat_vals: u32 = concatenated.parse().expect("Can't convert");
            calibration_values.push(concat_vals);
        }
    }
    let sum: u32 = calibration_values.iter().sum();
    return sum;
}

fn convert_to_nums(line: String) -> u32 {
    let nums: HashMap<&str, u32> = HashMap::from([
        ("zero", 0),
        ("0", 0), 
        ("one", 1), 
        ("1", 1), 
        ("two", 2), 
        ("2", 2), 
        ("three", 3), 
        ("3", 3), 
        ("four", 4), 
        ("4", 4), 
        ("five", 5), 
        ("5", 5), 
        ("six", 6), 
        ("6", 6), 
        ("seven", 7), 
        ("7", 7), 
        ("eight", 8), 
        ("8", 8), 
        ("nine", 9), 
        ("9", 9),
    ]);
    let mut max = (-f64::INFINITY, "");
    let mut min = (f64::INFINITY, "");
    for (num_str, _) in &nums {
        let indices = line.match_indices(num_str).into_iter().collect::<Vec<_>>();
        if indices == [] { continue; }
        for index in &indices{
            if (index.0 as f64) > max.0 { max = (index.0 as f64, index.1); }
            if (index.0 as f64) < min.0 { min = (index.0 as f64, index.1); }
        }
    }
    return nums.get(min.1).unwrap()*10+nums.get(max.1).unwrap();
}


fn part2() -> u32{
    let mut sum: u32 = 0;
    let filepath: String = String::from("./input");
    let file = File::open(filepath).expect("Error with reading the input file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Error with reading a line");
        sum += convert_to_nums(line);
    }
    return sum;
}

fn main() {
    let part1_res: u32 = part1();
    println!("The first answer is... {part1_res}!");
    let part2_res: u32 = part2();
    println!("The second answer is... {part2_res}!");
}
