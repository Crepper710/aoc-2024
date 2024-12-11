use std::collections::HashMap;

use ilog::IntLog;

advent_of_code::solution!(11);

type Num = u128;

fn recur_count_blinks(num: Num, steps: usize, cache: &mut HashMap<(Num, usize), Num>) -> Num {
    if steps == 0 {
        return 1;
    }
    if let Some(result) = cache.get(&(num, steps)) {
        return *result;
    }
    let result = if num == 0 {
        recur_count_blinks(1, steps - 1, cache)
    } else {
        let log = num.log10() as u32 + 1;
        if log & 1 == 0 {
            let split = (10 as Num).pow(log / 2);
            recur_count_blinks(num / split, steps - 1, cache) + recur_count_blinks(num % split, steps - 1, cache)
        } else {
            recur_count_blinks(num * 2024, steps - 1, cache)
        }
    };
    cache.insert((num, steps), result);
    return result;
}

fn parse_input(input: &str) -> Vec<Num> {
    input.trim().split(' ').map(|s| s.parse::<Num>().unwrap()).collect()
}

pub fn part_one(input: &str) -> Option<Num> {
    let stones = parse_input(input);
    let mut cache = HashMap::new();
    let mut total = 0;
    for stone in stones {
        total += recur_count_blinks(stone, 25, &mut cache);
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<Num> {
    let stones = parse_input(input);
    let mut cache = HashMap::new();
    let mut total = 0;
    for stone in stones {
        total += recur_count_blinks(stone, 75, &mut cache);
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
