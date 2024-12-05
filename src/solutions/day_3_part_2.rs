use std::fs::read_to_string;

use regex::Regex;

pub fn execute(filepath: String){
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let re_do: Regex = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don\'t\(\)").unwrap();

    let lines:Vec<_> = read_to_string(filepath).map(|x| x.chars().collect::<Vec<char>>()).unwrap();

    let mut result:i64 = 0;
    let mut collect_do = true;
    let window_size = 12;

    let mut ignore_index=0; // this is a tracker. Marks the initial index after a given window encounters a mul match. 
    // If we don't do something like this the code might consider multiple matches while the window slides
    // For example when mul happens for small numbers "!@#dbmul(3,4)" -> "@#dbmul(3,4)b" -> "#dbmul(3,4)bv"

    lines.to_vec().windows(window_size).enumerate().for_each(|(i,x)| {
            // build the word from chars.
            // yes, yes.. it would be nice to use only the char references
            // but honestly that would be a lot of work for now.
            // This word var is used as a buffer, there's no high usage of memory,
            // only the penalty of malloc happens.
            let word:String = x.iter().collect();

            // Am I collecting and found a dont? stop collecting
            if collect_do && re_dont.is_match(&word){
                // "Found a don't at position {}", i);
                collect_do = false;
            }
            // Am I not collecting and found a do? Go back to collect.
            else if !collect_do && re_do.is_match(&word){
                // println!("Found a do at position {}", i);
                collect_do = true;
            }

            // If I must collect and don't need to skip this window
            // And it's a mul match
            if collect_do && i >= ignore_index && re.is_match(&word){
                // println!("Found a mul at position {}", i);
                // operate and save on result variable
                for (_, [number_1, number_2]) in re.captures_iter(&word).map(|c| c.extract()) {
                    result += number_1.parse::<i64>().unwrap()*number_2.parse::<i64>().unwrap();
                }
                // here I'm tracking the end of this window
                // skipping the window entirely after detection 
                // so we don't get duplicate matches.
                ignore_index = i+window_size;
            }
    });
    println!("Result is {}", result);
}