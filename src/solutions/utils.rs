use std::{fs::File, io::{BufReader,BufRead, Lines}};

pub fn load_file(filepath: String) -> Lines<BufReader<File>>{
    let buf = BufReader::new(File::open(filepath).expect("Unable to open file"));
    buf.lines()
}
