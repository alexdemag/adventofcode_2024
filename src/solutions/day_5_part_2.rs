use std::{fs::File, io::{BufRead, BufReader}};

#[derive(Debug)]
struct Rule{
    left: u32,
    right: u32,
}

fn check_rules_against_manual_and_fix(manual: &mut Vec<u32>, rules: &Vec<Rule>) -> Option<u32>{
    let mut needs_check = true;
    let mut sort_was_needed = false;
    let mut had_a_sort_on_this_run = false;

    while needs_check{
        rules.iter().for_each(|r| {
            let left_index = manual.iter().position(|x| x == &r.left);
            let right_index = manual.iter().position(|x| x == &r.right);

            // Fail if both have values but left value index is higher than right value index
            if left_index.is_some() && right_index.is_some() && left_index.unwrap() > right_index.unwrap(){
                // Flag that we did needed to swap values.
                // Important so we can rerun the check from the beginning.
                sort_was_needed = true;
                had_a_sort_on_this_run = true;
                // That's sort of a bubble sort.
                manual.swap(left_index.unwrap(), right_index.unwrap());
            }
            () 
        });
        if sort_was_needed && had_a_sort_on_this_run{
            needs_check = true;
            had_a_sort_on_this_run = false;
        }
        else{
            needs_check = false;
        }
    }
    // Return the Option<u32> with mid value in case this was a fixed one.
    if sort_was_needed{
        Some(manual.get((manual.len()-1)/2).unwrap().clone())
    }
    else{
        None
    }
}

pub fn execute(filepath: String){
    let buf = BufReader::new(File::open(filepath).expect("Unable to open file"));
    let lines = buf.lines();
    
    let mut rules: Vec<Rule> = Vec::new();
    let mut manual_pages: Vec<Vec<u32>> = Vec::new();

    // parse the rules and manual pages
    lines.for_each(|x|{
        let line = x.unwrap();
        if line.contains("|"){
            let rules_raw = line.split("|").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
            let r = Rule{left: rules_raw[0], right: rules_raw[1]};
            rules.push(r);
        }
        else if line.contains(","){
            let manual_page = line.split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
            manual_pages.push(manual_page);
        }
    });

    let mut mid_number_sum: u32 = 0;

    // I'll loop the manual rows one by one 
    // and apply rules + return and sum the middle number
    // whenever it exists
    // It could've been a map + multithread but I didn't feel like it on this one
    manual_pages.iter_mut().for_each(|x|{
        match check_rules_against_manual_and_fix(x, &rules){
            Some(x) => mid_number_sum += x,
            None => ()
        }
    });

    println!("{}", mid_number_sum);
}