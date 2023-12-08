use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseInstError;

impl From<&u8> for Instruction {
    fn from(value: &u8) -> Self {
        match value {
            b'R' => Instruction::Right,
            b'L' => Instruction::Left,
            _ => panic!("Wrong Val: {}", value),
        }
    }
}

fn part1(input: &str) -> u64 {
    let insts: Vec<Instruction> = input
        .lines()
        .next()
        .unwrap()
        .as_bytes()
        .iter()
        .filter(|&x| x != &b'\n')
        .map(|x| Instruction::from(x))
        .collect();
    let lines: Vec<&str> = input.lines().skip(2).collect();
    let mut idx_inst = 0;
    let mut idx_lines = 0;
    let mut nbr_jmp = 0;
    let mut idx1;
    let mut idx2;
    let mut map_lines: HashMap<&str, usize> = HashMap::new();
    {
        let mut idx = 0;
        for line in lines.clone() {
            map_lines.insert(&line[..3], idx);
            idx += 1;
        }
    }
    println!("building hashmap done");
    loop {
        if lines[idx_lines].starts_with("ZZZ") {
            break;
        }
        nbr_jmp += 1;
        match insts[idx_inst] {
            Instruction::Left => {
                idx1 = 7;
                idx2 = 10;
                
            },
            Instruction::Right => {
                idx1 = 12;
                idx2 = 15;
            },
        }
        let target = &lines[idx_lines][idx1..idx2];
        idx_lines = map_lines[target];
        idx_inst += 1;
        if idx_inst >= insts.len() {
            idx_inst = 0;
        }
    }
    nbr_jmp
}

fn main() {
    let input = include_str!("../input.txt");
    println!("part1 result = {}", part1(input));
}

#[cfg(test)]
mod test {
    use crate::part1;

    #[test]
    fn exemple_test_1() {
        let input = include_str!("../exemple_1.txt");
        assert_eq!(part1(input), 2);
        let input = include_str!("../exemple_2.txt");
        assert_eq!(part1(input), 6);
    }
}
