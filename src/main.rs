mod solutions;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let challenge = match args.get(1){
        Some(x) => x.as_str(),
        None => ""
    };
    let filename = match args.get(2){
        Some(x) => x.as_str(),
        None => ""
    };
    
    let filepath = format!("./src/data/{}.txt", filename);
    
    // cargo run -- day_2_part_1 day_2_part_1_small

    match challenge{
        "day_1_part_1" => solutions::day_1_part_1::execute(filepath),
        "day_1_part_2" => solutions::day_1_part_2::execute(filepath),
        "day_2_part_1" => solutions::day_2_part_1::execute(filepath),
        "day_2_part_2" => solutions::day_2_part_2::execute(filepath),
        _ => println!{"No challenge found for {} input", challenge}
    }   
}
