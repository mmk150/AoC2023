use std::cmp::max;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_num: i32 = args[1].parse::<i32>().unwrap();
    let contents = day2::read(day_num).to_owned();
    let ans1 = part_one(&contents);
    let ans2 = part_two(&contents);
    println!("Part1 answer:{}", ans1);
    println!("Part2 answer:{}", ans2);
}

fn part_one(contents: &str) -> i32 {
    //red,green,blue
    let MAX: Vec<i32> = vec![12, 13, 14];

    let mut sum: i32 = 0;
    let mut i: i32 = 0;
    for line in contents.lines() {
        let mut check: Vec<i32> = vec![0, 0, 0];
        i += 1;
        let newline = line.split(":").collect::<Vec<&str>>()[1];
        for round in newline.split(";") {
            for color in round.split(",") {
                let parts = color.trim().split(" ").collect::<Vec<&str>>();
                let num = parts[0].parse::<i32>().unwrap();
                let colval = parts[1];

                match colval {
                    "red" => {
                        check[0] = max(num, check[0]);
                    }
                    "green" => {
                        check[1] = max(num, check[1]);
                    }
                    "blue" => {
                        check[2] = max(num, check[2]);
                    }
                    _ => {
                        println!("error, thats weird");
                    }
                }
            }
        }
        let mut enoughCubes: bool = true;
        println!("Line47");
        println!("{}", line);
        println!("{}", sum);
        println!("{:?}", MAX);
        println!("{:?}", check);
        for j in 0..3 {
            let res = MAX[j] - check[j];
            if res < 0 {
                enoughCubes = false;
                println!("line 57:{}", res);
            }
        }
        if enoughCubes == true {
            sum += i;
        }
        println!("{}", sum);
    }
    return sum;
}

fn part_two(contents: &str) -> i32 {
    let mut sum: i32 = 0;
    let mut i: i32 = 0;
    for line in contents.lines() {
        let mut check: Vec<i32> = vec![0, 0, 0];
        i += 1;
        let newline = line.split(":").collect::<Vec<&str>>()[1];
        for round in newline.split(";") {
            for color in round.split(",") {
                let parts = color.trim().split(" ").collect::<Vec<&str>>();
                //println!("{:?}", parts);
                let num = parts[0].parse::<i32>().unwrap();
                let colval = parts[1];
                match colval {
                    "red" => {
                        check[0] = max(num, check[0]);
                    }
                    "green" => {
                        check[1] = max(num, check[1]);
                    }
                    "blue" => {
                        check[2] = max(num, check[2]);
                    }

                    _ => {
                        println!("error, thats weird");
                    }
                }
            }
        }
        let mut prod = 1;
        for j in 0..3 {
            prod *= max(1, check[j]);
        }
        sum += prod;
    }
    return sum;
}
