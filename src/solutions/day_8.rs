use std::{collections::HashSet, hash::Hash};
use itertools::Itertools;
use num_traits::abs;

use super::utils::load_file;

struct Tower{
    x: usize,
    y: usize,
    freq: char
}
#[derive(PartialEq,Eq,Hash)]
struct Node{
    x: usize,
    y: usize
}

#[derive(PartialEq)]
enum Slope{
    HORIZONTAL,
    VERTICAL,
    UPWARDS,
    DOWNARDS,
    INVALID
}

fn find_slope_and_towers_in_order<'a>(tower_1: &'a Tower, tower_2: &'a Tower) -> (Slope, &'a Tower, &'a Tower){
    let mut t1 = tower_1;
    let mut t2 = tower_2;
    let mut slope = Slope::INVALID;
    // My reference is that the left size y is always lower than the right side.
    // If the towers come swapped I'll swap them back
    if tower_1.y > tower_2.y{
        t1 = tower_2;
        t2 = tower_1; 
    }

    // Get slope
    if t1.x > t2.x && t1.y < t2.y{
        slope = Slope::UPWARDS
    }else if t1.y == t2.y {
        slope = Slope::VERTICAL
    }else if t1.x == t2.x {
        slope = Slope::HORIZONTAL
    }else if t1.x < t2.x && t1.y < t2.y {
        slope = Slope::DOWNARDS
    }
    // Return slope with towers in the correct order
    (slope, t1, t2)
}

fn find_antinodes(t1: &Tower, t2: &Tower, x_limit: usize, y_limit: usize, slope: Slope) -> HashSet<Node>{
    // This function demands t2.y > t1.y 
    let mut antinodes: HashSet<Node> = HashSet::new();
    
    if slope == Slope::HORIZONTAL{
        let delta = abs(t2.y as i64 - t1.y as i64) as usize;

        let antinode_1_y = t1.y.checked_sub(delta);
        if antinode_1_y.is_some_and(|x| x < y_limit){
            let _ = antinodes.insert(Node{x: t1.x, y: antinode_1_y.unwrap()});
        }

        let antinode_2_y = t2.y.checked_add(delta);
        if antinode_1_y.is_some_and(|x| x < y_limit){
            let _ = antinodes.insert(Node{x: t1.x, y: antinode_2_y.unwrap()});
        }
    } else if slope == Slope::VERTICAL{
        let delta = abs(t2.x as i64 - t1.x as i64) as usize;

        let antinode_1_x = t1.x.checked_sub(delta);
        if antinode_1_x.is_some_and(|x| x<x_limit){
            let _ = antinodes.insert(Node{x: antinode_1_x.unwrap(), y: t1.y});
        }

        let antinode_2_x = t2.x.checked_add(delta);
        if antinode_2_x.is_some_and(|x| x<x_limit){
            let _ = antinodes.insert(Node{x: antinode_2_x.unwrap(), y: t1.y});
        }
    } else if slope == Slope::DOWNARDS{
        let x_delta = abs(t2.x as i64 - t1.x as i64) as usize;
        let y_delta = abs(t2.y as i64 - t1.y as i64) as usize;
        
        let antinode_1_x = t1.x.checked_sub(x_delta);
        let antinode_1_y = t1.y.checked_sub(y_delta);
        if antinode_1_x.is_some_and(|x| x<x_limit) && antinode_1_y.is_some_and(|y| y < y_limit){
            let _ = antinodes.insert(Node{x: antinode_1_x.unwrap(), y: antinode_1_y.unwrap()});
        }

        let antinode_2_x = t2.x.checked_add(x_delta);
        let antinode_2_y = t2.y.checked_add(y_delta);
        if antinode_2_x.is_some_and(|x| x<x_limit) && antinode_2_y.is_some_and(|y| y < y_limit){
            let _ = antinodes.insert(Node{x: antinode_2_x.unwrap(), y: antinode_2_y.unwrap()});
        }
    } else if slope == Slope::UPWARDS{
        let x_delta = abs(t2.x as i64 - t1.x as i64) as usize;
        let y_delta = abs(t2.y as i64 - t1.y as i64) as usize;

        let antinode_1_x = t1.x.checked_add(x_delta);
        let antinode_1_y = t1.y.checked_sub(y_delta);
        if antinode_1_x.is_some_and(|x| x<x_limit) && antinode_1_y.is_some_and(|y| y < y_limit){
            let _ = antinodes.insert(Node{x: antinode_1_x.unwrap(), y: antinode_1_y.unwrap()});
        }

        let antinode_2_x = t2.x.checked_sub(x_delta);
        let antinode_2_y = t2.y.checked_add(y_delta);
        if antinode_2_x.is_some_and(|x| x<x_limit) && antinode_2_y.is_some_and(|y| y < y_limit){
            let _ = antinodes.insert(Node{x: antinode_2_x.unwrap(), y: antinode_2_y.unwrap()});
        }
    }
    antinodes
}

