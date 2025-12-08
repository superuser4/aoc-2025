use std::fs;

pub fn day6() {
    let fp = fs::read_to_string("src/day6/input.txt").unwrap();
    let rows: Vec<Vec<&str>> = fp.lines()
        .map(|a| a.trim())
        .filter(|lin| !lin.is_empty())
        .map(|lin| lin.split_whitespace().collect())
        .collect();
    
    let max_col = rows.iter().map(|r| r.len()).max().unwrap_or(0);
    let mut cols: Vec<String> = vec![String::new(); max_col];
    for row in &rows {
        for (i, val) in row.iter().enumerate() {
            if !cols[i].is_empty() {
                cols[i].push(' ');
            }
            cols[i].push_str(val);
        } 
    }

    let mut grand_total: u64 = 0;
    for (_i, col) in cols.iter().enumerate() {
        let mut col_total: u64 = 0;
        let spl_v: Vec<String> = col.trim().split(" ").map(|d| d.to_string()).collect();
        let mut num_vec: Vec<u64> = Vec::new();
        for num_str in spl_v {
            if num_str == "+" {
                let total: u64 = num_vec.iter().sum();
                col_total = total;
            } else if num_str == "*" {
                let mut total = 1;
                for num in num_vec.iter() {
                    total *= num;
                }
                col_total = total;

            } else {
                let num = num_str.parse().unwrap();
                num_vec.push(num);
            }
        }
        grand_total += col_total;
    }
    println!("total: {grand_total}");
}

