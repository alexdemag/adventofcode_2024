use std::{collections::{HashSet}, fs::File, io::{BufRead, BufReader}};

#[derive(Clone)]
struct Guard{
    map: Vec<Vec<char>>,
    x_limit: usize,
    y_limit: usize,
    initial_position: (usize,usize)
}

#[derive(Clone,Copy,Eq,PartialEq)]
enum GuardDirection {
    RIGHT,
    LEFT,
    UP,
    DOWN
}

impl Guard {
    pub fn get_total_positions(&mut self) -> u32{
        let mut direction:GuardDirection = GuardDirection::UP;
        let mut visited_positions: HashSet<(usize,usize)> = HashSet::new();

        let mut next_position = Guard::_next_position(&self.initial_position, &direction);

        while next_position.0.is_some_and(|x| x < self.x_limit) && next_position.1.is_some_and(|y| y < self.y_limit)
            {
                let next_x = next_position.0.unwrap();
                let next_y = next_position.1.unwrap();

                if next_x == self.initial_position.0 && next_y == self.initial_position.1 && direction == GuardDirection::UP
                {
                    break; // If I'm at the starting position going the same way as before, I'm looping. Stop.
                }else{
                    let char_next_pos = self.map[next_x][next_y];
                    // Special case where we find a obstacle and have to turn right
                    if char_next_pos == '#'{
                        // If this is an obstacle, walk back and turn right.
                        next_position = Guard::_previous_position(&(next_x, next_y), &direction);
                        direction = Guard::_turn_right(direction);
                    }
                    else{
                        visited_positions.insert((next_x,next_y));
                        next_position = Guard::_next_position(&(next_x, next_y), &direction);
                    }
                }          
            }
            visited_positions.len() as u32
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

    let mut guards: Vec<Guard> = Vec::new();

    lab_map.iter().enumerate().for_each(|(i_x, x)|{
        x.iter().enumerate().for_each(|(i_y, y)| {
            if y == &'^'{ // Search for "^" initial position. if found push it to the guards Vec
                let x_limit = lab_map.len();
                let y_limit = lab_map[0].len();
                let initial_position = (i_x, i_y);
                guards.push(Guard{map: lab_map.clone(), x_limit: x_limit, y_limit: y_limit, initial_position: initial_position})
            }
        });
    });

    let res = guards.iter_mut().map( |x|{
        x.get_total_positions()
    }).collect::<Vec<u32>>();
    
    println!("{}",res[0])


}
