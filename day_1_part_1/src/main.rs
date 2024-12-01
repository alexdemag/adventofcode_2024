use std::{fs::File, io::{BufReader, Read}};

fn main() {
    // Read file
    let file = File::open("./src/input.txt").expect("Unable to open file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).expect("unable to read");

    let mut list_1: Vec<u32> = Vec::new();
    let mut list_2: Vec<u32> = Vec::new();
    let mut result: u32 = 0;
    // Split rows. For each row split numbers. Add each number to the list parsed as u32.
    contents.split("\n").for_each(|row|{
        let z:Vec<_> = row.split("   ").map(str::to_string).collect();

        list_1.push(z[0].to_string().parse::<u32>().unwrap());
        list_2.push(z[1].to_string().parse::<u32>().unwrap());
    }); 
    // Sort lists
    list_1.sort();
    list_2.sort();

    // Calculate distances
    for (a,b) in list_1.iter().zip(list_2){
        if a>=&b{
            result += a-b;
        }
        else{
            result += b-a;
        }
    }

    println!("Total distance is {}", result);
}
