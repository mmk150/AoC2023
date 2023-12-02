use std::env;
use std::fs;

const PATH: &str = "../../inputs/day";

fn read(day_num: i32) -> String {
    let final_path: &str = &(PATH.to_owned() + &day_num.to_string() + ".txt");
    let content = fs::read_to_string(final_path).expect("Should have been able to read the file");
    return content;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_num: i32 = args[1].parse::<i32>().unwrap();
    let contents = read(day_num).to_owned();
    let ans1 = part_one(&contents);
    let ans2 = part_two(&contents);
    println!("Part1 answer:{}", ans1);
    println!("Part2 answer:{}", ans2);
}

fn part_one(content: &str) -> i32 {
    let mut sum = 0;
    for line in content.lines() {
        let mut first = 2;
        let mut fstring = "a".to_owned();
        let mut lstring = "a".to_owned();
        for chara in line.chars() {
            if chara.is_digit(10) {
                if first != 0 {
                    fstring = chara.to_string().to_owned();
                    first = 0;
                }
                lstring = chara.to_string().to_owned();
            }
        }
        let sumstring = fstring.clone() + &lstring;
        sum += sumstring.parse::<i32>().unwrap();
    }
    return sum;
}

fn check_substring(substring: &str) -> bool {
    match substring {
        "zero" | "one" | "two" | "three" | "four" | "five" | "six" | "seven" | "eight" | "nine" => {
            true
        }
        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => true,
        _ => false,
    }
}

fn conv_substring(substring: &str) -> &str {
    match substring {
        "zero" => "0",
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => substring,
    }
}

fn part_two(content: &str) -> i32 {
    let mut sum = 0;

    for line in content.lines() {
        let mut charvec: Vec<&str> = Vec::new();
        for i in 0..line.len() {
            for j in i + 1..line.len() + 1 {
                let substring = &line[i..j];
                let is_dig = check_substring(substring);
                if is_dig {
                    charvec.push(conv_substring(substring));
                }
            }
        }
        let fstring = charvec[0].to_string();
        let lstring = charvec[charvec.len() - 1].to_string();
        let sumstring = fstring.clone() + &lstring;
        sum += sumstring.parse::<i32>().unwrap();
    }
    return sum;
}