fn find_antinodes_part_2(t1: &Tower, t2: &Tower, x_limit: usize, y_limit: usize, slope: Slope) -> HashSet<Node>{
    // This function demands t2.y > t1.y 
    let mut antinodes: HashSet<Node> = HashSet::new();

    // Adding towers themselves as antinodes
    let _ = antinodes.insert(Node{x: t1.x, y: t1.y});
    let _ = antinodes.insert(Node{x: t2.x, y: t2.y});
    
    if slope == Slope::HORIZONTAL{
        let delta = abs(t2.y as i64 - t1.y as i64) as usize;

        let mut antinode_1_y = t1.y.checked_sub(delta);
        while antinode_1_y.is_some_and(|x| x < y_limit){
            let _ = antinodes.insert(Node{x: t1.x, y: antinode_1_y.unwrap()});
            antinode_1_y = antinode_1_y.unwrap().checked_sub(delta);
        }

        let mut antinode_2_y = t2.y.checked_add(delta);
        while antinode_1_y.is_some_and(|x| x < y_limit){
            let _ = antinodes.insert(Node{x: t1.x, y: antinode_2_y.unwrap()});
            antinode_2_y = antinode_2_y.unwrap().checked_add(delta);
        }
    } else if slope == Slope::VERTICAL{
        let delta = abs(t2.x as i64 - t1.x as i64) as usize;

        let mut antinode_1_x = t1.x.checked_sub(delta);
        while antinode_1_x.is_some_and(|x| x<x_limit){
            let _ = antinodes.insert(Node{x: antinode_1_x.unwrap(), y: t1.y});
            antinode_1_x = antinode_1_x.unwrap().checked_sub(delta);
        }

        let mut antinode_2_x = t2.x.checked_add(delta);
        while antinode_2_x.is_some_and(|x| x<x_limit){
            let _ = antinodes.insert(Node{x: antinode_2_x.unwrap(), y: t1.y});
            antinode_2_x = antinode_2_x.unwrap().checked_add(delta);
        }
    } else if slope == Slope::DOWNARDS{
        let x_delta = abs(t2.x as i64 - t1.x as i64) as usize;
        let y_delta = abs(t2.y as i64 - t1.y as i64) as usize;
        
        let mut antinode_1_x = t1.x.checked_sub(x_delta);
        let mut antinode_1_y = t1.y.checked_sub(y_delta);
        while antinode_1_x.is_some_and(|x| x<x_limit) && antinode_1_y.is_some_and(|y| y < y_limit){
            let _ = antinodes.insert(Node{x: antinode_1_x.unwrap(), y: antinode_1_y.unwrap()});
            antinode_1_x = antinode_1_x.unwrap().checked_sub(x_delta);
            antinode_1_y = antinode_1_y.unwrap().checked_sub(y_delta);
        }

        let mut antinode_2_x = t2.x.checked_add(x_delta);
        let mut antinode_2_y = t2.y.checked_add(y_delta);
        while antinode_2_x.is_some_and(|x| x<x_limit) && antinode_2_y.is_some_and(|y| y < y_limit){
            let _ = antinodes.insert(Node{x: antinode_2_x.unwrap(), y: antinode_2_y.unwrap()});
            antinode_2_x = antinode_2_x.unwrap().checked_add(x_delta);
            antinode_2_y = antinode_2_y.unwrap().checked_add(y_delta);
        }
    } else if slope == Slope::UPWARDS{
        let x_delta = abs(t2.x as i64 - t1.x as i64) as usize;
        let y_delta = abs(t2.y as i64 - t1.y as i64) as usize;

        let mut antinode_1_x = t1.x.checked_add(x_delta);
        let mut antinode_1_y = t1.y.checked_sub(y_delta);
        while antinode_1_x.is_some_and(|x| x<x_limit) && antinode_1_y.is_some_and(|y| y < y_limit){
            let _ = antinodes.insert(Node{x: antinode_1_x.clone().unwrap(), y: antinode_1_y.clone().unwrap()});
            antinode_1_x = antinode_1_x.unwrap().checked_add(x_delta);
            antinode_1_y = antinode_1_y.unwrap().checked_sub(y_delta);
        }

        let mut antinode_2_x = t2.x.checked_sub(x_delta);
        let mut antinode_2_y = t2.y.checked_add(y_delta);
        while antinode_2_x.is_some_and(|x| x<x_limit) && antinode_2_y.is_some_and(|y| y < y_limit){
            let _ = antinodes.insert(Node{x: antinode_2_x.unwrap().clone(), y: antinode_2_y.unwrap().clone()});
            antinode_2_x = antinode_2_x.unwrap().checked_sub(x_delta);
            antinode_2_y = antinode_2_y.unwrap().checked_add(y_delta);
        }
    }
    antinodes
}

