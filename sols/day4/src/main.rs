use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_num: i32 = args[1].parse::<i32>().unwrap();
    let contents = day4::read(day_num).to_owned();
    let ans1 = part_one(&contents);
    let ans2 = part_two(&contents);
    println!("Part1 answer:{}", ans1);
    println!("Part2 answer:{}", ans2);
}

fn part_one(contents: &str) -> i32 {
    let mut sum = 0;
    let card_vec: Vec<day4::Card> = day4::generate_card_vals(contents);
    for card in card_vec {
        sum += card.card_value;
    }
    return sum;
}

fn part_two(contents: &str) -> i32 {
    let mut sum = 0;
    let card_vec: Vec<day4::Card> = day4::generate_card_vals(contents);
    let mut matches = 0;
    let mut card_copies: Vec<i32> = vec![1; card_vec.len()];
    for (i, card) in card_vec.iter().enumerate() {
        matches = card.card_matches;
        for k in 0..card_copies[i] {
            for j in 1..matches + 1 {
                let mut sum = j as usize;
                sum = sum + i;
                if sum < card_vec.len() {
                    card_copies[sum] += 1;
                }
            }
        }
    }
    for num in card_copies {
        println!("{}", num);
        sum += num;
    }
    return sum;
}
