use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(8);

fn parse_input(input: &str) -> (HashMap<char, Vec<(usize, usize)>>, usize) {
    let mut map = HashMap::new();
    let mut len = 0;
    for (y, line) in input.lines().enumerate() {
        len = line.len();
        for (x, c) in line.chars().enumerate() {
            if c == '.' {continue;}
            map.entry(c).and_modify(|v: &mut Vec<_>| v.push((x, y))).or_insert(vec![(x, y)]);
        }
    }
    return (map, len);
}

pub fn part_one(input: &str) -> Option<usize> {
    let (antennas, len) = parse_input(input);
    let mut antinodes = HashSet::new();
    for locations in antennas.clone().values() {
        for pair in locations.into_iter().permutations(2) {
            antinodes.insert(((pair[0].0 * 2).checked_sub(pair[1].0), (pair[0].1 * 2).checked_sub(pair[1].1)));
        }
    }
    Some(antinodes.into_iter().filter(|pos| if let (Some(x), Some(y)) = pos {*x < len && *y < len} else {false}).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (antennas, len) = parse_input(input);
    let mut antinodes = HashSet::new();
    for locations in antennas.clone().values() {
        for pair in locations.into_iter().permutations(2) {
            let (x_1, y_1) = pair[0];
            let (x_2, y_2) = pair[1];
            let x_step = x_1.wrapping_sub(*x_2);
            let y_step = y_1.wrapping_sub(*y_2);
            let mut x = *x_1;
            let mut y = *y_1;
            while x < len && y < len {
                antinodes.insert((x, y));
                x = x.wrapping_add(x_step);
                y = y.wrapping_add(y_step);
            }
        }
    }
    Some(antinodes.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
