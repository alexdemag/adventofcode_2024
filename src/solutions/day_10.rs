use std::collections::{HashMap, HashSet};

use super::utils::load_file;

fn find_trailheads(map: &Vec<Vec<u32>>) -> Vec<(usize, usize)>{
    let mut trailheads: Vec<(usize,usize)> = Vec::new();
    map.iter().enumerate().for_each(|row| {
        row.1.iter().enumerate().for_each(|column| {
            if column.1 == &(0 as u32){
                trailheads.push((row.0, column.0));
            }
        });
    });
    trailheads
}

fn is_valid_coord(coord: (usize,usize), max_row:&usize, max_col:&usize)->bool{
    if &coord.0 < max_row && &coord.1 < max_col{
        true
    } else{
        false
    }
}

fn valid_coordinates(coord: (usize,usize), max_row:&usize, max_col:&usize) -> Vec<(usize,usize)>{
    let mut valid_coordinates: Vec<(usize,usize)> = Vec::new();
    let right = (coord.0, coord.1+1);
    if is_valid_coord(right, max_row, max_col){
        valid_coordinates.push(right)
    }
    let left = (coord.0, coord.1.checked_sub(1));
    if left.1.is_some(){
        valid_coordinates.push((left.0, left.1.unwrap()))
    }
    let up = (coord.0.checked_sub(1), coord.1);
    if up.0.is_some(){
        valid_coordinates.push((up.0.unwrap(), up.1));
    }
    let down = (coord.0+1, coord.1);
    if is_valid_coord(down, max_row, max_col){
        valid_coordinates.push(down)
    }
    valid_coordinates
}

// Part 1
fn look_around(map: &Vec<Vec<u32>>, head: (usize, usize), next: (usize, usize),max_row: &usize, max_col: &usize, traihead_found_nines: &mut HashSet<(usize,usize)>) {
    if map[next.0][next.1] == 9 {
        let _ = traihead_found_nines.insert(next.clone());
    }
    else {
        let valid_coordinates = valid_coordinates(next, &max_row, &max_col);
        let next_steps = valid_coordinates.iter().filter(|x| map[x.0][x.1] == map[next.0][next.1]+1).collect::<Vec<_>>();
        next_steps.iter().for_each(|x| look_around(map, head, **x, max_row, max_col, traihead_found_nines));
    }
}

fn check_trail(map: &Vec<Vec<u32>>, head: (usize, usize), traihead_scores: &mut HashMap<(usize,usize), u128>) {
    let max_row = map.len();
    let max_col = map[0].len();
    let mut traihead_found_nines: HashSet<(usize,usize)> = HashSet::new();
    look_around(map, head, head, &max_row, &max_col, &mut traihead_found_nines);
    let scores = traihead_found_nines.len() as u128;
    let _ = traihead_scores.insert(head, scores.clone());
}

// Part2
fn check_trail_all_possibilities(map: &Vec<Vec<u32>>, head: (usize, usize), traihead_scores: &mut HashMap<(usize,usize), u128>) {
    let max_row = map.len();
    let max_col = map[0].len();
    let mut traihead_found_nines: Vec<(usize,usize)> = Vec::new();
    look_around_all_possibilities(map, head, head, &max_row, &max_col, &mut traihead_found_nines);
    let scores = traihead_found_nines.len() as u128;
    let _ = traihead_scores.insert(head, scores.clone());
}

fn look_around_all_possibilities(map: &Vec<Vec<u32>>, head: (usize, usize), next: (usize, usize),max_row: &usize, max_col: &usize, traihead_found_nines: &mut Vec<(usize,usize)>) {
    if map[next.0][next.1] == 9 {
        let _ = traihead_found_nines.push(next.clone());
    }
    else {
        let valid_coordinates = valid_coordinates(next, &max_row, &max_col);
        let next_steps = valid_coordinates.iter().filter(|x| map[x.0][x.1] == map[next.0][next.1]+1).collect::<Vec<_>>();
        next_steps.iter().for_each(|x| look_around_all_possibilities(map, head, **x, max_row, max_col, traihead_found_nines));
    }
}

fn part_1(map:Vec<Vec<u32>>) -> u128{
    let trailheads: Vec<(usize, usize)> = find_trailheads(&map);
    let mut traihead_scores:HashMap<(usize,usize), u128> = HashMap::new();
    trailheads.iter().for_each(|tr| check_trail(&map, *tr, &mut traihead_scores));
    let final_score = traihead_scores.values().sum();
    print!("scores: {}",final_score);
    final_score
}

fn part_2(map:Vec<Vec<u32>>) -> u128{
    let trailheads: Vec<(usize, usize)> = find_trailheads(&map);
    let mut traihead_scores:HashMap<(usize,usize), u128> = HashMap::new();
    trailheads.iter().for_each(|tr| check_trail_all_possibilities(&map, *tr, &mut traihead_scores));
    let final_score = traihead_scores.values().sum();
    print!("scores: {}",final_score);
    final_score
}

pub fn execute(filepath:String,part:u8){
    let lines = load_file(filepath);

    let parsed = lines.into_iter().map(|x| {
        x.unwrap().chars().map(|y| {
            y.to_digit(10).unwrap()}).collect::<Vec<_>>()
        }).collect::<Vec<_>>();
    
    let res = match part{
        1 => part_1(parsed),
        2 => part_2(parsed),
        _ => 0
    };

    }

#[test]
fn test_part_1(){
    let input: Vec<Vec<u32>> = vec![
                                    vec![8,9,0,1,0,1,2,3],
                                    vec![7,8,1,2,1,8,7,4],
                                    vec![8,7,4,3,0,9,6,5],
                                    vec![9,6,5,4,9,8,7,4],
                                    vec![4,5,6,7,8,9,0,3],
                                    vec![3,2,0,1,9,0,1,2],
                                    vec![0,1,3,2,9,8,0,1],
                                    vec![1,0,4,5,6,7,3,2]
                                ];
    let res = part_1(input);
    assert_eq!(res, 36)
}
 #[test]
 fn test_part_2(){
    let input: Vec<Vec<u32>> = vec![
                                    vec![8,9,0,1,0,1,2,3],
                                    vec![7,8,1,2,1,8,7,4],
                                    vec![8,7,4,3,0,9,6,5],
                                    vec![9,6,5,4,9,8,7,4],
                                    vec![4,5,6,7,8,9,0,3],
                                    vec![3,2,0,1,9,0,1,2],
                                    vec![0,1,3,2,9,8,0,1],
                                    vec![1,0,4,5,6,7,3,2]
                                ];
    let res = part_2(input);
    assert_eq!(res, 81)
}