use std::{fs::read, path::Path};

#[derive(Debug, Clone)]
struct Ratio {
    nbr1: u64,
    nbr2: u64,
    star_x: usize,
    star_y: usize,
}


//part 1
fn check_adjacent_cells(arr: &[&[u8]], row: usize, col: usize) -> bool {
    let directions: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let max_row = arr.len();
    let max_col = arr[0].len();

    for (dr, dc) in directions.iter() {
        let new_row = row as i32 + dr;
        let new_col = col as i32 + dc;
        
        if new_row >= 0
            && new_row < max_row as i32
            && new_col >= 0
            && new_col < max_col as i32
        {
            let c = arr[new_row as usize][new_col as usize] as char;
            if c == '.' {
                continue;
            }
            if !c.is_numeric() {
                return true;
            }
        }
    }
    false
}

//return true if symbols near [x,y]
fn check_nears(arr: &[&[u8]], idx_line: usize, idx_start: usize, idx_end: usize) -> bool {
    for idx in idx_start..idx_end {
        if check_adjacent_cells(arr, idx_line, idx) {
            return true;
        }
    }
    false
}

fn part1(input: String) -> u64 {
    let mut result: u64 = 0;
    let input: Vec<&[u8]> = input.split('\n').map(|x| x.as_bytes()).collect();
    let mut idx = 0;
    while idx < input.len() {
        // println!("idx = {}", idx);
        let mut idx2 = 0;
        while idx2 < input[idx].len() {
            // println!("idx2 = {}", idx2);
            if !(input[idx][idx2] as char).is_numeric() {
                idx2 += 1;
                continue;
            }
            let mut end_nbr = idx2;
            while end_nbr < input[idx].len() && (input[idx][end_nbr] as char).is_numeric() {
                end_nbr += 1;
            }
            let nbr: u64 = String::from_utf8(input[idx][idx2..end_nbr].to_vec()).unwrap().parse().unwrap();
            if check_nears(input.as_slice(), idx, idx2, end_nbr) {
                result += nbr;
            }
            idx2 += end_nbr - idx2;
        }
        idx += 1;
    }
    result
}

//part 2
fn adjacent_cells_part2(arr: &[&[u8]], row: usize, col: usize) -> Option<(usize, usize)> {
    let directions: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let max_row = arr.len();
    let max_col = arr[0].len();

    for (dr, dc) in directions.iter() {
        let new_row = row as i32 + *dr;
        let new_col = col as i32+ *dc;
        
        if new_row >= 0
            && new_row < max_row as i32
            && new_col >= 0
            && new_col < max_col as i32
        {
            let c = arr[new_row as usize][new_col as usize] as char;
            if c == '.' {
                continue;
            }
            if c == '*' {
                return Some((new_row as usize, new_col as usize));
            }
        }
    }
    None
}

fn check_nears_part2(arr: &[&[u8]], idx_line: usize, idx_start: usize, idx_end: usize, nbr: u64, result: &mut Vec<Ratio>) -> bool {
    for idx in idx_start..idx_end {
        if let Some((x,y)) = adjacent_cells_part2(arr, idx_line, idx) {
            let iter = result.iter_mut().find(|rt| rt.star_x == x && rt.star_y == y);
            match iter {
                None =>  result.push(Ratio { nbr1: nbr, nbr2: 0, star_x: x, star_y: y }),
                Some(rt) => {
                    rt.nbr2 = nbr;
                }
            }
            break;
        }
    }
    false
}

fn part2(input: String) -> u64 {
    let mut result: Vec<Ratio> = Vec::new();
    let input: Vec<&[u8]> = input.split('\n').map(|x| x.as_bytes()).collect();
    let mut idx = 0;
    while idx < input.len() {
        let mut idx2 = 0;
        while idx2 < input[idx].len() {
            if !(input[idx][idx2] as char).is_numeric() {
                idx2 += 1;
                continue;
            }
            let mut end_nbr = idx2;
            while end_nbr < input[idx].len() && (input[idx][end_nbr] as char).is_numeric() {
                end_nbr += 1;
            }
            let nbr: u64 = String::from_utf8(input[idx][idx2..end_nbr].to_vec()).unwrap().parse().unwrap();
            check_nears_part2(input.as_slice(), idx, idx2, end_nbr, nbr, &mut result);
            idx2 += end_nbr - idx2;
        }
        idx += 1;
    }
    let mut result_nbr: u64 = 0;
    let result: Vec<&Ratio> = result.iter().filter(|x| x.nbr2 != 0 && x.nbr1 != 0).collect();
    for ratio in result.iter() {
        result_nbr = result_nbr.checked_add(ratio.nbr1.checked_mul(ratio.nbr2).unwrap()).unwrap();
    }
    result_nbr
}

fn read_file(path: &str) -> Option<String> {
    let input = match read(Path::new(path)) {
        Ok(x) => x,
        Err(err) => {
            eprintln!("Cant read file: {err}");
            return None;
        }
    };
    String::from_utf8(input).ok()
}

fn main() {
    
    let input = read_file("./input.txt").unwrap();
    println!("result part 1 = {}", part1(input.clone()));
    println!("result part 2 = {}", part2(input));
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn exemple_test_1() {
        let input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..".to_string();
        //part 1
        assert_eq!(part1(input), 4361);
    }

    #[test]
    fn exemple_test_2() {
        let input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..".to_string();
        //part 2
        assert_eq!(part2(input), 467835);
    }
}
