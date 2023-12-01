use std::fs::read;
use std::path::Path;

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

fn calculate_result(input: String) -> u32 {
    let mut cal_vale: Vec<u32> = Vec::new();
    for line in input.lines() {
        let first_nbr = match line.find(|x: char| x.is_numeric()) {
            Some(x) => x,
            None => continue,
        };
        let last_nrb = match line.rfind(|x: char| x.is_numeric()) {
            Some(x) => x,
            None => continue,
        };
        let val1 = line.as_bytes()[first_nbr];
        let val2 = line.as_bytes()[last_nrb];
        cal_vale.push(((val1 - b'0') * 10 + val2 - b'0').into());
    }
    cal_vale.iter().sum()
}

fn main() {
    let input = read_file("./input.txt").expect("CANT READ FILE");
    println!("result = {}", calculate_result(input));
}

#[cfg(test)]
mod test{
    use crate::calculate_result;

    #[test]
    fn exemple_test() {
        let input: String = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet".to_string();
        assert_eq!(142, calculate_result(input));
    }
}