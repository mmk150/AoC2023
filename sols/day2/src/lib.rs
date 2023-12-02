use std::env;
use std::fs;

const PATH: &str = "../../inputs/day";

pub fn read(day_num: i32) -> String {
    let final_path: &str = &(PATH.to_owned() + &day_num.to_string() + ".txt");
    let content = fs::read_to_string(final_path).expect("Should have been able to read the file");
    return content;
}
