use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_num: i64 = args[1].parse::<i64>().unwrap();
    let contents = day5::read(day_num).to_owned();
    let ans1 = part_one(&contents);
    println!("Part1 answer:{}", ans1);
    let ans2 = part_two(&contents);
    println!("Part2 answer:{}", ans2);
}

fn part_one(contents: &str) -> i64 {
    let mut sum = 0;
    let mut tuplez = day5::parse_contents(contents);
    let mut seeds = tuplez.0;
    let mut pile_of_maps = tuplez.1;
    let mut rez: Vec<i64> = Vec::new();
    let mut temp: i64 = 0;
    for seed_num in seeds {
        temp = seed_num;
        for mapg in pile_of_maps.clone() {
            // println!("seednum:{}", seed_num);
            // println!("tempvals:{}", temp);
            for actual_map in mapg.map_list {
                let mut rets = actual_map.num_map(temp);
                if rets.1 {
                    temp = rets.0;
                    break;
                }
            }
        }
        rez.push(temp)
    }
    let mut min: i64 = i64::MAX;
    for val in rez {
        // println!("vals:{}", val);
        if val < min {
            min = val;
        }
    }

    return min;
}

fn part_tworh(contents: &str) -> i64 {
    let mut sum = 0;
    let mut tuplez = day5::parse_contents(contents);
    let mut seeds = tuplez.0;
    let mut actualseeds: Vec<day5::Range> = Vec::new();
    let mut first: i64 = 0;
    let mut second: i64 = 0;
    for (i, val) in seeds.iter().enumerate() {
        if (i & 1) == 0 {
            first = *val;
        } else {
            second = *val;
            let mut new_ranger = day5::Range::new(first, first + second, second);
            actualseeds.push(new_ranger);
        }
    }
    println!("actualseeds:{}", actualseeds.len());
    let mut pile_of_maps = tuplez.1;
    let mut rez: Vec<Vec<day5::Range>> = Vec::new();

    for seed_range in actualseeds {
        let mut temp: Vec<day5::Range> = Vec::new();
        temp.push(seed_range.clone());
        for mapg in pile_of_maps.clone() {
            // println!("seednum:{:?}", seed_range);
            // println!("tempvals:{:?}", temp);
            for actual_map in mapg.map_list {
                let mut rets = temp
                    .iter()
                    .map(|x| (actual_map.range_map(*x).0))
                    .flatten()
                    .collect();
                if rets != temp {
                    temp = rets;
                    break;
                }
            }
        }
        rez.push(temp)
    }
    let mut min: i64 = i64::MAX;
    for vec in rez {
        for rngy in vec {
            // println!("lol:{:?}", rngy);
            if rngy.start < min {
                min = rngy.start;
            }
            // if rngy.end < min {
            //     min = rngy.end;
            // }
        }
    }
    return min;
}

fn part_two(contents: &str) -> i64 {
    let mut sum: i64 = i64::MAX;
    let mut tuplez = day5::parse_contents(contents);
    let mut seeds = tuplez.0;

    let mut first: i64 = 0;
    let mut second: i64 = 0;
    let mut pile_of_maps = tuplez.1;
    for (i, val) in seeds.iter().enumerate() {
        if (i & 1) == 0 {
            first = *val;
        } else {
            second = *val;
            let mut actualseeds: Vec<i64> = Vec::new();
            for iter in first..first + second - 1 {
                let mut temp = iter;
                for mapg in &pile_of_maps {
                    for actual_map in &mapg.map_list {
                        let mut rets = actual_map.num_map(temp);
                        if rets.1 {
                            temp = rets.0;
                            break;
                        }
                    }
                }
                actualseeds.push(temp);
            }
            let mut min: i64 = i64::MAX;
            for val in actualseeds {
                if val < min {
                    min = val;
                }
            }
            println!("min cand:{}", min);
            if min < sum {
                sum = min;
            }
        }
    }

    return sum;
}
