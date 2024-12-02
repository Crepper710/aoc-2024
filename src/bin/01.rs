use std::{collections::HashMap, ops::{Add, Div, Sub}};

advent_of_code::solution!(1);

fn split_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    input.lines().map(|s| (
            s[0..s.len().div(2).sub(1)].parse::<u32>().unwrap(),
            s[s.len().div(2).add(2)..s.len()].parse::<u32>().unwrap()
    )).collect::<(Vec<_>, Vec<_>)>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = split_lists(input);
    left.sort_unstable();
    right.sort_unstable();
    Some(left.into_iter().zip(right.into_iter()).map(|(l, r)| l.abs_diff(r)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = split_lists(input);
    let mut map = HashMap::new();
    for r in right {
        *map.entry(r).or_insert(0) += 1;
    }
    Some(left.into_iter().map(|l| l * map.get(&l).unwrap_or(&0)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