fn part_1(input: Vec<Vec<char>>) -> usize{
    // I'll let it panic if inputs are zero length. It shouldn't run anyways.
    let x_limit = input.len();
    let y_limit = input[0].len();

    let mut towers: Vec<Tower> = Vec::new();
    let mut distinct_towers: HashSet<char> = HashSet::new();

    // Run over map, get towers
    input.into_iter().enumerate().for_each(|(x,row)| {
        row.into_iter().enumerate().for_each(|(y, character)| {
            if character != '.'{
                distinct_towers.insert(character.clone());
                towers.push(Tower{x: x, y: y, freq: character});
            }
        })
    });

    let mut tower_combinations:Vec<Vec<&Tower>> = Vec::new();

    // Build tower combinations for each tower
    distinct_towers.iter().for_each(|freq|{
        let tower_comb = towers.iter().filter(|x| x.freq == *freq).combinations(2).collect::<Vec<_>>();
        tower_combinations.extend(tower_comb);
    });
    
    // Get antinodes
   let antinodes = tower_combinations.iter().map(|tower_combination| {
        let this_antinodes = tower_combination.windows(2).map(|t|{
            let (s, t1, t2) = find_slope_and_towers_in_order(t[0], t[1]);
            let antinodes = find_antinodes(t1, t2, x_limit, y_limit, s);
            antinodes
        }).collect::<Vec<_>>();
        this_antinodes
    }).collect::<Vec<_>>();

    // Count antinodes
    let antinode_count = antinodes.iter().flatten().flatten().unique().collect::<Vec<_>>().iter().count();
    antinode_count
}

