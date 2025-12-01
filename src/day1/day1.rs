use std::fs;

pub fn day1() {
   let mut lines: Vec<String> = Vec::new();
   for line in fs::read_to_string("./src/day1/input.txt").unwrap().lines() {
       lines.push(line.to_string());
   }

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
        
        if first  == 'R' {
            vault_loc += rotation;
        } else if first == 'L' {
            vault_loc -= rotation;
        }
        
        vault_loc = vault_loc.rem_euclid(100);

        if vault_loc == 0 {
            count += 1;
        }
    }
    println!("Count: {count}");
}

pub fn day1_part2() {
   let mut lines: Vec<String> = Vec::new();
   for line in fs::read_to_string("./src/day1/input.txt").unwrap().lines() {
       lines.push(line.to_string());
   }

   let mut count: i64 = 0;
   let mut vault_loc: i64 = 50;
   
    for line in lines {
        let mut chars = line.chars();
        let minlen: usize = 2;
        if &chars.clone().count() < &minlen {
            continue;
        }

        let first = chars.next().unwrap();
        let rotation: i64 = chars.collect::<String>().parse().unwrap();
        
        let old_loc = vault_loc;
        if first == 'R' {
            vault_loc += rotation;
            if vault_loc == 0 {count +=1;
            } else if vault_loc > 99 {
                count += vault_loc / 100;
                vault_loc = vault_loc.rem_euclid(100);
            }
        } else if first == 'L' {
            vault_loc -= rotation;
            if vault_loc == 0 { count +=1; }
            else if vault_loc < 0 {
                if old_loc != 0 {
                    count += 1;
                }
                count += -vault_loc / 100;
                vault_loc = vault_loc.rem_euclid(100);
            }
        }
    }
    println!("Count: {count}");
}
