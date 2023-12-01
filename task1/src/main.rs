use std::fs;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::vec::Vec;

fn part1() -> i32 {
    let mut calibration_values: Vec<i32> = Vec::new();
    let contents = fs::read_to_string("./input")
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
            let concat_vals: i32 = concatenated.parse().expect("Can't convert");
            calibration_values.push(concat_vals);
        }
    }
    let sum: i32 = calibration_values.iter().sum();
    return sum;
}

fn part2(){
    let filepath: String = String::from("./input");
    let file = File::open(filepath).expect("");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
    }
}

fn main() {
    let part1_res: i32 = part1();
    println!("The first answer is... {part1_res}!");
    part2();
}
