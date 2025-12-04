use std::fs;

pub fn day4() {
    let s = fs::read_to_string("./src/day4/input.txt").unwrap();
    let lines: Vec<String> = s.lines().map(|i| i.to_string()).collect();

    let m: [[i32; 2]; 8] = [
        [-1, -1], [-1, 0], [-1, 1],
        [0, -1],          [0, 1],
        [1, -1], [1 , 0], [1, 1]
    ];
    
    let mut total = 0;
    for i in 0..lines.len() {
        let len_line = lines[i].len();
        let cur_lin: Vec<char> = lines[i].chars().collect();
        for j in 0..len_line{
            let mut count: u64 = 0;
            if cur_lin[j] == '@' {
                // check diagonals + up/down
                for elem in m.iter(){
                    let row_loc: i32 = (j as i32 + elem[0]).try_into().unwrap();
                    let col_loc: i32 = (i as i32 + elem[1]).try_into().unwrap();
                    if (row_loc > (len_line as i32 -1) || row_loc < 0) || (col_loc < 0 || col_loc > lines.len() as i32) {
                        continue;
                    }

                    let other_line: Vec<char> = lines[col_loc as usize].chars().collect(); 
                    if other_line.is_empty() {
                        continue;
                    }
                    if other_line[row_loc as usize] == '@' {
                        count += 1;
                    }
                }
                if count < 4 {
                   total += 1;
                }
            }
        }
    }
    println!("Total: {total}");
}
