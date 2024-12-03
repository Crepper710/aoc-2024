advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut iter = input.chars();
    let mut result = 0;
    'outer: while let Some(c) = iter.next() {
        if c != 'm' {continue;}
        if iter.next() != Some('u') {continue;}
        if iter.next() != Some('l') {continue;}
        if iter.next() != Some('(') {continue;}
        let mut counter = 0;
        let mut num = 0;
        while counter < 4 {
            if let Some(c) = iter.next() {
                if c == ',' && counter != 0 {
                    break;
                }
                counter += 1;
                if !c.is_digit(10) {
                    continue 'outer;
                }
                num = num * 10 + (c as u32 - '0' as u32);
            } else {
                break 'outer;
            }
        }
        counter = 0;
        let mut num2 = 0;
        while counter < 4 {
            if let Some(c) = iter.next() {
                if c == ')' && counter != 0 {
                    break;
                }
                counter += 1;
                if !c.is_digit(10) {
                    continue 'outer;
                }
                num2 = num2 * 10 + (c as u32 - '0' as u32);
            } else {
                break 'outer;
            }
        }
        result += num * num2;
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut iter = input.chars().peekable();
    let mut result = 0;
    let mut enabled = true;
    'outer: while let Some(c) = iter.next() {
        if c == 'd' {
            if iter.peek() != Some(&'o') {continue;}
            iter.next();
            match iter.peek() {
                Some(&'(') => {
                    iter.next();
                    if iter.peek() != Some(&')') {continue;}
                    iter.next();
                    enabled = true;
                }
                Some(&'n') => {
                    iter.next();
                    if iter.peek() != Some(&'\'') {continue;}
                    iter.next();
                    if iter.peek() != Some(&'t') {continue;}
                    iter.next();
                    if iter.peek() != Some(&'(') {continue;}
                    iter.next();
                    if iter.peek() != Some(&')') {continue;}
                    iter.next();
                    enabled = false;
                }
                _ => continue
            }
        }
        if !enabled {continue;}
        if c != 'm' {continue;}
        if iter.peek() != Some(&'u') {continue;}
        iter.next();
        if iter.peek() != Some(&'l') {continue;}
        iter.next();
        if iter.peek() != Some(&'(') {continue;}
        iter.next();
        let mut counter = 0;
        let mut num = 0;
        while counter < 4 {
            if let Some(&c) = iter.peek() {
                if c == ',' && counter != 0 {
                    iter.next();
                    break;
                }
                counter += 1;
                if !c.is_digit(10) {
                    continue 'outer;
                }
                iter.next();
                num = num * 10 + (c as u32 - '0' as u32);
            } else {
                break 'outer;
            }
        }
        counter = 0;
        let mut num2 = 0;
        while counter < 4 {
            if let Some(&c) = iter.peek() {
                if c == ')' && counter != 0 {
                    iter.next();
                    break;
                }
                counter += 1;
                if !c.is_digit(10) {
                    continue 'outer;
                }
                iter.next();
                num2 = num2 * 10 + (c as u32 - '0' as u32);
            } else {
                break 'outer;
            }
        }
        result += num * num2;
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
