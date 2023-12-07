use std::env;
use std::fs;
#[allow(dead_code)]

const path: &str = "../../inputs/day";
const test_path: &str = "../../tests/day";

pub struct Gear {
    pub or_pa: Op,
    pub gpnums: Vec<PartNum>,
}

#[derive(Clone)]
pub struct Op {
    pub row_num: i32,
    pub col_num: i32,
}

impl Op {
    pub fn new(x: i32, y: i32) -> Op {
        return Op {
            row_num: x,
            col_num: y,
        };
    }
}

#[derive(Clone)]
pub struct PartNum {
    pub value: i32,
    pub coords: Vec<Op>,
}

pub fn generate_Partnums(contents: &str) -> Vec<PartNum> {
    let mut vec_nums: Vec<PartNum> = Vec::new();
    let lines = contents.lines();
    for (row, line) in lines.enumerate() {
        let mut temp_sum = 0;
        let mut vec_ops: Vec<Op> = Vec::new();
        let mut str_vec: Vec<char> = Vec::new();
        for (column, chara) in line.chars().enumerate() {
            if chara.is_digit(10) {
                let new_op = Op::new(row as i32, column as i32);
                vec_ops.push(new_op);
                str_vec.push(chara);

                if column == line.len() - 1 {
                    let mut strslice = "".to_owned();
                    for charac in &str_vec {
                        strslice.push(*charac);
                    }
                    if strslice == "".to_owned() {
                        continue;
                    }
                    temp_sum = strslice.parse::<i32>().unwrap();
                    let pnum: PartNum = PartNum {
                        value: temp_sum,
                        coords: vec_ops,
                    };
                    vec_nums.push(pnum);
                    vec_ops = Vec::new();
                    str_vec = Vec::new();
                }
            } else {
                let mut strslice = "".to_owned();
                for charac in &str_vec {
                    strslice.push(*charac);
                }
                if strslice == "".to_owned() {
                    continue;
                }
                temp_sum = strslice.parse::<i32>().unwrap();
                let pnum: PartNum = PartNum {
                    value: temp_sum,
                    coords: vec_ops,
                };
                vec_nums.push(pnum);
                vec_ops = Vec::new();
                str_vec = Vec::new();
            }
        }
    }

    return vec_nums;
}

pub fn generate_gears(contents: &str) -> Vec<Gear> {
    let mut vec_our_gears: Vec<Gear> = Vec::new();
    let vec_pnums: Vec<PartNum> = generate_Partnums(contents);
    let lines = contents.lines();
    let mut vec_gear_syms: Vec<Op> = Vec::new();
    for (row, line) in lines.enumerate() {
        for (col, chara) in line.chars().enumerate() {
            if chara == '*' {
                let new_op: Op = Op::new(row as i32, col as i32);
                vec_gear_syms.push(new_op);
            }
        }
    }
    for gear_maybe in vec_gear_syms {
        let mut gear_pnum_list: Vec<PartNum> = Vec::new();
        for pnum in &vec_pnums {
            if calc_dist_pnum(&gear_maybe, &pnum) {
                gear_pnum_list.push(pnum.clone());
            }
        }
        let new_gear = Gear {
            or_pa: gear_maybe,
            gpnums: gear_pnum_list,
        };
        if new_gear.gpnums.len() == 2 {
            vec_our_gears.push(new_gear);
        }
    }
    return vec_our_gears;
}

//return true if any pair is within distance, otherwise false
pub fn calc_dist_pnum(or_pa: &Op, pnum: &PartNum) -> bool {
    for pair in pnum.coords.iter() {
        let xdiff = (pair.row_num - or_pa.row_num) as i32;
        let ydiff = (pair.col_num - or_pa.col_num) as i32;
        let dist = (xdiff.abs() * xdiff.abs() + ydiff.abs() * ydiff.abs()) as f32;
        let newdist = dist.sqrt().floor() as i32;

        if newdist <= 1 {
            return true;
        }
    }
    return false;
}

pub fn read(day_num: i32) -> String {
    let final_path: &str = &(path.to_owned() + &day_num.to_string() + ".txt");
    println!("{}", final_path);
    let content = fs::read_to_string(final_path).expect("Should have been able to read the file");
    return content;
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let day_num: i32 = args[1].parse::<i32>().unwrap();
    read(day_num);
}
