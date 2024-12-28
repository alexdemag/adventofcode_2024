use std::{fs::File, io::{BufRead, BufReader}, ops::ControlFlow};
use itertools::Itertools;
use rayon::prelude::*;


#[derive(Clone,Copy,Debug)]
enum Operation{
    ADD,
    MULTIPLY,
    CONCATENATE
}

impl Operation{
    fn solve(a: &i128, b:i128, operation: Operation) -> i128 {
        let r = match operation {
            Operation::ADD => a + b,
            Operation::MULTIPLY => a * b,
            Operation::CONCATENATE => format!("{}{}",a,b).parse::<i128>().unwrap()
        };
        r
    }
}

fn load_file(filepath: String) -> Vec<(i128,Vec<i128>)>{ 
    let buf = BufReader::new(File::open(filepath).expect("Unable to open file"));
    let lines = buf.lines();

    let calibration_equations = lines.into_iter().map(|x| {
        let str =  x.unwrap();
        let row:Vec<&str> = str.split(":").collect();
        (row[0].parse::<i128>().unwrap(), 
        row[1].trim_start().split_whitespace().map(|x| x.parse::<i128>().unwrap()).collect::<Vec<i128>>())
        
    }).collect::<Vec<(i128,Vec<i128>)>>();
    calibration_equations
}

fn solution(calibration_equations: Vec<(i128,Vec<i128>)>)->i128{
    // find operator combinations
    let calibration_equation_len = calibration_equations.iter().map(|x| 
        {x.1.len()-1}).collect::<Vec<usize>>();

    let operations = calibration_equation_len.iter().map(|x| {
        let base_operations = vec![Operation::ADD, Operation::MULTIPLY, Operation::CONCATENATE];
        
        let op = (1..=*x).map(|_| 0..=base_operations.len()-1).multi_cartesian_product().collect::<Vec<_>>()
            .iter().map(|x| x.iter().map(|i| base_operations[*i]).collect::<Vec<Operation>>()).collect::<Vec<_>>();
        op
    }).collect::<Vec<Vec<Vec<Operation>>>>();
    // a given position of calibration equations is :
    // 1 - [ result , list of numbers]
    //              2 - list of number
    //              3 - number

    let total_calibration_result = calibration_equations.par_iter().zip(operations).map(|(ce, o)| {
        let mut result_number: i128 = 0;
        let _result = o.iter().try_for_each(|op_list|{
                //let mut possible_operation_result: Vec<i128> = Vec::new();
                let mut operations_result: i128 = 0;
                let mut found_result: bool = false;

                // Saves possible operation results on outside variable, evaluates sequentially
                let _cf = ce.1.windows(2).zip(op_list.into_iter().enumerate()).try_for_each(|(number_window,(i,op))| {
                    operations_result = match i{
                        0 => Operation::solve(&number_window[0], number_window[1], *op),
                        _ => Operation::solve(&operations_result, number_window[1], *op)
                    };

                    if operations_result == ce.0 && i == ce.1.len()-2{
                        found_result = true;
                        return ControlFlow::Break("found number");
                    }
                    ControlFlow::Continue(())
                });

                // If the equation was satisfied
                // return the 
                if found_result{
                    result_number = operations_result;
                    ControlFlow::Break("found number")
                } else {
                    //result_vec.push(0);
                    ControlFlow::Continue(())
                }
            });
            result_number
        }).collect::<Vec<_>>();
        
        let final_result: i128 = total_calibration_result.iter().sum();
        //println!("{:?}", total_calibration_result);
        println!("{:?}", final_result);
        final_result
}

pub fn execute(filepath:String){
    let v = load_file(filepath);
    let _s = solution(v);

}

#[test]
fn test_solution(){
    let test: Vec<(i128,Vec<i128>)> = vec![
        (190, vec![10, 19]),
        (3267, vec![81, 40, 27]),
        (83, vec![17, 5]),
        (156, vec![15, 6]),
        (7290, vec![6, 8, 6, 15]),
        (161011, vec![16, 10, 13]),
        (192, vec![17, 8, 14]),
        (21037, vec![9, 7, 18, 13]),
        (292, vec![11, 6, 16, 20]),
    ];
    assert_eq!(solution(test), 11387);
}