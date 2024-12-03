use std::{i8, ops::Sub, u32};

advent_of_code::solution!(2);

fn parse(input: &str) -> impl Iterator<Item = Vec<i8>> + '_ {
    input.lines().map(|s| s.split(' ').map(|s| s.parse::<i8>().unwrap()).collect())
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = parse(input);
    let mut total = 0;
    'outer: for report in reports {
        let diff = report[1] - report[0];
        let bounds = if diff >= -3 && diff <= -1 {
            (-3, -1)
        } else {
            if diff < 1 || diff > 3 {
                continue;
            }
            (1, 3)
        };
        for i in 1..report.len().sub(1) {
            let diff = report[i + 1] - report[i];
            if diff < bounds.0 || diff > bounds.1 {
                continue 'outer;
            }
        }
        total += 1;
    }
    return Some(total);
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = parse(input);
    let mut total = 0;
    for report in reports {
        'outer: for j in 0..report.len() {
            let mut report = report.clone();
            report.remove(j);
            let diff = report[1] - report[0];
            let bounds = if diff >= -3 && diff <= -1 {
                (-3, -1)
            } else {
                if diff < 1 || diff > 3 {
                    continue;
                }
                (1, 3)
            };
            for i in 1..report.len().sub(1) {
                let diff = report[i + 1] - report[i];
                if diff < bounds.0 || diff > bounds.1 {
                    continue 'outer;
                }
            }
            total += 1;
            break;
        }
    }
    return Some(total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
