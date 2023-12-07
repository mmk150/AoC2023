use std::env;
use std::fs;
#[allow(dead_code)]
const PATH: &str = "../../inputs/day";
const TEST_PATH: &str = "../../tests/day";

#[derive(Clone)]
pub struct Card {
    pub card_value: i32,
    pub card_matches: i32,
}

pub fn generate_card_vals(contents: &str) -> Vec<Card> {
    let mut cardz: Vec<Card> = Vec::new();
    for line in contents.lines() {
        let newline = line.split(":").collect::<Vec<&str>>()[1];
        let line_split = newline.split("|").collect::<Vec<&str>>();
        let winning_part = line_split[0].trim().split(" ").collect::<Vec<&str>>();
        let your_part = line_split[1].trim().split(" ").collect::<Vec<&str>>();

        let mut vec_win: Vec<i32> = Vec::new();
        let mut vec_yours: Vec<i32> = Vec::new();
        for num in your_part {
            if num == " " || num == "" {
                continue;
            }
            let num_as_int = num.parse::<i32>().unwrap();
            vec_yours.push(num_as_int);
        }
        for num in winning_part {
            if num == " " || num == "" {
                continue;
            }
            let num_as_int = num.parse::<i32>().unwrap();
            vec_win.push(num_as_int);
        }
        let mut matches = 0;
        for number1 in vec_win {
            for number2 in &vec_yours {
                if number1 == number2.clone() {
                    matches += 1;
                    break;
                }
            }
        }
        if matches == 0 {
            let new_card = Card {
                card_value: 0,
                card_matches: 0,
            };
            cardz.push(new_card);
            continue;
        }
        let mut prod_val = 1;
        for i in 1..matches {
            prod_val *= 2;
        }
        let new_card = Card {
            card_value: prod_val,
            card_matches: matches,
        };
        cardz.push(new_card);
    }
    return cardz;
}

pub fn read(day_num: i32) -> String {
    let final_path: &str = &(PATH.to_owned() + &day_num.to_string() + ".txt");
    println!("{}", final_path);
    let content = fs::read_to_string(final_path).expect("Should have been able to read the file");
    return content;
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let day_num: i32 = args[1].parse::<i32>().unwrap();
    read(day_num);
}
