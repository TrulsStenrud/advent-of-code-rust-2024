advent_of_code::solution!(2);

fn is_safe(line: &str) -> bool {
    let mut numbers = line.split_whitespace();
    let mut prev = numbers
        .next()
        .expect("Not enough numbers in line.")
        .parse::<u32>()
        .expect("Failed to parse u32 from string.");
    let mut increasing: Option<bool> = None;
    for next in numbers.map(|it| it.parse::<u32>().expect("Failed to parse u32 from string.")) {
        if (prev == next)
            || ((prev < next) != *increasing.get_or_insert(prev < next))
            || prev.abs_diff(next) > 3
        {
            return false;
        }
        prev = next;
    }

    true
}

fn is_safe_n(numbers: &Vec<u32>, skip_i: usize) -> bool {
    let mut index = 0;
    if skip_i == index {
        index += 1;
    }
    let mut prev = numbers[index];
    index += 1;
    if skip_i == index {
        index += 1;
    }
    let second = numbers[index];
    index += 1;
    let increasing = prev < second;
    if prev == second || prev.abs_diff(second) > 3 {
        return false;
    }
    prev = second;
    for c in index..numbers.len() {
        if c == skip_i {
            continue;
        }
        let next = numbers[c];

        if (prev == next) || ((prev < next) != increasing) || prev.abs_diff(next) > 3 {
            return false;
        }
        prev = next;
    }

    true
}
fn is_safe_ish(line: &str) -> bool {
    let mut numbers = line.split_whitespace();
    let mut prev = numbers
        .next()
        .expect("Not enough numbers in line.")
        .parse::<u32>()
        .expect("Failed to parse u32 from string.");

    let second = numbers
        .next()
        .expect("Not enough numbers in line.")
        .parse::<u32>()
        .expect("Failed to parse u32 from string.");

    let mut has_false = false;

    let increasing = prev < second;
    if prev == second || prev.abs_diff(second) > 3 {
        has_false = true;
    } else {
        prev = second;
    }
    for curr in numbers {
        let next = curr
            .parse::<u32>()
            .expect("Failed to parse u32 from string.");

        if (prev == next) || ((prev < next) != increasing) || prev.abs_diff(next) > 3 {
            if has_false {
                return false;
            }
            has_false = true;
        } else {
            prev = next;
        }
    }

    true
}

fn are_you_sure(line: &str) -> bool {
    let numbers = line
        .split_whitespace()
        .map(|n| n.parse::<u32>().expect("Unable to parse u32"))
        .collect::<Vec<_>>();

    for i in 0..numbers.len() {
        if is_safe_n(&numbers, i) {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().filter(|line| is_safe(line)).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|line| is_safe_ish(line) || are_you_sure(line))
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_tw1() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
