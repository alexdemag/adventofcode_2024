use std::{fs::File, io::{BufRead, BufReader}};

#[derive(PartialEq, Debug)]
enum ReportStatus{
    SAFE,
    UNSAFE
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum NumberSignal{
    POSITIVE,
    NEGATIVE,
    ZERO
}

fn get_number_signal(n:i32) -> NumberSignal{
    if n<0{
        NumberSignal::NEGATIVE
    }
    else if n>0{
        NumberSignal::POSITIVE
    }
    else{
        NumberSignal::ZERO
    }
}

fn safe_or_unsafe(v:&Vec<i32>, fails:u8)->ReportStatus{
    if fails<2{
        let deltas = v.windows(2).map(|x| x[1]-x[0]).collect::<Vec<i32>>();
        let deltas_in_range = deltas.iter().map(|x| x.abs() >= 1 && x.abs()<=3).collect::<Vec<bool>>();
        let direction = deltas.windows(2).map(|x| {
                                                                            if get_number_signal(x[1]) != get_number_signal(x[0]){
                                                                                false
                                                                            }
                                                                            else{
                                                                                true
                                                                            }
                                                                        }).collect::<Vec<bool>>();

        if deltas_in_range.iter().any(|x| x==&false){
            let index = deltas_in_range.iter().position(|x| x==&false).unwrap();

            let mut v_to_go_1 = v.clone();
            v_to_go_1.remove(index);
            let mut v_to_go_2 = v.clone();
            v_to_go_2.remove(index+1);

            let status_1 = safe_or_unsafe(&v_to_go_1, fails+1);
            let status_2 = safe_or_unsafe(&v_to_go_2, fails+1);

            if status_1 == ReportStatus::SAFE || status_2 == ReportStatus::SAFE{
                return ReportStatus::SAFE
            } else{
                return ReportStatus::UNSAFE
            } 
        }
        else if direction.iter().any(|x| x==&false){
            let index = direction.iter().position(|x| x==&false).unwrap();
            
            let mut v_to_go_1 = v.clone();
            v_to_go_1.remove(index);
            let mut v_to_go_2 = v.clone();
            v_to_go_2.remove(index+1);
            let mut v_to_go_3 = v.clone();
            v_to_go_3.remove(index+2);

            let status_1 = safe_or_unsafe(&v_to_go_1, fails+1);
            let status_2 = safe_or_unsafe(&v_to_go_2, fails+1);
            let status_3 = safe_or_unsafe(&v_to_go_3, fails+1);

            if status_1 == ReportStatus::SAFE || status_2 == ReportStatus::SAFE || status_3 == ReportStatus::SAFE{
                return ReportStatus::SAFE
            } else{
                return ReportStatus::UNSAFE
            } 
        }
    }
    else {
        return ReportStatus::UNSAFE
    }
    ReportStatus::SAFE
}

pub fn execute(file_path: String){
    let msg_err = format!("Unable to open file: {}", file_path);
    let file = File::open(file_path).expect(&msg_err);
    let lines = BufReader::new(file).lines();
    let mut safe_reports:u32 = 0;

    lines.into_iter().for_each(|x| {
        let report = x.unwrap().split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let status = safe_or_unsafe(&report, 0);
        //println!("{:?},{:?}",&report,status);
        if status == ReportStatus::SAFE{
            safe_reports+=1
        }
    });

    println!("Amount of safe reports: {}", safe_reports)
}