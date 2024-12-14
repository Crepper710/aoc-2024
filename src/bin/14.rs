use core::f64;
use std::{str::FromStr, usize};

use anyhow::{Context, Error, Result};
use itertools::Itertools;

advent_of_code::solution!(14);

struct Robot {
    pos: (isize, isize),
    velocity: (isize, isize),
}

impl Robot {
    fn step(&self, amount: isize, bounds: (isize, isize)) -> (isize, isize) {
        return (
            (self.pos.0 + self.velocity.0 * amount).rem_euclid(bounds.0),
            (self.pos.1 + self.velocity.1 * amount).rem_euclid(bounds.1),
        )
    }

    fn step_once_mut(&mut self, bounds: (isize, isize)) {
        self.pos.0 = (self.pos.0 + self.velocity.0).rem_euclid(bounds.0);
        self.pos.1 = (self.pos.1 + self.velocity.1).rem_euclid(bounds.1);
    }
}

impl FromStr for Robot {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (initial_pos, velocity) = s[2..].split_once(' ').context("failed to split initial_pos and velocity")?;
        let initial_pos = initial_pos.split_once(',').context("failed to split initial_pos")?;
        let velocity = velocity[2..].split_once(',').context("failed to split velocity")?;
        Ok(Self {
            pos: (initial_pos.0.parse::<isize>()?, initial_pos.1.parse::<isize>()?),
            velocity: (velocity.0.parse::<isize>()?, velocity.1.parse::<isize>()?),
        })
    }
}

fn parse_input(input: &str) -> Box<[Robot]> {
    input.lines().map(|s| s.parse::<Robot>().unwrap()).collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let robots = parse_input(input);
    let max_x = robots.iter().map(|r| r.pos.0).max().unwrap() + 1; // should be 11 or 101
    let max_y = robots.iter().map(|r| r.pos.1).max().unwrap() + 1; // should be 7 or 103
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

fn variance(nums: Box<[isize]>) -> f64 {
    let mean = nums.iter().sum::<isize>() as f64 / nums.len() as f64;
    IntoIterator::into_iter(nums).map(|x| (x as f64 - mean).powi(2)).sum()
}

pub fn part_two(input: &str) -> Option<isize> {
    let mut robots = parse_input(input);
    let mut min_x_variance = f64::MAX;
    let mut best_x = 0isize;
    let mut min_y_variance = f64::MAX;
    let mut best_y = 0;
    for i in 0..103 {
        robots.iter_mut().for_each(|r| r.step_once_mut((101, 103)));
        let x_variance = variance(robots.iter().map(|r| r.pos.0).collect());
        let y_variance = variance(robots.iter().map(|r| r.pos.1).collect());
        //println!("{i} {} {}", x_variance, y_variance);
        if x_variance < min_x_variance {
            min_x_variance = x_variance;
            best_x = i;
        }
        if y_variance < min_y_variance {
            min_y_variance = y_variance;
            best_y = i;
        }
    }
    //println!("{x}: {max_x_variance} {y}: {max_y_variance}");
    Some((best_x + 51 * (best_y - best_x) * 101).rem_euclid(101 * 103) + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }
}
