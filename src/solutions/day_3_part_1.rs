use std::{fs::File, io::{BufRead, BufReader}};

use regex::Regex;

pub fn execute(filepath: String){
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let msg_err = format!("Unable to open file: {}", filepath);
    let file = File::open(filepath).expect(&msg_err);
    let lines = BufReader::new(file).lines();

    let mut result:i32 = 0;

    for line_result in lines{
        let line = line_result.unwrap();    
        
        for (_, [number_1, number_2]) in re.captures_iter(&line).map(|c| c.extract()) {
            result += number_1.parse::<i32>().unwrap()*number_2.parse::<i32>().unwrap();
        }
    }
    println!("Result is {}", result);
}