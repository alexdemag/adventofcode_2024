use std::{fs::File, io::{BufReader, Read}};
use std::collections::HashMap;

fn main() {
    // Read file
    let file = File::open("./src/input.txt").expect("Unable to open file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).expect("unable to read");

    let mut list_1: Vec<u32> = Vec::new();
    let mut list_2_frequency = HashMap::<u32,u32>::new();

    // Split rows. For each row split numbers. Add each number to the list parsed as u32.
    contents.split("\n").for_each(|row|{
        let z:Vec<_> = row.split("   ").map(str::to_string).collect();
        
        let list_1_value = z[0].to_string().parse::<u32>().unwrap();
        list_1.push(list_1_value);

        // This time we only care about list 2 number frequency.
        // Saving it on a HashMap. Increments when there's a number match
        let list_2_value =  z[1].to_string().parse::<u32>().unwrap();
        let pre_existing_list_2_frequency = match list_2_frequency.get(&list_2_value){
            Some(v) => v,
            None => &0
        };
        list_2_frequency.insert(list_2_value, pre_existing_list_2_frequency + 1);
    });

    // Calculate similarity score
    let mut similarity_score: u32 = 0;

    // Iter over elements on list_1
    list_1.iter().for_each(|x| {
        // get list 2 frequency for this element
        let l2f = match list_2_frequency.get(x){
            Some(v) => v,
            None => &0
        };
        // calculate similarity score
        let this_similarity_score = x*l2f;
        similarity_score += this_similarity_score;
    });

    println!("Similarity score is: {}", similarity_score)

    
}