use std::{fs::File, io::{BufRead, BufReader}};
use std::collections::HashMap;

fn main() {
    // Read file
    let file = File::open("./src/input.txt").expect("Unable to open file");
    // Read file row by row without loading it entirely into memory.
    let buf_reader_lines = BufReader::new(file).lines();

    let mut list_1: Vec<u32> = Vec::new();
    // Track number frequency on this HashMap
    let mut list_2_frequency = HashMap::<u32,u32>::new();

    // For each row split the numbers.
    buf_reader_lines.for_each(|row|{
        let row_values = row.unwrap(); //get the row
        let z: Vec<&str> = row_values.split("   ").collect(); //split and collect row_value position references into this vec
        
        let list_1_value = z[0].parse::<u32>().unwrap(); // read position zero and parse to u32.
        list_1.push(list_1_value); // push to list_1.

        // This time we only care about list 2 number frequency.
        // Saving it on a HashMap. Increments when there's a number match
        let list_2_value =  z[1].parse::<u32>().unwrap(); // read position 2 and parse to u32.
        // Check the frequency tracker for pre existing frequency
        let pre_existing_list_2_frequency = match list_2_frequency.get(&list_2_value){
            Some(v) => v,
            None => &0
        };
        // push to tracker frequency + 1.
        list_2_frequency.insert(list_2_value, pre_existing_list_2_frequency + 1);
    });

    // Calculate similarity score
    let mut similarity_score: u32 = 0;

    // Iter over elements on list_1
    list_1.iter().for_each(|x| {
        // get list 2 frequency for this element
        let frequency = match list_2_frequency.get(x){
            Some(v) => v,
            None => &0
        };
        // calculate similarity score
        let this_similarity_score = x * frequency;
        // Update similarity score
        similarity_score += this_similarity_score;
    });

    println!("Similarity score is: {}", similarity_score)

    
}