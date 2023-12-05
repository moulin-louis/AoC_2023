use std::collections::HashMap;

const TRANSFORM: [&str; 7] = [
    "seed-to-soil",
    "soil-to-fertilizer",
    "fertilizer-to-water",
    "water-to-light",
    "light-to-temperature",
    "temperature-to-humidity",
    "humidity-to-location",
];

#[derive(Clone, Copy, Debug)]
struct Range {
    start_src: u64,
    start_dest: u64,
    len: u64,
}

// fn display_map(map: &HashMap<String, Vec<Range>>) {
//     for (key, val) in map.iter() {
//         println!("{} : {:?}\n", key, val);
//     }
// }
fn process_nbr(transform: &str, str: &str, map: &mut HashMap<String, Vec<Range>>) {
    for line in str.lines() {
        let nbr: Vec<u64> = line.split(' ').map(|x| x.parse().unwrap()).collect();
        let range = Range {
            start_src: nbr[1],
            start_dest: nbr[0],
            len: nbr[2],
        };
        if map.contains_key(transform) {
            map.get_mut(transform).unwrap().push(range);
        } else {
            map.insert(transform.to_string(), vec![range]);
        }
    }
}

fn build_map(input: &str) -> HashMap<String, Vec<Range>> {
    let mut result: HashMap<String, Vec<Range>> = HashMap::new();
    for transformation in TRANSFORM {
        let idx = input.find(transformation).unwrap() + transformation.len() + 6;
        let idx_end = input[idx..].find("\n\n").unwrap();
        process_nbr(transformation, &input[idx..idx + idx_end], &mut result);
    }
    result
}

fn process_seed_to_location(seeds: &mut [u64], map: &HashMap<String, Vec<Range>>) -> u64 {
    for trans in TRANSFORM {
        let ranges = map.get(trans).unwrap();
        for seed in seeds.iter_mut() {
            match ranges
                .iter()
                .find(|&x| (x.start_src..x.start_src + x.len).contains(seed))
            {
                None => {}
                Some(x) => *seed = x.start_dest + (*seed - x.start_src),
            };
        }
    }
    *seeds.iter().min().unwrap()
}

fn part1(input: &str) -> u64 {
    let mut seeds: Vec<u64> = input[7..input.find('\n').unwrap()]
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();
    process_seed_to_location(&mut seeds, &build_map(input))
}

fn process_seed_to_location_2(seeds: &mut [u64], map: &HashMap<String, Vec<Range>>) -> u64 {
    let mut current_min: u64 = u64::MAX;
    for idx in (0..seeds.len()).step_by(2) {
        for seed in seeds[idx]..seeds[idx] + seeds[idx + 1] {
            let mut tmp = seed;
            for trans in TRANSFORM {
                let ranges = map.get(trans).unwrap();
                match ranges
                    .iter()
                    .find(|&x| (x.start_src..x.start_src + x.len).contains(&tmp))
                {
                    None => {}
                    Some(x) => tmp = x.start_dest + (tmp - x.start_src),
                };
            }
            if tmp < current_min {
                current_min = tmp;
            }
        }
    }
    current_min
}

fn part2(input: &str) -> u64 {
    let mut seeds: Vec<u64> = input[7..input.find('\n').unwrap()]
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();
    process_seed_to_location_2(&mut seeds, &build_map(input))
}

fn main() {
    let input = include_str!("../input.txt");
    // println!("part1 result = {}", part1(input));
    println!("part2 result = {}", part2(input));
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn exemple_test_1() {
        let input = include_str!("../example.txt");
        assert_eq!(part1(input), 35);
    }

    #[test]
    fn exempl_test_2() {
        let input = include_str!("../example.txt");
        assert_eq!(part2(input), 46);
    }
}
