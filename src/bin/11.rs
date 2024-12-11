use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, 25)
}
pub fn part_two(input: &str) -> Option<u64> {
    solve(input, 75)
}

fn add_count(hash_map: &mut HashMap<u64, u64>, n: u64, count: u64) {
    if hash_map.contains_key(&n) {
        hash_map.insert(n, hash_map.get(&n).unwrap() + count);
    } else {
        hash_map.insert(n, count);
    }
}
fn solve(input: &str, iterations: u64) -> Option<u64> {
    let mut things = input
        .split_whitespace()
        .map(|it| it.parse::<u64>().unwrap())
        .fold(HashMap::new(), |mut hash_map, n| {
            if hash_map.contains_key(&n) {
                hash_map.insert(n, hash_map.get(&n).unwrap() + 1);
            } else {
                hash_map.insert(n, 1 as u64);
            }
            hash_map
        });

    for _ in 0..iterations {
        things = things.iter().fold(
            HashMap::with_capacity(things.len()),
            |mut hash_map, (stone, count)| {
                if stone == &0 {
                    add_count(&mut hash_map, 1, *count);
                } else if (stone.checked_ilog10().unwrap() + 1) % 2 == 0 {
                    let stone_str = stone.to_string();
                    let (first, second) = stone_str.split_at(stone_str.len() / 2);
                    add_count(&mut hash_map, first.parse::<u64>().unwrap(), *count);
                    add_count(&mut hash_map, second.parse::<u64>().unwrap(), *count);
                } else {
                    add_count(&mut hash_map, stone * 2024, *count);
                }
                hash_map
            },
        );
    }

    Some(things.iter().map(|(_, value)| value).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_day_11() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two_day_11() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
