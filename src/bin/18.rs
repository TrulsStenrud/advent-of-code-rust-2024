use priority_queue::PriorityQueue;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<i32> {
    part_one_core(input, 70)
}

pub fn part_one_core(input: &str, n: usize) -> Option<i32> {
    let mut corruption = HashSet::with_capacity(input.lines().count());

    input.lines().take(1024).for_each(|line| {
        let mut numbers = line.split(",");
        corruption.insert((
            numbers.next().unwrap().parse::<usize>().unwrap(),
            numbers.next().unwrap().parse::<usize>().unwrap(),
        ));
    });

    let mut used = HashSet::new();
    let mut candidates = PriorityQueue::new();
    candidates.push((0, 0), 0i32);

    let mut prev = HashMap::new();
    let mut last_one = (0, 0);
    while !candidates.is_empty() {
        let ((x, y), cost) = candidates.pop().unwrap();
        last_one = (x, y);
        if x == n && y == n {
            let mut path = HashSet::new();
            let mut curr = (x, y);
            path.insert(curr);
            while prev.contains_key(&curr) {
                curr = prev[&curr];
                path.insert(curr);
            }
            for x in 0..n + 1 {
                for y in 0..n + 1 {
                    if corruption.contains(&(x, y)) {
                        print!("#")
                    } else if path.contains(&(x, y)) {
                        print!("x")
                    } else {
                        print!(".")
                    }
                }
                println!();
            }

            return Some(-cost);
        } else {
            used.insert((x, y));
        }

        if x < n {
            if !corruption.contains(&(x + 1, y)) && !used.contains(&(x + 1, y)) {
                prev.insert((x + 1, y), (x, y));
                candidates.push_increase((x + 1, y), cost - 1);
            }
        }
        if x > 0 {
            if !corruption.contains(&(x - 1, y)) && !used.contains(&(x - 1, y)) {
                prev.insert((x - 1, y), (x, y));
                candidates.push_increase((x - 1, y), cost - 1);
            }
        }
        if y < n {
            if !corruption.contains(&(x, y + 1)) && !used.contains(&(x, y + 1)) {
                prev.insert((x, y + 1), (x, y));
                candidates.push_increase((x, y + 1), cost - 1);
            }
        }
        if y > 0 {
            if !corruption.contains(&(x, y - 1)) && !used.contains(&(x, y - 1)) {
                prev.insert((x, y - 1), (x, y));
                candidates.push_increase((x, y - 1), cost - 1);
            }
        }
    }

    let mut path = HashSet::new();
    let mut curr = last_one;
    path.insert(curr);
    while prev.contains_key(&curr) {
        curr = prev[&curr];
        path.insert(curr);
    }
    for x in 0..n + 1 {
        for y in 0..n + 1 {
            if corruption.contains(&(x, y)) {
                print!("#")
            } else if path.contains(&(x, y)) {
                print!("O")
            } else {
                print!(".")
            }
        }
        println!();
    }
    panic!()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_day_18() {
        let result = part_one_core(&advent_of_code::template::read_file("examples", DAY), 6);
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two_day_18() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
