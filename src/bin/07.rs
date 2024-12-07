use std::collections::HashSet;

advent_of_code::solution!(7);

fn parse_input(input: &str) -> Vec<(u64, Vec<(u64, u64)>)> {
    let mut map = vec![];
    for line in input.lines() {
        let mut iter = line.chars();
        let mut vec = vec![];
        let mut key = 0;
        while let Some(c) = iter.next() {
            if c == ':' {break;}
            key = key * 10 + c as u64 - 48; // '0' as u32 == 48
        }
        iter.next();
        while let Some(c) = iter.next() {
            let mut value = c as u64 - 48;
            let mut digits = 10;
            while let Some(c) = iter.next() {
                if c == ' ' {break;}
                value = value * 10 + c as u64 - 48;
                digits *= 10;
            }
            vec.push((value, digits));
        }
        map.push((key, vec));
    }
    return map;
}

pub fn part_one(input: &str) -> Option<u64> {
    let entries = parse_input(input);
    let mut total = 0;
    for (expected, values) in entries {
        for i in 0..(1 << (values.len() - 1)) {
            let mut actual = values[0].0;
            for j in 1..values.len() {
                if ((i >> (j - 1)) & 1) == 1 {
                    actual *= values[j].0;
                } else {
                    actual += values[j].0;
                }
            }
            if actual == expected {
                total += expected;
                break;
            }
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let entries = parse_input(input);
    let mut total = 0;
    let mut ignore = HashSet::new();
    for (k, (expected, values)) in entries.clone().into_iter().enumerate() {
        for i in 0..(1 << (values.len() - 1)) {
            let mut actual = values[0].0;
            for j in 1..values.len() {
                if ((i >> (j - 1)) & 1) == 1 {
                    actual *= values[j].0;
                } else {
                    actual += values[j].0;
                }
            }
            if actual == expected {
                total += expected;
                ignore.insert(k);
                break;
            }
        }
    }
    for (i, (expected, values)) in entries.into_iter().enumerate() {
        if ignore.contains(&i) {continue;}
        for i in 0..3usize.pow(values.len() as u32 - 1) {
            let mut actual = values[0].0;
            for j in 1..values.len() {
                match (i / (3usize.pow(j as u32 - 1))) % 3 {
                    0 => {
                        actual = if let Some(next) = actual.checked_add(values[j].0) {
                            next
                        } else {
                            continue;
                        }
                    },
                    1 => {
                        actual = if let Some(next) = actual.checked_mul(values[j].0) {
                            next
                        } else {
                            continue;
                        }
                    },
                    2 => {
                        actual = if let Some(Some(next)) = actual.checked_mul(values[j].1).map(|i| i.checked_add(values[j].0)) {
                            next
                        } else {
                            continue;
                        }
                    },
                    _ => unreachable!()
                }
                if actual > expected {
                    break;
                }
            }
            if actual == expected {
                total += expected;
                break;
            }
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
