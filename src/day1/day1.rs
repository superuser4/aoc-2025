use std::fs;

pub fn day1() {
   let mut lines: Vec<String> = Vec::new();
   for line in fs::read_to_string("./src/day1/input.txt").unwrap().lines() {
       lines.push(line.to_string());
   }

  
   const UPPER_BOUND: i32 = 99;
   const LOWER_BOUND: i32 = 0;
   
   let mut count: u32 = 0;
   let mut vault_loc: i32 = 50;
   
    for line in lines {
        let mut chars = line.chars();
        let minlen: usize = 2;
        if &chars.clone().count() < &minlen {
            continue;
        }

        let first = chars.next().unwrap();
        let rotation: i32 = chars.collect::<String>().parse().unwrap();
        
        let last_num = vault_loc.clone();
        if first  == 'R' {
            vault_loc += rotation;
        } else if first == 'L' {
            vault_loc -= rotation;
        }
        
        while vault_loc > UPPER_BOUND {
            let diff = vault_loc - UPPER_BOUND -1;
            vault_loc = diff;
        }
        while vault_loc < LOWER_BOUND {
            let diff = UPPER_BOUND - vault_loc.abs() + 1;
            vault_loc = diff;
        }

        if vault_loc == 0 {
            println!("vault is 0, last operation was: {line} on the vault_loc: {last_num}");
            count += 1;
        }
    }
    println!("Count: {count}");
}
