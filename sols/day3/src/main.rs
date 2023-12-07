use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_num: i32 = args[1].parse::<i32>().unwrap();
    let contents = day3::read(day_num).to_owned();
    let ans1 = part_one(&contents);
    let ans2 = part_two(&contents);
    println!("Part1 answer:{}", ans1);
    println!("Part2 answer:{}", ans2);
}

fn part_one(contents: &str) -> i32 {
    let mut sum = 0;
    let pnums = day3::generate_Partnums(contents);

    let mut lines = contents.lines();

    for (row, line) in lines.enumerate() {
        for (col, chara) in line.chars().enumerate() {
            let mut or_pa = day3::Op::new(row as i32, col as i32);
            if chara.is_digit(10) {
                continue;
            } else {
                if chara != '.' {
                    for pnum in pnums.iter() {
                        if day3::calc_dist_pnum(&or_pa, &pnum) {
                            sum += pnum.value;
                        }
                    }
                }
            }
        }
    }

    return sum;
}

fn part_two(contents: &str) -> i32 {
    let mut sum = 0;
    let gearList: Vec<day3::Gear> = day3::generate_gears(contents);
    for gear in gearList {
        let mut prod = 1;
        for nums in gear.gpnums {
            prod *= nums.value;
        }
        sum += prod;
    }
    return sum;
}
