use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut is_bigger: HashMap<String, HashMap<String, bool>> = HashMap::new();

    let mut is_building_rules = true;
    let mut solution = 0;
    for line in input.lines() {
        if line.is_empty() {
            // println!("Now switch");
            is_building_rules = false;
        } else if is_building_rules {
            let mut split = line.split("|");
            let left = split.next().unwrap();
            let right = split.next().unwrap();

            if let Some(existing) = is_bigger.get_mut(left) {
                existing.insert(right.to_string(), false);
            } else {
                let mut new_map = HashMap::new();
                new_map.insert(right.to_string(), false);
                is_bigger.insert(left.to_string(), new_map);
            }
            if let Some(existing) = is_bigger.get_mut(right) {
                existing.insert(left.to_string(), true);
            } else {
                let mut new_map = HashMap::new();
                new_map.insert(left.to_string(), true);
                is_bigger.insert(right.to_string(), new_map);
            }
        } else {
            let original = line.split(",").collect::<Vec<_>>();
            let mut numbers = original.clone();
            // println!("{:?}", original);
            numbers.sort_by(|x, y| {
                // println!(
                //     "Comparing {} and {}, {}",
                //     x,
                //     y,
                //     is_bigger[&x.to_string()][&y.to_string()]
                // );
                if is_bigger[&x.to_string()][&y.to_string()] {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            });
            // println!("{:?}", original);
            // println!("{:?}", numbers);
            if numbers.iter().zip(original).all(|(a, b)| *a == b) {
                // println!("Was same");
                solution += numbers[numbers.len() / 2].parse::<u32>().unwrap();
            }
        }
    }
    Some(solution)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut is_bigger: HashMap<String, HashMap<String, bool>> = HashMap::new();

    let mut is_building_rules = true;
    let mut solution = 0;
    for line in input.lines() {
        if line.is_empty() {
            // println!("Now switch");
            is_building_rules = false;
        } else if is_building_rules {
            let mut split = line.split("|");
            let left = split.next().unwrap();
            let right = split.next().unwrap();

            if let Some(existing) = is_bigger.get_mut(left) {
                existing.insert(right.to_string(), false);
            } else {
                let mut new_map = HashMap::new();
                new_map.insert(right.to_string(), false);
                is_bigger.insert(left.to_string(), new_map);
            }
            if let Some(existing) = is_bigger.get_mut(right) {
                existing.insert(left.to_string(), true);
            } else {
                let mut new_map = HashMap::new();
                new_map.insert(left.to_string(), true);
                is_bigger.insert(right.to_string(), new_map);
            }
        } else {
            let original = line.split(",").collect::<Vec<_>>();
            let mut numbers = original.clone();
            // println!("{:?}", original);
            numbers.sort_by(|x, y| {
                // println!(
                //     "Comparing {} and {}, {}",
                //     x,
                //     y,
                //     is_bigger[&x.to_string()][&y.to_string()]
                // );
                if is_bigger[&x.to_string()][&y.to_string()] {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            });
            // println!("{:?}", original);
            // println!("{:?}", numbers);
            if !numbers.iter().zip(original).all(|(a, b)| *a == b) {
                solution += numbers[numbers.len() / 2].parse::<u32>().unwrap();
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
