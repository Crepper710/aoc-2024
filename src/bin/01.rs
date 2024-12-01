advent_of_code::solution!(1);

fn split_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    input.lines().map(|s| s.split_once("   ").unwrap()).map(|(l, r)| (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap())).collect::<(Vec<_>, Vec<_>)>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = split_lists(input);
    left.sort();
    right.sort();
    Some(left.into_iter().zip(right.into_iter()).map(|(l, r)| l.abs_diff(r)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = split_lists(input);
    Some(left.into_iter().map(|l| l * right.iter().filter(|r| r == &&l).count() as u32).sum())
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
