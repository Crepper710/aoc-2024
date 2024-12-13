advent_of_code::solution!(13);

struct ClawMachine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

const SHIFT: i64 = 10000000000000;

impl ClawMachine {
    fn solve(&self, offset: i64) -> Option<i64> {
        let prize = (self.prize.0 + offset, self.prize.1 + offset);
        let divisor = self.button_a.0 * self.button_b.1 - self.button_a.1 * self.button_b.0;
        if divisor == 0 {
            return None;
        }
        let dividend = prize.0 * self.button_b.1 - prize.1 * self.button_b.0;
        if dividend % divisor != 0 || self.button_b.0 == 0 {
            return None;
        }
        let a_presses = dividend / divisor;
        let rest = prize.0 - self.button_a.0 * a_presses;
        if rest % self.button_b.0 != 0 { // stupid edge case
            return None;
        }
        return Some(a_presses * 3 + (rest / self.button_b.0));
    }
}

#[inline(always)]
fn parse_unsafe(s: &str) -> i64 {
    s.chars().map(|c| c as i64 - 48).reduce(|a, b| a * 10 + b).unwrap()
}

fn parse_input(input: &str) -> Box<[ClawMachine]> {
    input.trim_end().split("\n\n").map(|s| {
        let button_a_x= &s[12..14];
        let button_a_y = &s[18..20];
        let button_b_x = &s[33..35];
        let button_b_y = &s[39..41];
        let target = &s[51..s.len()];
        let (prize_x, prize_y) = target.split_once(", Y=").unwrap();
        ClawMachine {
            button_a: (parse_unsafe(button_a_x), parse_unsafe(button_a_y)),
            button_b: (parse_unsafe(button_b_x), parse_unsafe(button_b_y)),
            prize: (parse_unsafe(prize_x), parse_unsafe(prize_y)),
        }
    }).collect()
}

pub fn part_one(input: &str) -> Option<i64> {
    let claw_machines = parse_input(input);
    Some(claw_machines.iter().filter_map(|c| c.solve(0)).sum())
}

pub fn part_two(input: &str) -> Option<i64> {
    let claw_machines = parse_input(input);
    Some(claw_machines.iter().filter_map(|c| c.solve(SHIFT)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
