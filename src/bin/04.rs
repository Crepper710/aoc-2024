advent_of_code::solution!(4);

fn parse_input(input: &str) -> Box<[Box<[char]>]> {
    input.lines().map(|s| s.chars().collect()).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let field = parse_input(input);
    let len = field.len();
    let mut total = 0;
    let len_ = len - 4;
    for y in 0..len {
        for x in 0..len {
            if field[y][x] != 'X' {continue;}
            'block: { // right
                if x > len_ {break 'block;}
                if field[y][x + 1] != 'M' {break 'block;}
                if field[y][x + 2] != 'A' {break 'block;}
                if field[y][x + 3] != 'S' {break 'block;}
                total += 1;
            }
            'block: { // left
                if x < 3 {break 'block;}
                if field[y][x - 1] != 'M' {break 'block;}
                if field[y][x - 2] != 'A' {break 'block;}
                if field[y][x - 3] != 'S' {break 'block;}
                total += 1;
            }
            'block: { // bottom
                if y > len_ {break 'block;}
                if field[y + 1][x] != 'M' {break 'block;}
                if field[y + 2][x] != 'A' {break 'block;}
                if field[y + 3][x] != 'S' {break 'block;}
                total += 1;
            }
            'block: { // right
                if y < 3 {break 'block;}
                if field[y - 1][x] != 'M' {break 'block;}
                if field[y - 2][x] != 'A' {break 'block;}
                if field[y - 3][x] != 'S' {break 'block;}
                total += 1;
            }
            'block: { // bottom right
                if x > len_ || y > len_ {break 'block;}
                if field[y + 1][x + 1] != 'M' {break 'block;}
                if field[y + 2][x + 2] != 'A' {break 'block;}
                if field[y + 3][x + 3] != 'S' {break 'block;}
                total += 1;
            }
            'block: { // top right
                if x > len_ || y < 3 {break 'block;}
                if field[y - 1][x + 1] != 'M' {break 'block;}
                if field[y - 2][x + 2] != 'A' {break 'block;}
                if field[y - 3][x + 3] != 'S' {break 'block;}
                total += 1;
            }
            'block: { // bottom left
                if x < 3 || y > len_ {break 'block;}
                if field[y + 1][x - 1] != 'M' {break 'block;}
                if field[y + 2][x - 2] != 'A' {break 'block;}
                if field[y + 3][x - 3] != 'S' {break 'block;}
                total += 1;
            }
            'block: { // top right
                if x < 3 || y < 3 {break 'block;}
                if field[y - 1][x - 1] != 'M' {break 'block;}
                if field[y - 2][x - 2] != 'A' {break 'block;}
                if field[y - 3][x - 3] != 'S' {break 'block;}
                total += 1;
            }
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let field = parse_input(input);
    let len = field.len();
    let max = len - 1;
    let mut total = 0;
    for y in 1..max {
        for x in 1..max {
            if field[y][x] != 'A' {continue;}
            let mut m = 0;
            let mut s = 0;
            let c = (
                field[y - 1][x - 1],
                field[y - 1][x + 1],
                field[y + 1][x - 1],
                field[y + 1][x + 1],
            );
            match c.0 {
                'M' => m += 1,
                'S' => s += 1,
                _ => continue
            }
            match c.1 {
                'M' => m += 1,
                'S' => s += 1,
                _ => continue
            }
            match c.2 {
                'M' => m += 1,
                'S' => s += 1,
                _ => continue
            }
            match c.3 {
                'M' => m += 1,
                'S' => s += 1,
                _ => continue
            }
            if m == s && c.0 != c.3 && c.1 != c.2 {
                total += 1;
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
