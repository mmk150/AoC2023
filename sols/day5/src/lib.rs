use std::env;
use std::fs;
#[allow(dead_code)]
const PATH: &str = "../../inputs/day";
const TEST_PATH: &str = "../../tests/day";

#[derive(Clone)]
pub struct MapGroup {
    //pub map_type: String,
    pub map_list: Vec<Map>,
}

impl MapGroup {
    pub fn new(list: Vec<Map>) -> MapGroup {
        MapGroup {
            //map_type: mtype,
            map_list: list,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Map {
    pub dest_range_start: i64,
    pub source_range_start: i64,
    pub range_length: i64,
}

impl Map {
    pub fn new(dest: i64, source: i64, range: i64) -> Map {
        Map {
            dest_range_start: dest,
            source_range_start: source,
            range_length: range,
        }
    }

    pub fn num_map(self, num: i64) -> (i64, bool) {
        let mut res: i64 = 0;
        let mut didAnything: bool = false;
        if num >= self.source_range_start && num < self.source_range_start + self.range_length {
            res = self.dest_range_start + (num - self.source_range_start);
            didAnything = true;
        } else {
            res = num;
            didAnything = false;
        }
        return (res, didAnything);
    }

    pub fn range_map(self, rng: Range) -> (Vec<Range>, bool) {
        let mut lower = rng.start;
        let mut upper = rng.end;
        let mut didAnything: bool = false;
        if lower >= self.source_range_start + self.range_length || upper < self.source_range_start {
            let mut ret_vec: Vec<Range> = Vec::new();
            ret_vec.push(rng);
            return (ret_vec, false);
        } else {
            if lower <= self.source_range_start
                && upper >= self.source_range_start
                && upper < self.source_range_start + self.range_length
            {
                let new_upper = self.num_map(upper).0;
                let new_lower = self.num_map(self.source_range_start).0;
                let new_len = new_upper - new_lower;
                let new_range = Range::new(new_lower, new_upper, new_len);

                let remains_upper = self.source_range_start - 1;
                let remains_lower = lower;
                let rem_len = remains_upper - remains_lower;
                let rem_range = Range::new(remains_lower, remains_upper, rem_len);

                let mut ret_vec: Vec<Range> = Vec::new();
                ret_vec.push(new_range);
                ret_vec.push(rem_range);
                return (ret_vec, true);
            } else {
                if upper > self.source_range_start + self.range_length
                    && lower <= self.source_range_start + self.range_length
                    && lower >= self.source_range_start
                {
                    let new_upper = self
                        .num_map(self.source_range_start + self.range_length - 1)
                        .0;
                    let new_lower = self.num_map(lower).0;
                    let new_len = new_upper - new_lower;
                    let new_range = Range::new(new_lower, new_upper, new_len);

                    let remains_upper = upper;
                    let remains_lower = self.source_range_start + self.range_length;
                    let rem_len = remains_upper - remains_lower;
                    let rem_range = Range::new(remains_lower, remains_upper, rem_len);

                    let mut ret_vec: Vec<Range> = Vec::new();
                    ret_vec.push(new_range);
                    ret_vec.push(rem_range);
                    return (ret_vec, true);
                } else {
                    if upper < self.source_range_start + self.range_length
                        && lower >= self.source_range_start
                    {
                        let new_upper = self.num_map(upper).0;
                        let new_lower = self.num_map(lower).0;
                        let new_len = new_upper - new_lower;
                        let new_range = Range::new(new_lower, new_upper, new_len);
                        let mut ret_vec: Vec<Range> = Vec::new();
                        ret_vec.push(new_range);
                        return (ret_vec, true);
                    }
                    if upper > self.source_range_start + self.range_length
                        && lower < self.source_range_start
                    {
                        let new_upper = self
                            .num_map(self.source_range_start + self.range_length - 1)
                            .0;
                        let new_lower = self.num_map(self.source_range_start).0;
                        let new_len = new_upper - new_lower;
                        let new_range = Range::new(new_lower, new_upper, new_len);

                        let remains1_upper = upper;
                        let remains1_lower = self.source_range_start + self.range_length;
                        let rem1_len = remains1_upper - remains1_lower;
                        let rem1_range = Range::new(remains1_lower, remains1_upper, rem1_len);

                        let remains2_upper = self.source_range_start;
                        let remains2_lower = lower;
                        let rem2_len = remains2_upper - remains2_lower;
                        let rem2_range = Range::new(remains2_lower, remains2_upper - 1, rem2_len);

                        let mut ret_vec: Vec<Range> = Vec::new();
                        ret_vec.push(new_range);
                        ret_vec.push(rem1_range);
                        ret_vec.push(rem2_range);
                        return (ret_vec, true);
                    } else {
                        let mut ret_vec: Vec<Range> = Vec::new();
                        ret_vec.push(rng);
                        return (ret_vec, false);
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Range {
    pub start: i64,
    pub end: i64,
    pub length: i64,
}

impl Range {
    pub fn new(st: i64, en: i64, len: i64) -> Range {
        Range {
            start: st,
            end: en,
            length: len,
        }
    }
}

pub fn parse_contents(contents: &str) -> (Vec<i64>, Vec<MapGroup>) {
    let mut seeds_vec: Vec<i64> = Vec::new();
    let mut maps_vec: Vec<MapGroup> = Vec::new();
    let mut lines = contents.lines();
    seeds_vec = generate_seeds(lines.next().unwrap());

    let mut rest = lines.collect::<Vec<&str>>().join("\n");

    maps_vec = generate_maps(&rest);

    return (seeds_vec, maps_vec);
}

pub fn generate_seeds(line1: &str) -> Vec<i64> {
    //seeds: 79 14 55 13
    let mut placeholder = line1.split(":").collect::<Vec<&str>>()[1];
    let mut nums = placeholder
        .trim()
        .split(" ")
        .map(|x| {
            if x.trim() != "" {
                return x.trim().parse::<i64>().unwrap();
            } else {
                return -9999;
            }
        })
        .collect();
    return nums;
}
pub fn generate_maps(contents: &str) -> Vec<MapGroup> {
    let mut vec_maps: Vec<MapGroup> = Vec::new();
    let mut maps = contents.trim().split(":").collect::<Vec<&str>>();
    let mut last_map_type = "".to_owned();
    let mut curr_map_type = "".to_owned();
    for (i, item) in maps.iter().enumerate() {
        let mut vecvec: Vec<Vec<i64>> = Vec::new();
        for line in item.lines() {
            curr_map_type = last_map_type.clone();
            if line.contains("map") {
                last_map_type = line.to_owned();
            } else {
                let mut nums = line
                    .trim()
                    .split(" ")
                    .map(|x| {
                        if x.trim() != "" {
                            return x.trim().parse::<i64>().unwrap();
                        } else {
                            return -9999;
                        }
                    })
                    .collect();
                vecvec.push(nums);
            }
        }
        let mut mappygroupy: MapGroup;
        let mut mappyveccy: Vec<Map> = Vec::new();
        //println!("group:{}", curr_map_type);
        for (i, elem) in vecvec.iter().enumerate() {
            if elem.len() == 1 {
                continue;
            }
            //println!(" elem:{:?}", elem);
            let new_map = Map::new(elem[0], elem[1], elem[2]);
            mappyveccy.push(new_map);
        }
        mappygroupy = MapGroup::new(mappyveccy);
        vec_maps.push(mappygroupy);
    }

    return vec_maps;
}

pub fn read(day_num: i64) -> String {
    let final_path: &str = &(PATH.to_owned() + &day_num.to_string() + ".txt");
    println!("{}", final_path);
    let content = fs::read_to_string(final_path).expect("Should have been able to read the file");
    return content;
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let day_num: i64 = args[1].parse::<i64>().unwrap();
    read(day_num);
}
