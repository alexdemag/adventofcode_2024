use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}};
use rayon::prelude::*;

#[derive(Clone)]
struct Guard{
    map: Vec<Vec<char>>,
    x_limit: usize,
    y_limit: usize,
    initial_position: (usize,usize)
}

#[derive(Clone,Copy,Eq,PartialEq,Hash)]
enum GuardDirection {
    RIGHT,
    LEFT,
    UP,
    DOWN
}

impl Guard {
    pub fn get_visited_positions(&mut self) -> HashSet<(usize,usize)> {
        let mut direction: GuardDirection = GuardDirection::UP;
        let mut visited_positions:HashSet<(usize,usize)> = HashSet::new();
        let mut hit_objects: HashSet<(usize,usize,GuardDirection)> = HashSet::new();
        // Initialize next position
        let mut next_position = Guard::_next_position(&self.initial_position, &direction);

        while next_position.0.is_some_and(|x| x < self.x_limit) && next_position.1.is_some_and(|y| y < self.y_limit)
            {
                if hit_objects.contains(&(next_position.0.unwrap(), next_position.1.unwrap(), direction))
                {
                    break; // If I'm hitting the same obstacle on the same direction, stop.
                }else{
                    let char_next_pos = self.map[next_position.0.unwrap()][next_position.1.unwrap()];
                    // Special case where we find a obstacle and have to turn right
                    if char_next_pos == '#'{
                        // Check If that's the first hit. If it is, record it.
                        hit_objects.insert((next_position.0.unwrap(), next_position.1.unwrap(), direction));
                        visited_positions.insert((next_position.0.unwrap(),next_position.1.unwrap()));
                        next_position = Guard::_previous_position(&(next_position.0.unwrap(), next_position.1.unwrap()), &direction);
                        direction = Guard::_turn_right(direction);
                    }
                    else{
                        visited_positions.insert((next_position.0.unwrap(),next_position.1.unwrap()));
                        next_position = Guard::_next_position(&(next_position.0.unwrap(), next_position.1.unwrap()), &direction);
                    }
                }          
            }
            visited_positions
    }

    pub fn check_is_loop(&mut self) -> bool {
        let mut direction: GuardDirection = GuardDirection::UP;
        let mut hit_objects: HashSet<(usize,usize,GuardDirection)> = HashSet::new();
        let mut is_loop:bool = false;
        // Initialize next position
        let mut next_position = Guard::_next_position(&self.initial_position, &direction);

        while next_position.0.is_some_and(|x| x < self.x_limit) && next_position.1.is_some_and(|y| y < self.y_limit)
            {
                if hit_objects.contains(&(next_position.0.unwrap(), next_position.1.unwrap(), direction))
                {
                    is_loop = true;
                    break; // If I'm hitting the same obstacle on the same direction, stop.
                }else{
                    // Get character
                    let char_next_pos = self.map[next_position.0.unwrap()][next_position.1.unwrap()];
                    // Special case where we find a obstacle and have to turn right
                    if char_next_pos == '#'{
                        // track this hit with position + direction you were going.
                        // You may hit the same object in a different edge.
                        hit_objects.insert((next_position.0.unwrap(), next_position.1.unwrap(), direction));
                        next_position = Guard::_previous_position(&(next_position.0.unwrap(), next_position.1.unwrap()), &direction);
                        direction = Guard::_turn_right(direction);
                    }
                    else{
                        next_position = Guard::_next_position(&(next_position.0.unwrap(), next_position.1.unwrap()), &direction);
                    }
                }          
            }
            is_loop
    }
    fn _next_position(current_position: &(usize,usize), current_direction: &GuardDirection) -> (Option<usize>,Option<usize>){
        match current_direction{
            GuardDirection::UP => (current_position.0.checked_sub(1), Some(current_position.1)),
            GuardDirection::DOWN => (current_position.0.checked_add(1), Some(current_position.1)),
            GuardDirection::LEFT => (Some(current_position.0), current_position.1.checked_sub(1)),
            GuardDirection::RIGHT => (Some(current_position.0), current_position.1.checked_add(1)),
        }
    }

    fn _previous_position(current_position: &(usize,usize), current_direction: &GuardDirection) -> (Option<usize>,Option<usize>){
        match current_direction{
            GuardDirection::UP => (current_position.0.checked_add(1), Some(current_position.1)),
            GuardDirection::DOWN => (current_position.0.checked_sub(1), Some(current_position.1)),
            GuardDirection::LEFT => (Some(current_position.0), current_position.1.checked_add(1)),
            GuardDirection::RIGHT => (Some(current_position.0), current_position.1.checked_sub(1)),
        }
    }

    fn _turn_right(gr: GuardDirection)->GuardDirection{
        match gr {
            GuardDirection::UP => GuardDirection::RIGHT,
            GuardDirection::RIGHT => GuardDirection::DOWN,
            GuardDirection::DOWN => GuardDirection::LEFT,
            GuardDirection::LEFT => GuardDirection::UP
        }
    }

}

pub fn execute(filepath: String){
    let buf = BufReader::new(File::open(filepath).expect("Unable to open file"));
    let lines = buf.lines();

    let lab_map = lines.into_iter().map(|x| {
        let row = x.unwrap().chars().collect::<Vec<char>>();
        row
    }).collect::<Vec<Vec<char>>>();

    let mut x_limit:usize = 0;
    let mut y_limit:usize = 0;
    let mut initial_position: (usize, usize) = (0, 0);

    lab_map.iter().enumerate().for_each(|(i_x, x)|{
        x.iter().enumerate().for_each(|(i_y, y)| {
            if y == &'^'{
                x_limit = lab_map.len();
                y_limit = lab_map[0].len();
                initial_position = (i_x, i_y);
            }
        });
    });

    let mut guard = Guard{map: lab_map.clone(), x_limit: x_limit, y_limit: y_limit, initial_position: initial_position};

    let visited_positions = guard.get_visited_positions();

    // Brute force the crap out of this
    rayon::ThreadPoolBuilder::new().num_threads(4).build_global().unwrap();
    let which_ones_are_loops_if_replace_by_obstacle: Vec<bool> = visited_positions.par_iter().map(|x|{
        let mut new_lab_map = lab_map.clone();
        new_lab_map[x.0][x.1] = '#';

        Guard{map: new_lab_map, x_limit: x_limit, y_limit: y_limit, initial_position: initial_position}.check_is_loop()
    }).collect::<Vec<bool>>();

    let loop_count = which_ones_are_loops_if_replace_by_obstacle.iter().filter(|x| x==&&true).count();
    println!("{}",loop_count)
}
