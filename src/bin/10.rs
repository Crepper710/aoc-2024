use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(10);

fn parse_input(input: &str) -> [HashSet<(usize, usize)>; 10] {
    let mut positions = [0; 10].map(|_| HashSet::new());
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            positions[c as usize - 48].insert((x, y));
        }
    }
    return positions;
}

pub fn part_one(input: &str) -> Option<usize> {
    let postions = parse_input(input);
    let mut trail_heads = HashSet::new();
    let mut queue: VecDeque<(usize, (usize, usize), (usize, usize))> = postions[0].iter().cloned().map(|pos| (0, pos, pos)).collect();
    while let Some((height, (x, y), start)) = queue.pop_front() {
        if height == 9 {
            trail_heads.insert(((x, y), start));
            continue;
        }
        let next = &postions[height + 1];
        if next.contains(&(x, y.wrapping_sub(1))) {
            queue.push_back((height + 1, (x, y.wrapping_sub(1)), start));
        }
        if next.contains(&(x, y.wrapping_add(1))) {
            queue.push_back((height + 1, (x, y.wrapping_add(1)), start));
        }
        if next.contains(&(x.wrapping_sub(1), y)) {
            queue.push_back((height + 1, (x.wrapping_sub(1), y), start));
        }
        if next.contains(&(x.wrapping_add(1), y)) {
            queue.push_back((height + 1, (x.wrapping_add(1), y), start));
        }
    }
    Some(trail_heads.len())
}

pub fn part_two(input: &str) -> Option<u32> {
    let postions = parse_input(input);
    let mut total = 0;
    let mut queue: VecDeque<(usize, (usize, usize))> = postions[0].iter().cloned().map(|pos| (0, pos)).collect();
    while let Some((height, (x, y))) = queue.pop_front() {
        if height == 9 {
            total += 1;
            continue;
        }
        let next = &postions[height + 1];
        if next.contains(&(x, y.wrapping_sub(1))) {
            queue.push_back((height + 1, (x, y.wrapping_sub(1))));
        }
        if next.contains(&(x, y.wrapping_add(1))) {
            queue.push_back((height + 1, (x, y.wrapping_add(1))));
        }
        if next.contains(&(x.wrapping_sub(1), y)) {
            queue.push_back((height + 1, (x.wrapping_sub(1), y)));
        }
        if next.contains(&(x.wrapping_add(1), y)) {
            queue.push_back((height + 1, (x.wrapping_add(1), y)));
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
