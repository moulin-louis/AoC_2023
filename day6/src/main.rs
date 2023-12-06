#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn create_races(input: &str) -> Vec<Race> {
    let mut result: Vec<Race> = Vec::new();
    let mut lines = input.lines();
    let times: Vec<u64> = lines.next().unwrap().split(' ').filter(|x| x.parse::<u64>().is_ok()).map(|x| x.parse().unwrap()).collect();
    let distances: Vec<u64> = lines.next().unwrap().split(' ').filter(|x| x.parse::<u64>().is_ok()).map(|x| x.parse().unwrap()).collect();
    for idx in 0..times.len() {
        result.push(Race { time: times[idx], distance: distances[idx] });
    }
    result
}

fn check_every_posibility_race(race: &Race) -> u64 {
    let mut result = 0;
    for x in 1..race.time {
        result += if x * (race.time - x) > race.distance { 1 } else { 0 };
    }
    result
}

fn part1(input: &str) -> u64 {
    let races = create_races(input);
    let mut result = check_every_posibility_race(&races[0]);
    let _ = races.iter().skip(1).map(|x| result *= check_every_posibility_race(x)).collect::<Vec<_>>();
    result
}


fn part2() -> u64 {
    let race = Race{
        time: 35696887,
        distance: 213116810861248,
    };
    check_every_posibility_race(&race)
}

fn main() {
    let input = include_str!("../input.txt");
    println!("part 1 result = {}", part1(input));
    println!("part 2 result = {}", part2());
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn exemple_1() {
        let input = include_str!("../exemple.txt");
        assert_eq!(part1(input), 288);
    }

    #[test]
    fn exemple_2() {
        let race = Race{
            time: 71530,
            distance: 940200,
        };
        assert_eq!(check_every_posibility_race(&race), 71503);
    }
}
