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

fn is_safe_ish(
    line: &Vec<u32>,
    i: usize,
    skip: Option<usize>,
    increasing: &mut Option<bool>,
) -> bool {
    if i == line.len() {
        return true;
    }

    let mut prev = line[i - 1];
    if let Some(skip) = skip {
        if skip == i {
            return is_safe_ish(line, i + 1, Some(skip), increasing);
        } else if skip == i - 1 {
            if i == 1 {
                return is_safe_ish(line, i + 1, Some(skip), increasing);
            } else {
                prev = line[i - 2];
            }
        }
    }
    let curr = line[i];

    if (prev == curr)
        || ((prev < curr) != *increasing.get_or_insert(prev < curr))
        || prev.abs_diff(curr) > 3
    {
        if skip.is_none() {
            return is_safe_ish(line, i + 1, Some(i), &mut None)
                || is_safe_ish(line, i, Some(i - 1), &mut None);
        } else {
            return false;
        }
    }

    return is_safe_ish(line, i + 1, skip, increasing);
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().filter(|line| is_safe(line)).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|line| {
                is_safe_ish(
                    &line
                        .split_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect(),
                    1,
                    None,
                    &mut None,
                )
            })
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
