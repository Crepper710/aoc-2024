use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let pattern = Regex::new("mul\\((\\d{1,3}),(\\d{1,3})\\)").unwrap();
    let mut result = 0;
    for caps in pattern.captures_iter(input) {
        result += caps[1].parse::<u32>().unwrap() * caps[2].parse::<u32>().unwrap();
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let pattern = Regex::new("mul\\((\\d{1,3}),(\\d{1,3})\\)|do(?:n't)?\\(\\)").unwrap();
    let mut result = 0;
    let mut enabled = true;
    for caps in pattern.captures_iter(input) {
        if &caps[0] == "do()" {
            enabled = true;
            continue;
        }
        if &caps[0] == "don't()" {
            enabled = false;
            continue;
        }
        if !enabled {
            continue;
        }
        result += caps[1].parse::<u32>().unwrap() * caps[2].parse::<u32>().unwrap();
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
    }
}
