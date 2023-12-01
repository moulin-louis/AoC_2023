use std::fs::read;
use std::path::Path;

// enum Val<'a> {
//     Str(&'a str),
//     Nbr(u32),
// }

// const NUMBER: [Val; 18] = [
//     Val::Str("one"),
//     Val::Str("two"),
//     Val::Str("three"),
//     Val::Str("four"),
//     Val::Str("five"),
//     Val::Str("six"),
//     Val::Str("seven"),
//     Val::Str("height"),
//     Val::Str("nine"),
//     Val::Nbr(1),
//     Val::Nbr(2),
//     Val::Nbr(3),
//     Val::Nbr(4),
//     Val::Nbr(5),
//     Val::Nbr(6),
//     Val::Nbr(7),
//     Val::Nbr(8),
//     Val::Nbr(9),
// ];
//
const NUMBERS: [&str; 18] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
    "5", "6", "7", "8", "9",
];

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

fn calculate_result(input: String) -> usize {
    let mut cal_vale: Vec<usize> = Vec::new();
    for line in input.lines() {
        let mut first_idx: usize = 0;
        let mut last_x: i32 = -1;
        for nbr in NUMBERS {
            match line.find(nbr) {
                Some(x) => {
                    if x as i32 > last_x && last_x != -1 {
                        continue;
                    }
                    last_x = x as i32;
                    first_idx = NUMBERS.iter().position(|x| x == &nbr).unwrap();
                }
                None => continue,
            };
        }
        let mut last_idx: usize = 0;
        last_x = 0;
        for nbr in NUMBERS {
            match line.rfind(nbr) {
                Some(x) => {
                    if (x as i32) < last_x && last_x != -1 {
                        continue;
                    }
                    last_x = x as i32;
                    last_idx = NUMBERS.iter().position(|x| x == &nbr).unwrap();
                }
                None => continue,
            };
        }
        let mut first_val = first_idx;
        let mut last_val = last_idx;
        if first_val <= 8 {
            first_val += 1;
        } else {
            first_val -= 8;
        }
        if last_val <= 8 {
            last_val += 1;
        } else {
            last_val -= 8;
        }
        let result = first_val * 10 + last_val;
        cal_vale.push(result);
    }
    cal_vale.iter().sum()
}

fn main() {
    let input = read_file("./input.txt").expect("CANT READ FILE");
    println!("result = {}", calculate_result(input));
}

#[cfg(test)]
mod test {
    use crate::calculate_result;

    #[test]
    fn example_test() {
        // let input = "xtwone3four\n".to_string();
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen".to_string();
        assert_eq!(calculate_result(input), 281);
    }
}
