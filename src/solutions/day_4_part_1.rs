use std::fs::read_to_string;
use rayon::prelude::*;

fn spin_check_for_xmas(v: &Vec<Vec<char>>, i_x:&usize, i_y:&usize, x_max:&i32, y_max: &i32) -> u32{
    // get the v as reference 
    // spin over position checking all angles
    
    let word = "XMAS";

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
    
    // As this evaluation runs always q1->q4, these rules suffice to guarantee the absence of overlapping counts.
    if q1{
        // right
        let found = format!("{}{}{}{}", v[*i_x][*i_y],v[*i_x][*i_y+1],v[*i_x][*i_y+2],v[*i_x][*i_y+3]);
        if found==word{
            total+=1;
        }      
        // q1 angle
        let found = format!("{}{}{}{}", v[*i_x][*i_y],v[*i_x-1][*i_y+1],v[*i_x-2][*i_y+2],v[*i_x-3][*i_y+3]);
        if found==word{
            total+=1;
        }
        // up
        let found = format!("{}{}{}{}", v[*i_x][*i_y],v[*i_x-1][*i_y],v[*i_x-2][*i_y],v[*i_x-3][*i_y]);
        if found==word{
            total+=1;
        }
    }

    if q2{
        // left
        let found = format!("{}{}{}{}", v[*i_x][*i_y],v[*i_x][*i_y-1],v[*i_x][*i_y-2],v[*i_x][*i_y-3]);
        if found==word{
            total+=1;
        }      
        // q2 angle
        let found = format!("{}{}{}{}", v[*i_x][*i_y],v[*i_x-1][*i_y-1],v[*i_x-2][*i_y-2],v[*i_x-3][*i_y-3]);
        if found==word{
            total+=1;
        }
        if !q1{ // Because if q1, we did evaluate this direction
            // up
            let found = format!("{}{}{}{}", v[*i_x][*i_y],v[*i_x-1][*i_y],v[*i_x-2][*i_y],v[*i_x-3][*i_y]);
            if found==word{
                total+=1;
            }
        }
    }
    if q3 {
        // left
        if !q2{ // Because if q2, we did evaluate this direction
            let found = format!("{}{}{}{}", v[*i_x][*i_y],v[*i_x][*i_y-1],v[*i_x][*i_y-2],v[*i_x][*i_y-3]);
            if found==word{
                total+=1;
            }
        }
        // q3 angle
        let found = format!("{}{}{}{}", v[*i_x][*i_y],v[*i_x+1][*i_y-1],v[*i_x+2][*i_y-2],v[*i_x+3][*i_y-3]);
        if found==word{
            total+=1;
        }

        // down
        let found = format!("{}{}{}{}", v[*i_x][*i_y],v[*i_x+1][*i_y],v[*i_x+2][*i_y],v[*i_x+3][*i_y]);
        if found==word{
            total+=1;
        }
    }
    if q4 {
        // down
        if !q3{ // Because if q2, we did evaluate this direction
            let found = format!("{}{}{}{}", v[*i_x][*i_y],v[*i_x+1][*i_y],v[*i_x+2][*i_y],v[*i_x+3][*i_y]);
            if found==word{
                total+=1;
            }
        }
        // q4 angle
        let found = format!("{}{}{}{}", v[*i_x][*i_y],v[*i_x+1][*i_y+1],v[*i_x+2][*i_y+2],v[*i_x+3][*i_y+3]);
        if found==word{
            total+=1;
        }
        // right
        if !q1{ // Because if q1, we did evaluate this direction
            let found = format!("{}{}{}{}", v[*i_x][*i_y],v[*i_x][*i_y+1],v[*i_x][*i_y+2],v[*i_x][*i_y+3]);
            if found==word{
                total+=1;
            }
        }
    } 

    total
    
}

pub fn execute(filepath: String){
    let matrix: Vec<Vec<char>> = read_to_string(filepath).unwrap().split("\n")
    .into_iter().map(|x| x.chars().collect()).collect::<Vec<Vec<char>>>();

    let y_max = matrix.len() as i32;
    let x_max = matrix[0].len() as i32;

    // using Rayon to run this faster.
    let res = matrix.par_iter().enumerate().map(|(i_x, x)|{
        let r = x.par_iter().enumerate().map(|(i_y, _y)| {
            spin_check_for_xmas(&matrix, &i_x, &i_y, &x_max, &y_max)
        }).collect::<Vec<u32>>();
        r
    }).collect::<Vec<_>>();

    let sum:u32 = res.into_iter().flatten().collect::<Vec<u32>>().iter().sum();

    println!("{:?}", &sum)
}