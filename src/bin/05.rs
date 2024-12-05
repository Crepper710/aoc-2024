use std::{cmp::Ordering, collections::HashSet};

advent_of_code::solution!(5);

fn parse_input(input: &str) -> (HashSet<(u32, u32)>, Vec<Vec<u32>>) {
    let mut lines = input.lines().map(str::as_bytes);
    let mut set = HashSet::new();
    while let Some(line) = lines.next() {
        if line.len() == 0 {break;}
        // 528 == '0' as u32 * 11
        set.insert((line[0] as u32 * 10 + line[1] as u32 - 528, line[3] as u32 * 10 + line[4] as u32 - 528));
    }
    let mut vec = vec![];
    for line in lines {
        let len = line.len() / 3 + 1;
        let mut sub = Vec::with_capacity(len);
        for i in 0..len {
            // 528 == '0' as u32 * 11
            sub.push(line[i * 3] as u32 * 10 + line[i * 3 + 1] as u32 - 528)
        }
        vec.push(sub)
    }
    (set, vec)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input);
    let mut total = 0;
    for update in updates {
        if update.is_sorted_by(|a, b| rules.contains(&(*a, *b))) {
            total += update[update.len() / 2]
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input);
    let mut total = 0;
    for mut update in updates {
        if update.is_sorted_by(|a, b| rules.contains(&(*a, *b))) {
            continue;
        }
        update.sort_by(|a, b| if rules.contains(&(*a, *b)) {Ordering::Less} else {Ordering::Greater});
        total += update[update.len() / 2];
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
