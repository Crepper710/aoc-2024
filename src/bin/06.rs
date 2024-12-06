use std::{collections::HashSet, usize};

advent_of_code::solution!(6);

fn parse_input(input: &str) -> (Vec<Vec<bool>>, (isize, isize)) {
    let mut vec = vec![];
    let mut pos = (0, 0);
    for (y, line) in input.lines().enumerate() {
        let mut sub = Vec::with_capacity(line.len());
        for (x, c) in line.chars().enumerate() {
            sub.push(c == '#');
            if c == '^' {
                pos = (x as isize, y as isize);
            }
        }
        vec.push(sub);
    }
    (vec, pos)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (field, (mut x, mut y)) = parse_input(input);
    let mut direction = 0;
    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut visited = field.iter().map(|v| v.into_iter().map(|_| false).collect::<Vec<_>>()).collect::<Vec<_>>();
    loop {
        visited[y as usize][x as usize] = true;
        let next_x = x + directions[direction].0;
        let next_y = y + directions[direction].1;
        if let Some(Some(b)) = field.get(next_y as usize).map(|v| v.get(next_x as usize)) {
            if *b {
                direction = (direction + 1) % 4;
            } else {
                x = next_x;
                y = next_y;
            }
        } else {
            break;
        }
    }
    Some(visited.into_iter().map(|v| v.into_iter().filter(|b| *b).count() as u32).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (field, (init_x, init_y)) = parse_input(input);
    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let init_visited = {
        let temp = field.iter().map(|v| v.into_iter().map(|_| false).collect::<Vec<_>>()).collect::<Vec<_>>();
        [temp.clone(), temp.clone(), temp.clone(), temp]
    };
    let to_visit = {
        let mut x = init_x;
        let mut y = init_y;
        let mut direction = 0;
        let mut visited = HashSet::new();
        loop {
            visited.insert((y as usize, x as usize));
            let next_x = x + directions[direction].0;
            let next_y = y + directions[direction].1;
            if let Some(Some(b)) = field.get(next_y as usize).map(|v| v.get(next_x as usize)) {
                if *b {
                    direction = (direction + 1) % 4;
                } else {
                    x = next_x;
                    y = next_y;
                }
            } else {
                break;
            }
        }
        visited
    };
    let mut total = 0;
    for (blocked_y, blocked_x) in to_visit.into_iter() {
        let mut x = init_x;
        let mut y = init_y;
        let mut direction = 0;
        let mut visited = init_visited.clone();
        loop {
            if visited[direction][y as usize][x as usize] { // found loop
                total += 1;
                break;
            }
            visited[direction][y as usize][x as usize] = true;
            let next_x = x + directions[direction].0;
            let next_y = y + directions[direction].1;
            if let Some(Some(b)) = field.get(next_y as usize).map(|v| v.get(next_x as usize)) {
                if *b || (blocked_y == next_y as usize && blocked_x == next_x as usize) {
                    direction = (direction + 1) % 4;
                } else {
                    x = next_x;
                    y = next_y;
                }
            } else {
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
