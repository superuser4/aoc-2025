use std::fs;

pub fn day5() {
    let input: String = fs::read_to_string("src/day5/input.txt").unwrap();
    let sections: Vec<&str> = input.split("\n\n").collect();
    
    let id_ranges: Vec<String> = sections[0].lines().map(|i| i.to_string()).collect();
    let ids:Vec<String> = sections[1].lines().map(|i| i.to_string()).collect();

    let mut fresh = 0;
    for id in ids {
        for id_range in id_ranges.iter() {
            let spl_v: Vec<String> = id_range.split("-").map(|i| i.to_string()).collect();
            let left_range:u64 = spl_v[0].parse().unwrap();
            let right_range: u64= spl_v[1].parse().unwrap();

            let id_num: u64= id.parse().unwrap();
            
            if id_num >= left_range && id_num <= right_range {
                fresh += 1;
                break;
            }
        }
    }
    println!("Fresh: {fresh}");
}


pub fn day5_part2() {
    let input: String = fs::read_to_string("src/day5/input.txt").unwrap();
    let sections: Vec<&str> = input.split("\n\n").collect();
    let id_ranges: Vec<String> = sections[0].lines().map(|i| i.to_string()).collect();
    
    let mut ranges: Vec<Vec<u64>> = Vec::new();
    let mut fresh = 0;
    for id_range in id_ranges {
        let spl_v: Vec<String> = id_range.split("-").map(|i| i.to_string()).collect();
        let left_range:u64 = spl_v[0].parse().unwrap();
        let right_range: u64= spl_v[1].parse().unwrap();

        ranges.push(vec![left_range, right_range]);
    }
    
    ranges.sort_by(|a,b| a[0].cmp(&b[0]));
    let mut last_start = ranges[0][0]; 
    let mut last_end = ranges[0][1];
    for i in 1..ranges.len() {
        if ranges[i][0] <= last_end {
            if ranges[i][1] > last_end {
                last_end = ranges[i][1];    
            }
        } else {
            fresh += last_end - last_start + 1;
            last_start = ranges[i][0];
            last_end = ranges[i][1];
        }
    }
    fresh += last_end - last_start + 1;

    println!("Fresh: {fresh}");
}
