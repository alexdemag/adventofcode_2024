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
    
    // i.e cargo run -- day_2_part_1 day_2
    //                  args: challenge file
    // files are set to be on ./src/data/{file_name}.txt
    // no need to add .txt suffix when calling this CLI.

    match challenge{
        "day_1_part_1" => solutions::day_1_part_1::execute(filepath),
        "day_1_part_2" => solutions::day_1_part_2::execute(filepath),
        "day_2_part_1" => solutions::day_2_part_1::execute(filepath),
        "day_2_part_2" => solutions::day_2_part_2::execute(filepath),
        "day_3_part_1" => solutions::day_3_part_1::execute(filepath),
        "day_3_part_2" => solutions::day_3_part_2::execute(filepath),
        "day_4_part_1" => solutions::day_4_part_1::execute(filepath),
        "day_4_part_2" => solutions::day_4_part_2::execute(filepath),
        "day_5_part_1" => solutions::day_5_part_1::execute(filepath),
        "day_5_part_2" => solutions::day_5_part_2::execute(filepath),
        "day_6_part_1" => solutions::day_6_part_1::execute(filepath),
        "day_6_part_2" => solutions::day_6_part_2::execute(filepath),
        "day_7_part_1" => solutions::day_7_part_1::execute(filepath),
        "day_7_part_2" => solutions::day_7_part_2::execute(filepath),
        "day_8_part_1" => solutions::day_8::execute(filepath, 1),
        "day_8_part_2" => solutions::day_8::execute(filepath, 2),
        "day_9_part_1" => solutions::day_9::execute(filepath, 1),
        "day_9_part_2" => solutions::day_9::execute(filepath, 2),
        "day_10_part_1" => solutions::day_10::execute(filepath, 1),
        "day_10_part_2" => solutions::day_10::execute(filepath, 2),
        _ => println!{"No challenge found for {} input", challenge}
    }   
}
