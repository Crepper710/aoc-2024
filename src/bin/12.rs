use std::collections::HashSet;

advent_of_code::solution!(12);

fn parse_input(input: &str) -> Box<[Box<[char]>]> {
    input.lines().map(|s| s.chars().collect()).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let field = parse_input(input);
    let mut to_check = (0..field.len()).map(|y| (0..field.len()).map(move |x| (x, y))).flatten().collect::<HashSet<_>>();
    let mut total = 0;
    while let Some(&(x, y)) = to_check.iter().next() {
        to_check.remove(&(x, y));
        let mut stack = vec![(x, y)];
        let mut fences = 0;
        let mut count = 0;
        while let Some((x, y)) = stack.pop() {
            count += 1;

            let x_add = x.wrapping_add(1);
            let x_sub = x.wrapping_sub(1);
            let y_add = y.wrapping_add(1);
            let y_sub = y.wrapping_sub(1);

            let current = field.get(y).map(|v| v.get(x));
            if current != field.get(y_sub).map(|v| v.get(x)) {
                fences += 1;
            } else {
                if to_check.remove(&(x, y_sub)) {
                    stack.push((x, y_sub));
                }
            }
            if current != field.get(y).map(|v| v.get(x_add)) {
                fences += 1;
            } else {
                if to_check.remove(&(x_add, y)) {
                    stack.push((x_add, y));
                }
            }
            if current != field.get(y_add).map(|v| v.get(x)) {
                fences += 1;
            } else {
                if to_check.remove(&(x, y_add)) {
                    stack.push((x, y_add));
                }
            }
            if current != field.get(y).map(|v| v.get(x_sub)) {
                fences += 1;
            } else {
                if to_check.remove(&(x_sub, y)) {
                    stack.push((x_sub, y));
                }
            }
        }
        total += count * fences;
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let field = parse_input(input);
    let mut to_check = (0..field.len()).map(|y| (0..field.len()).map(move |x| (x, y))).flatten().collect::<HashSet<_>>();
    let mut total = 0;
    while let Some(&(x, y)) = to_check.iter().next() {
        to_check.remove(&(x, y));
        let mut stack = vec![(x, y)];
        let mut edges = 0;
        let mut count = 0;
        while let Some((x, y)) = stack.pop() {
            count += 1;

            let x_add = x.wrapping_add(1);
            let x_sub = x.wrapping_sub(1);
            let y_add = y.wrapping_add(1);
            let y_sub = y.wrapping_sub(1);

            let current = field.get(y).map(|v| v.get(x));

            let top_same = current == field.get(y_sub).map(|v| v.get(x));
            let right_same = current == field.get(y).map(|v| v.get(x_add));
            let bottom_same = current == field.get(y_add).map(|v| v.get(x));
            let left_same = current == field.get(y).map(|v| v.get(x_sub));

            if top_same {
                if to_check.remove(&(x, y_sub)) {
                    stack.push((x, y_sub));
                }
            }
            if right_same {
                if to_check.remove(&(x_add, y)) {
                    stack.push((x_add, y));
                }
            }
            if bottom_same {
                if to_check.remove(&(x, y_add)) {
                    stack.push((x, y_add));
                }
            }
            if left_same {
                if to_check.remove(&(x_sub, y)) {
                    stack.push((x_sub, y));
                }
            }

            // edge detection
            // number of edges == number of straight lines
            if (!top_same && !right_same) || (top_same && right_same && current != field.get(y_sub).map(|v| v.get(x_add))) {
                edges += 1;
            }
            if (!right_same && !bottom_same) || (right_same && bottom_same && current != field.get(y_add).map(|v| v.get(x_add)))  {
                edges += 1;
            }
            if (!bottom_same && !left_same) || (bottom_same && left_same && current != field.get(y_add).map(|v| v.get(x_sub))) {
                edges += 1;
            }
            if (!left_same && !top_same) || (left_same && top_same && current != field.get(y_sub).map(|v| v.get(x_sub))) {
                edges += 1;
            }
        }
        total += count * edges;
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
