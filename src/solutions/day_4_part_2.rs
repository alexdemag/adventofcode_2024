use std::{collections::HashMap, fs::read_to_string, sync::{Arc, Mutex}};
use rayon::prelude::*;

#[derive(Eq, PartialEq,Hash, Clone)]
struct XmasCenter{
    x: usize,
    y: usize
}

// STRATEGY -> Track the center of each MAS. If two MAS have the same center we have a mas "X".
fn spin_check_for_xmas(v: &Vec<Vec<char>>,xmas_center: Arc<Mutex<Vec<XmasCenter>>>,i_x:&usize, i_y:&usize, x_max:&i32, y_max: &i32) -> u32{
    // get the v as reference
    // spin over the given position checking all possibilities on each quarter around it
    let word = "MAS";

    let mut word_size: i32 = word.len().try_into().unwrap();
    word_size = word_size - 1;

    let mut total:u32=0;

    let i_x_check = *i_x as i32;
    let i_y_check = *i_y as i32;

    // Check which quartes to run
    let q1 = i_x_check - word_size >= 0 && i_y_check + word_size <= *y_max - 1;
    let q2 = i_x_check - word_size >= 0 && i_y_check - word_size >= 0;
    let q3 = i_x_check + word_size <= *x_max -1 && i_y_check - word_size >= 0;
    let q4 = i_x_check + word_size <= *x_max -1 && i_y_check + word_size <= *y_max - 1;
    
    // Evaluate only angled words.
    if q1{ 
        // q1 angle
        let found = format!("{}{}{}", v[*i_x][*i_y],v[*i_x-1][*i_y+1],v[*i_x-2][*i_y+2]); // yes this could be dynamic.. I know I know...
        if found==word{
            total+=1;
            xmas_center.lock().unwrap().push(XmasCenter{x: i_x-1, y: i_y+1});
        }
    }
    if q2{ 
        // q2 angle
        let found = format!("{}{}{}", v[*i_x][*i_y],v[*i_x-1][*i_y-1],v[*i_x-2][*i_y-2]);
        if found==word{
            total+=1;
            xmas_center.lock().unwrap().push(XmasCenter{x: i_x-1, y: i_y-1});
        }
    }
    if q3 {
        // q3 angle
        let found = format!("{}{}{}", v[*i_x][*i_y],v[*i_x+1][*i_y-1],v[*i_x+2][*i_y-2]);
        if found==word{
            total+=1;
            xmas_center.lock().unwrap().push(XmasCenter{x: i_x+1, y: i_y-1});
        }
    }
    if q4 {
        // q4 angle
        let found = format!("{}{}{}", v[*i_x][*i_y],v[*i_x+1][*i_y+1],v[*i_x+2][*i_y+2]);
        if found==word{
            total+=1;
            xmas_center.lock().unwrap().push(XmasCenter{x: i_x+1, y: i_y+1});
        }
    } 
    total
}

pub fn execute(filepath: String){
    let matrix: Vec<Vec<char>> = read_to_string(filepath).unwrap().split("\n")
    .into_iter().map(|x| x.chars().collect()).collect::<Vec<Vec<char>>>();

    let y_max = matrix.len() as i32;
    let x_max = matrix[0].len() as i32;

    // Arc<Mutex>> due to multithreading.
    let xmas_center: Arc<Mutex<Vec<XmasCenter>>> = Arc::new(Mutex::new(Vec::new()));

    // using Rayon to run this faster.
    let _ = matrix.par_iter().enumerate().map(|(i_x, x)|{
        let r = x.par_iter().enumerate().map(|(i_y, _y)| {
            spin_check_for_xmas(&matrix, xmas_center.clone(), &i_x, &i_y, &x_max, &y_max)
        }).collect::<Vec<u32>>();
        r
    }).collect::<Vec<_>>();

    let mut hm: HashMap<XmasCenter, u16> = HashMap::new();

    for n in xmas_center.lock().unwrap().iter() {
        hm.entry(n.clone()).and_modify(|count| *count += 1).or_insert(1);
    }

    let sum = hm.iter().filter(|x| x.1 > &1).count();

    println!("{:?}", &sum)
}