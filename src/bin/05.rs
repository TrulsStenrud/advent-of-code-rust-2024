use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

fn do_today(input: &str, correct_ones: bool) -> u32 {
    let mut is_bigger: HashMap<u32, HashSet<u32>> = HashMap::new();

    let mut is_building_rules = true;
    let mut solution = 0;
    for line in input.lines() {
        if line.is_empty() {
            is_building_rules = false;
        } else if is_building_rules {
            let mut split = line.split("|").map(|x| x.parse::<u32>().unwrap());
            let left = split.next().unwrap();
            let right = split.next().unwrap();

            if let Some(existing) = is_bigger.get_mut(&right) {
                existing.insert(left);
            } else {
                let mut new_map = HashSet::new();
                new_map.insert(left);
                is_bigger.insert(right, new_map);
            }
        } else {
            let mut numbers = line
                .split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            let mut is_correct = true;
            numbers.sort_by(|x, y| {
                if is_bigger[x].contains(y) {
                    std::cmp::Ordering::Greater
                } else {
                    is_correct = false;
                    std::cmp::Ordering::Less
                }
            });

            if is_correct == correct_ones {
                solution += numbers[numbers.len() / 2];
            }
        }
    }
    solution
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(do_today(input, true))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(do_today(input, false))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_day_5() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two_day_5() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
