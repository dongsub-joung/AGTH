use std::io::{stdin, BufRead};


pub fn init_inputing() -> String{
    let std= stdin();
    let mut buff= std.lock().lines();
    let input_string= buff.next().unwrap().unwrap();

    input_string
}