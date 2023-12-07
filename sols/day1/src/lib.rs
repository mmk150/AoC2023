use std::env;
use std::error::Error;
use std::fs;

const path: &str = "../../inputs/day";

fn read(day_num: i32) -> String {
    let final_path: &str = &(path.to_owned() + &day_num.to_string() + ".txt");
    println!("{}", final_path);
    let content = fs::read_to_string(final_path).expect("Should have been able to read the file");
    return content;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_num: i32 = args[1].parse::<i32>().unwrap();
    read(day_num);
}
