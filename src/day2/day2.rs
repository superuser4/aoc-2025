use std::fs::read_to_string;
use fancy_regex::Regex;

pub fn day2() {
    let input: String = read_to_string("./src/day2/input.txt").unwrap();
    let inp_vec: Vec<&str> = input.split(",").collect();

    let mut final_count: u64 = 0;
    for id_range in inp_vec {
        let v: Vec<&str> = id_range.split("-").collect();
        let left_int: u64 = v[0].trim().parse().unwrap();
        let right_int: u64 = v[1].trim().parse().unwrap();

        for i in left_int..=right_int {
            let s = i.to_string();
            if s.len() % 2 != 0 {
                continue;
            } 
            let (f_half, s_half) = s.split_at(s.len() / 2);
            if f_half == s_half {
                final_count += i;
            }
        }
    }
    println!("Final count: {final_count}"); 
}

pub fn day2_part2() {
    let input: String = read_to_string("./src/day2/input.txt").unwrap();
    let inp_vec: Vec<&str> = input.split(",").collect();

    let re = Regex::new(r"^(\d+)\1+$").unwrap();  
    let mut final_count: u64 = 0;
    for id_range in inp_vec {
        let v: Vec<&str> = id_range.split("-").collect();
        let left_int: u64 = v[0].trim().parse().unwrap();
        let right_int: u64 = v[1].trim().parse().unwrap();

        for i in left_int..=right_int {
            let s = i.to_string();
            if re.is_match(&s).unwrap() {
                final_count += i;
            }
        }
    }
    println!("Final count: {final_count}"); 
}
