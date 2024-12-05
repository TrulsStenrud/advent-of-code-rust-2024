use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut is_bigger: HashMap<u32, HashSet<u32>> = HashMap::new();

    let mut is_building_rules = true;
    let mut solution = 0;
    for line in input.lines() {
        if line.is_empty() {
            // println!("Now switch");
            is_building_rules = false;
        } else if is_building_rules {
            let mut split = line.split("|");
            let left = split.next().unwrap().parse::<u32>().unwrap();
            let right = split.next().unwrap().parse::<u32>().unwrap();

            if let Some(existing) = is_bigger.get_mut(&right) {
                existing.insert(left);
            } else {
                let mut new_map = HashSet::new();
                new_map.insert(left);
                is_bigger.insert(right, new_map);
            }
        } else {
            let original = line
                .split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            let mut numbers = original.clone();
            // println!("{:?}", original);
            numbers.sort_by(|x, y| {
                // println!(
                //     "Comparing {} and {}, {}",
                //     x,
                //     y,
                //     is_bigger[&x.to_string()][&y.to_string()]
                // );

                if is_bigger[x].contains(y) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            });
            // println!("{:?}", original);
            // println!("{:?}", numbers);
            if numbers.iter().zip(original).all(|(a, b)| *a == b) {
                solution += numbers[numbers.len() / 2];
            }
        }
    }
    Some(solution)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut is_bigger: HashMap<u32, HashSet<u32>> = HashMap::new();

    let mut is_building_rules = true;
    let mut solution = 0;
    for line in input.lines() {
        if line.is_empty() {
            // println!("Now switch");
            is_building_rules = false;
        } else if is_building_rules {
            let mut split = line.split("|");
            let left = split.next().unwrap().parse::<u32>().unwrap();
            let right = split.next().unwrap().parse::<u32>().unwrap();

            if let Some(existing) = is_bigger.get_mut(&right) {
                existing.insert(left);
            } else {
                let mut new_map = HashSet::new();
                new_map.insert(left);
                is_bigger.insert(right, new_map);
            }
        } else {
            let original = line
                .split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            let mut numbers = original.clone();
            // println!("{:?}", original);
            numbers.sort_by(|x, y| {
                // println!(
                //     "Comparing {} and {}, {}",
                //     x,
                //     y,
                //     is_bigger[&x.to_string()][&y.to_string()]
                // );

                if is_bigger[x].contains(y) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            });
            // println!("{:?}", original);
            // println!("{:?}", numbers);
            if !numbers.iter().zip(original).all(|(a, b)| *a == b) {
                solution += numbers[numbers.len() / 2];
            }
        }
    }
    Some(solution)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_tw1o() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
