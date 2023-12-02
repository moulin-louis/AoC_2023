use std::{fs::read, path::Path};

fn extract_nbr_part1(input: &String, cubes: &mut [u64; 3]) {
    let mut idx: usize = 0;
    for letter in input.chars() {
        if letter.is_numeric() {
            idx += 1;
            continue;
        }
    }
    let nbr: u64 = String::from_utf8(input.as_bytes()[0..idx].to_vec())
        .unwrap()
        .parse()
        .unwrap();
    match input.chars().nth(idx + 1).unwrap() {
        'r' => cubes[0] = cubes[0].max(nbr),
        'g' => cubes[1] = cubes[1].max(nbr),
        'b' => cubes[2] = cubes[2].max(nbr),
        _ => (),
    }
}

fn extract_result(input: &str, rules: [u64; 3], part: bool) -> u64 {
    let mut result: u64 = 0;
    let mut cubes: [u64; 3];
    for line in input.lines() {
        cubes = [0, 0, 0];
        let idx_ddot = line.find(':').unwrap();
        let id: u64 = String::from_utf8(line.as_bytes()[5..idx_ddot].to_vec())
            .unwrap()
            .parse()
            .unwrap();
        let _: Vec<_> = line.as_bytes()[idx_ddot + 1..]
            .split(|x| x == &b',' || x == &b';')
            .map(|x| String::from_utf8(x[1..].to_vec()).unwrap())
            .map(|x| extract_nbr_part1(&x, &mut cubes))
            .collect();
        if part {
            if !(cubes[0] > rules[0] || cubes[1] > rules[1] || cubes[2] > rules[2]) {
                result += id;
            }
        } else {
            result += cubes[0] * cubes[1] * cubes[2];
        }
    }
    result
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
    println!(
        "result part1 = {}",
        extract_result(&input, [12, 13, 14], true)
    );
    println!(
        "result part2 = {}",
        extract_result(&input, [0, 0, 0], false)
    );
}

#[cfg(test)]
mod test {

    use crate::*;

    #[test]
    fn exemple_test_1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string();
        //part 1
        assert_eq!(extract_result(&input, [12, 13, 14], true), 8);
    }

    #[test]
    fn exemple_test_2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string();
        //part 2
        assert_eq!(extract_result(&input, [0, 0, 0], false), 2286);
    }
}