fn part_2(input: Vec<Vec<char>>) -> usize{
    // I'll let it panic if inputs are zero length. It shouldn't run anyways.
    let x_limit = input.len();
    let y_limit = input[0].len();

    let mut towers: Vec<Tower> = Vec::new();
    let mut distinct_towers: HashSet<char> = HashSet::new();

    // Run over map, get towers
    input.into_iter().enumerate().for_each(|(x,row)| {
        row.into_iter().enumerate().for_each(|(y, character)| {
            if character != '.'{
                distinct_towers.insert(character.clone());
                towers.push(Tower{x: x, y: y, freq: character});
            }
        })
    });

    let mut tower_combinations:Vec<Vec<&Tower>> = Vec::new();

    // Build tower combinations for each tower
    distinct_towers.iter().for_each(|freq|{
        let tower_comb = towers.iter().filter(|x| x.freq == *freq).combinations(2).collect::<Vec<_>>();
        tower_combinations.extend(tower_comb);
    });
    
    // Get antinodes
    let antinodes = tower_combinations.iter().map(|tower_combination| {
        let this_antinodes = tower_combination.windows(2).map(|t|{
            let (s, t1, t2) = find_slope_and_towers_in_order(t[0], t[1]);
            let antinodes = find_antinodes_part_2(t1, t2, x_limit, y_limit, s);
            antinodes
        }).collect::<Vec<_>>();
        this_antinodes
    }).collect::<Vec<_>>();

    // Count antinodes
    let antinode_count = antinodes.iter().flatten().flatten().unique().collect::<Vec<_>>().iter().count();
    antinode_count
}

pub fn execute(filepath: String, part:u8){
    let lines = load_file(filepath);
    let input = lines.into_iter().map(|x| {
        let s = x.unwrap().chars().collect::<Vec<_>>();
        s
    }).collect::<Vec<_>>();

    if part == 1{
        let result = part_1(input);
        println!("Antinode Count: {}", result);
    } else if part == 2{
        let result = part_2(input);
        println!("Antinode Count: {}", result);   
    }
    else{
        println!("day not found");
    }
    
}

#[test]
fn test_part_1(){
    let input: Vec<Vec<char>> = vec![vec![".",".",".",".",".",".",".",".",".",".",".","."],
    vec![".",".",".",".",".",".",".",".","0",".",".","."],
    vec![".",".",".",".",".","0",".",".",".",".",".","."],
    vec![".",".",".",".",".",".",".","0",".",".",".","."],
    vec![".",".",".",".","0",".",".",".",".",".",".","."],
    vec![".",".",".",".",".",".","A",".",".",".",".","."],
    vec![".",".",".",".",".",".",".",".",".",".",".","."],
    vec![".",".",".",".",".",".",".",".",".",".",".","."],
    vec![".",".",".",".",".",".",".",".","A",".",".","."],
    vec![".",".",".",".",".",".",".",".",".","A",".","."],
    vec![".",".",".",".",".",".",".",".",".",".",".","."],
    vec![".",".",".",".",".",".",".",".",".",".",".","."]].iter().map(
                |row| {
                    row.iter().map(|chr| {
                        chr.parse::<char>().unwrap()
                    }).collect::<Vec<_>>()
                }).collect::<Vec<Vec<_>>>();
    let r = part_1(input);
    assert_eq!(r, 14)
}

#[test]
fn test_part_2(){
    let input: Vec<Vec<char>> = vec![vec![".",".",".",".",".",".",".",".",".",".",".","."],
    vec![".",".",".",".",".",".",".",".","0",".",".","."],
    vec![".",".",".",".",".","0",".",".",".",".",".","."],
    vec![".",".",".",".",".",".",".","0",".",".",".","."],
    vec![".",".",".",".","0",".",".",".",".",".",".","."],
    vec![".",".",".",".",".",".","A",".",".",".",".","."],
    vec![".",".",".",".",".",".",".",".",".",".",".","."],
    vec![".",".",".",".",".",".",".",".",".",".",".","."],
    vec![".",".",".",".",".",".",".",".","A",".",".","."],
    vec![".",".",".",".",".",".",".",".",".","A",".","."],
    vec![".",".",".",".",".",".",".",".",".",".",".","."],
    vec![".",".",".",".",".",".",".",".",".",".",".","."]].iter().map(
                |row| {
                    row.iter().map(|chr| {
                        chr.parse::<char>().unwrap()
                    }).collect::<Vec<_>>()
                }).collect::<Vec<Vec<_>>>();
    let r = part_2(input);
    assert_eq!(r, 34)
}