use std::fs::read;
use std::path::Path;

const NUMBERS: [&str; 18] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
    "5", "6", "7", "8", "9",
];

fn calculate_result(input: String) -> usize {
    let mut result: usize = 0;
    for line in input.lines() {
        let mut first_idx: usize = 42;
        let mut last_idx: usize = 42;
        for (index, _) in line.char_indices() {
            for idx in 0..NUMBERS.len() {
                if line[index..].starts_with(NUMBERS[idx]) {
                    first_idx = idx;
                    break;
                }
            }
            if first_idx != 42 { break; }
        }
        for (index, _) in line.char_indices().rev() {
            for idx in 0..NUMBERS.len() {
                if line[..(index + 1)].ends_with(NUMBERS[idx]) {
                    last_idx = idx;
                    break;
                }
            }
            if last_idx != 42 { break; }
        }
        if first_idx <= 8 { first_idx += 1; }
        else { first_idx -= 8; }
        if last_idx <= 8 { last_idx += 1; }
        else { last_idx -= 8; }
        result += first_idx * 10 + last_idx;
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
    let input = read_file("./input.txt").expect("CANT READ FILE");
    println!("result = {}", calculate_result(input));
}

#[cfg(test)]
mod test {
    use crate::calculate_result;

    #[test]
    fn example_test() {
        let input = "xtwone3four\n".to_string();
        assert_eq!(calculate_result(input.clone()), 24);

        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen".to_string();
        assert_eq!(calculate_result(input), 281);
    }
}
