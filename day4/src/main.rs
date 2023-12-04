use std::collections::{hash_map, HashMap};
use std::fs::read;
use std::path::Path;

fn part1(input: String) -> u64 {
    let mut result = 0;
    for line in input.lines() {
        let idx2 = line.find('|').unwrap();
        let wining_nbr: Vec<u64> = line[line.find(':').unwrap() + 2..idx2 - 1]
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let draw_nbr: Vec<u64> = line[idx2 + 1..]
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let mut tmp_result = 0;
        for nbr in draw_nbr {
            if wining_nbr.contains(&nbr) {
                if tmp_result == 0 {
                    tmp_result = 1
                } else {
                    tmp_result *= 2;
                }
            }
        }
        result += tmp_result;
    }
    result
}

fn part2(input: String) -> u64 {
    let mut result: HashMap<u64, u64> = HashMap::new();
    let mut len: u64 = 0;
    for line in input.lines() {
        len += 1;
        let id: u64 = line[line.find(|x: char| x.is_numeric()).unwrap()..line.find(':').unwrap()]
            .parse()
            .unwrap();
        let idx2 = line.find('|').unwrap();
        let wining_nbr: Vec<u64> = line[line.find(':').unwrap() + 2..idx2 - 1]
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let draw_nbr: Vec<u64> = line[idx2 + 1..]
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let nbr_wining_nbr: u64 = draw_nbr
            .iter()
            .filter(|&x| wining_nbr.contains(x))
            .collect::<Vec<&u64>>()
            .len() as u64;
        let nbr_card = match result.get(&id) {
            Some(x) => *x + 1,
            None => 1,
        };
        for _card in 0..nbr_card {
            for idx in 1..nbr_wining_nbr + 1 {
                match result.get_mut(&(id + idx)) {
                    Some(x) => *x += 1,
                    None => {
                        result.insert(id + idx, 1);
                    }
                }
            }
        }
    }
    result.values().sum::<u64>() + len
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
    println!("part 1 result = {}", part1(input.clone()));
    println!("part 2 result = {}", part2(input));
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn exemple_test_1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string();
        assert_eq!(part1(input), 13);
    }

    #[test]
    fn exemple_test_2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string();
        assert_eq!(part2(input), 30);
    }
}
