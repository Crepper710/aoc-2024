use std::collections::VecDeque;

advent_of_code::solution!(9);

#[inline(always)]
fn sum(start: usize, len: usize) -> usize { // sum start -> start + len
    (((start << 1) + len) * (len + 1)) >> 1
}

fn parse_input(input: &str) -> (VecDeque<(usize, usize)>, VecDeque<usize>) {
    let mut filled = VecDeque::new();
    let mut blank = VecDeque::new();
    for (i, n) in input.trim_end().chars().map(|c| c as usize - 48).enumerate() {
        if i & 1 == 1 {
            blank.push_back(n);
        } else {
            filled.push_back((n, (i / 2) as usize));
        }
    }
    blank.push_back(0);
    return (filled, blank);
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut filled, mut blank) = parse_input(input);
    let mut total = 0;
    let mut current_index = 0;
    loop {
        let fill = if let Some(fill) = filled.pop_front() {fill} else {break};
        total += sum(current_index, fill.0 - 1) * fill.1;
        current_index += fill.0;
        let mut blank = if let Some(blank) = blank.pop_front() {blank} else {break};
        while let Some(&fill) = filled.back() {
            if blank == 0 {break}
            if blank >= fill.0 {
                total += sum(current_index, fill.0 - 1) * fill.1;
                current_index += fill.0;
                filled.pop_back();
                blank -= fill.0;
            } else {
                total += sum(current_index, blank - 1) * fill.1;
                current_index += blank;
                filled.back_mut().unwrap().0 -= blank;
                break;
            }
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (filled, blank) = parse_input(input);
    let mut disk = vec![];
    for i in 0 ..filled.len() {
        disk.push((filled[i].0, Some(filled[i].1)));
        disk.push((blank[i], None));
    }
    disk.push((10, None));
    for i in (0..disk.len()).rev() {
        if let (len, Some(_)) = disk[i] {
            for j in 0..i {
                if let (blank, None) = disk[j] {
                    if len == blank {
                        disk.swap(i, j);
                        break;
                    }
                    if len > blank {continue;}
                    let temp = disk.remove(i);
                    disk.insert(j, temp);
                    disk[j + 1].0 -= len;
                    disk.insert(i + 1, (len, None));
                    break;
                }
            }
        }
    }
    let mut total = 0;
    let mut current_index = 0;
    for (len, id) in disk {
        if let Some(id) = id {
            total += sum(current_index, len - 1) * id;
        }
        current_index += len;
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
