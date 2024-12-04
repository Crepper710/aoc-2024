advent_of_code::solution!(4);

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|s| s.chars().collect()).collect()
}

// 1 1 1  1 2 3  3 2 1
// 2 2 2  2 3      3 2
// 3 3 3  3          3
//
// 1 2 3  3          3
// 1 2 3  2 3      3 2
// 1 2 3  1 2 3  3 2 1

macro_rules! do_check {
    ($c:expr, $buffer:expr, $total:expr) => {
        match ($c, $buffer) {
            ('S', 3) => {
                $buffer = -1;
                $total += 1;
            },
            ('X', -3) => {
                $buffer = 1;
                $total += 1;
            },
            ('A', 2) => $buffer = 3,
            ('M', -2) => $buffer = -3,
            ('M', 1) => $buffer = 2,
            ('A', -1) => $buffer = -2,
            ('X', _) => $buffer = 1,
            ('S', _) => $buffer = -1,
            _ => $buffer = 0,
        }
    };
}

pub fn part_one(input: &str) -> Option<u32> {
    let field = parse_input(input);
    let len = field.len();
    let max_i = len - 1;
    let mut total = 0;
    for i in 0..len {
        let mut buffer = (0, 0);
        for j in 0..len {
            do_check!(field[j][i], buffer.0, total); // vertical
            do_check!(field[i][j], buffer.1, total); // horizontal
        }
        if i < 3 {continue;}
        let mut buffer = (0, 0, 0, 0);
        for j in 0..=i {
            let x = i - j;
            let y = j;
            // print!("{x} {y}   ");
            do_check!(field[y][x], buffer.0, total); // start = top left
            do_check!(field[y][max_i - x], buffer.1, total); // start = top right
            if i == max_i {continue;}
            do_check!(field[max_i - y][x], buffer.2, total); // start = bottom left
            do_check!(field[max_i - y][max_i - x], buffer.3, total); // start = bottom right
        }
        // println!();
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
