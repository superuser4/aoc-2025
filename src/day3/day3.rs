use std::fs;

pub fn day3() {
    let f = fs::read_to_string("./src/day3/input.txt").unwrap();
    let lines = f.lines();

    let mut sum: u64 = 0;
    for line in lines {
        let mut num1 = 0;
        let mut num1_idx = 0;
        
        let line_len = line.len();
        for (i, ch) in line.chars().enumerate() {
            let cur_int = ch.to_digit(10).unwrap();
            if cur_int > num1 {
                num1 = cur_int;
                num1_idx = i;
            }
            if i == line_len-2 {
                break;
            }
        }

        let mut num2 = 0;
        for (i, ch) in line.chars().enumerate() {
            let cur_int = ch.to_digit(10).unwrap();
            if cur_int > num2 && i > num1_idx {
                num2 = cur_int;
            }
        }



        let fin_num: u64 = (num1.to_string() + &num2.to_string()).parse().unwrap();
        println!("Line: {line}");
        println!("Biggest number found: {fin_num}\n");
        sum += fin_num;
    }
    println!("Sum: {sum}");
}

pub fn day3_part2() {
    let s = fs::read_to_string("./src/day3/input.txt").unwrap();
    let lines = s.lines();

    let mut sum: u64 = 0;
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let digits: Vec<u32> = line.chars().map(|i| i.to_digit(10).unwrap()).collect();
        let len = digits.len();
        let mut result: Vec<u32> = Vec::with_capacity(12);
        
        let mut start =0;
        for i in 0..12 {
            let end = len.saturating_sub(12-i); 
            let mut max_num = 0;
            let mut max_idx = start;

            for j in start..=end {
                if digits[j] > max_num {
                    max_num = digits[j];
                    max_idx = j;
                }
            }
            result.push(max_num);
            start = max_idx+1;
        }
        let fin_str: String = result.iter().map(|i| i.to_string()).collect();
        let fin_num: u64 = fin_str.parse().unwrap();
        sum += fin_num;
    }
    println!("Sum: {:?}", sum);
}
