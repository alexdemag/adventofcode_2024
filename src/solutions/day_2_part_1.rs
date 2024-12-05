use std::{fs::File, io::{BufRead, BufReader}};

struct Report{
    v: Vec<i32>
}

#[derive(PartialEq)]
enum ReportStatus{
    SAFE,
    UNSAFE
}

#[derive(PartialEq)]
enum ReportDirection{
    INCREASE,
    DECREASE,
    NONE
}

impl Report{
    fn safe_or_unsafe(self) -> ReportStatus{
        let mut previous_direction:ReportDirection = ReportDirection::NONE;

        for i in 0..self.v.len()-1{
            // check differences
            let current = self.v[i+1] - self.v[i];
            let abs_current = current.abs();

            // if there's a out of boundary difference discard it early
            if abs_current < 1 || abs_current > 3 || abs_current == 0{
                return ReportStatus::UNSAFE
            }
            else{
                let this_direction = match current {
                    c if c > 0 => ReportDirection::INCREASE,
                    c if c < 0 => ReportDirection::DECREASE,
                    _ => ReportDirection::NONE
                };
                
                //
                if i>0 && this_direction!=previous_direction{
                    return ReportStatus::UNSAFE
                }

                // Update values
                previous_direction = this_direction;
            }
        }
        // Well... if we run the entire thing without any return it's safe.
        ReportStatus::SAFE
    }

    fn from_report_string(s: String) -> Report{
        // Split, convert to String, cast string to u32
        let v = s.split(" ").map(|x| x.to_string()).collect::<Vec<String>>()
        .iter_mut().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        Report{v: v}
    }

}
pub fn execute(file_path: String){
    let msg_err = format!("Unable to open file: {}", file_path);
    let file = File::open(file_path).expect(&msg_err);
    let buffer = BufReader::new(file).lines();
    let mut safe_reports:u32 = 0;

    buffer.into_iter().for_each(|x| {
        let row_string = x.unwrap();
        let r = Report::from_report_string(row_string);
        let rstatus = r.safe_or_unsafe();
        
        if rstatus == ReportStatus::SAFE{
            safe_reports +=1;
        }
    });

    println!("Amount of safe reports: {}", safe_reports)
}