use std::{str::FromStr, usize};

use anyhow::{Context, Error, Result};
use itertools::Itertools;

advent_of_code::solution!(14);

struct Robot {
    initial_pos: (isize, isize),
    velocity: (isize, isize),
}

impl Robot {
    fn step(&self, amount: isize, bounds: (isize, isize)) -> (isize, isize) {
        return (
            (self.initial_pos.0 + self.velocity.0 * amount).rem_euclid(bounds.0),
            (self.initial_pos.1 + self.velocity.1 * amount).rem_euclid(bounds.1),
        )
    }
}

impl FromStr for Robot {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (initial_pos, velocity) = s[2..].split_once(' ').context("failed to split initial_pos and velocity")?;
        let initial_pos = initial_pos.split_once(',').context("failed to split initial_pos")?;
        let velocity = velocity[2..].split_once(',').context("failed to split velocity")?;
        //println!("{initial_pos:?} {velocity:?}");
        Ok(Self {
            initial_pos: (initial_pos.0.parse::<isize>()?, initial_pos.1.parse::<isize>()?),
            velocity: (velocity.0.parse::<isize>()?, velocity.1.parse::<isize>()?),
        })
    }
}

fn parse_input(input: &str) -> Box<[Robot]> {
    input.lines().map(|s| s.parse::<Robot>().unwrap()).collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let robots = parse_input(input);
    let max_x = robots.iter().map(|r| r.initial_pos.0).max().unwrap() + 1; // should be 11 or 101
    let max_y = robots.iter().map(|r| r.initial_pos.1).max().unwrap() + 1; // should be 7 or 103
    let middle_x = max_x / 2;
    let middle_y = max_y / 2;
    let quadrants = robots
        .iter()
        .map(|r| r.step(100, (max_x, max_y))) // do steps
        .filter_map(|(x, y)| if x != middle_x && y != middle_y {
            Some((((x < middle_x) as usize) << 1) | (y < middle_y) as usize)

        } else {
            None
        })
        .counts();
    Some(quadrants.values().product())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
