use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    // Read file
    let file = File::open("./src/input.txt").expect("Unable to open file");
    // Read file row by row without loading it entirely into memory.
    let buf_reader_lines = BufReader::new(file).lines();

    let mut list_1: Vec<u32> = Vec::new();
    let mut list_2: Vec<u32> = Vec::new();
    let mut result: u32 = 0;
    // Split rows. For each row split numbers. Add each number to the list parsed as u32.
    buf_reader_lines.for_each(|row|{
        let row_values = row.unwrap(); //get the row
        let z:Vec<_> = row_values.split("   ").map(str::to_string).collect();

        list_1.push(z[0].to_string().parse::<u32>().unwrap());
        list_2.push(z[1].to_string().parse::<u32>().unwrap());
    }); 
    // Sort lists
    list_1.sort();
    list_2.sort();

    // Calculate distances
    for (a,b) in list_1.iter().zip(list_2){
        // This is a bit ugly but does the job without i32
        // The problem does not state that numbers can be negative.
        // Is that's the case this program wont run anyways
        // Because we parse numbers as u32.
        if a>=&b{
            result += a-b;
        }
        else{
            result += b-a;
        }
    }

    println!("Total distance is {}", result);
}
