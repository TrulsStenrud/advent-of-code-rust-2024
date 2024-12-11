advent_of_code::solution!(7);

fn is_valid(result: &u64, numbers: &Vec<u64>) -> bool {
    let spots_for_operators = numbers.len() as u64 - 1;
    let alternatives = (2 as u64).pow(spots_for_operators as u32);

    for i in 0..alternatives {
        let mut value = numbers[0];
        for n in 0..spots_for_operators {
            let operator = (i >> n) & 1;
            if operator == 0 {
                value += numbers[n as usize + 1];
            } else {
                value *= numbers[n as usize + 1];
            }
        }
        if value == *result {
            return true;
        }
    }

    return false;
}

fn helper(n: u64, numbers: Vec<u64>, results: &mut Vec<u64>) {
    if numbers.len() == 0 {
        results.push(n);
        return;
    }

    let mut numbers = numbers.clone();
    let next = numbers.pop().unwrap();

    // println!("{} {}", n, next);
    helper(n + next, numbers.clone(), results);
    helper(n * next, numbers.clone(), results);
    helper(
        format!("{}{}", n, next).parse().unwrap(),
        numbers.clone(),
        results,
    );
}

fn is_valid_2(result: &u64, numbers: &mut Vec<u64>) -> bool {
    let mut numbers = numbers.iter().rev().map(|x| *x).collect::<Vec<u64>>();
    // println!("{:?}", numbers);
    let first = numbers.pop().unwrap();

    let mut results = Vec::new();

    helper(first, numbers, &mut results);

    if results.contains(result) {
        // println!("    yes");
        return true;
    }
    // println!("    no");
    return false;
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;
    input.lines().for_each(|line| {
        let mut words = line.split(':');

        let left = words.next().unwrap().parse::<u64>().unwrap();

        let right = words
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        if is_valid(&left, &right) {
            sum += left;
        }
    });
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;
    input.lines().for_each(|line| {
        let mut words = line.split(':');

        let left = words.next().unwrap().parse::<u64>().unwrap();

        let mut right = words
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        if is_valid_2(&left, &mut right) {
            sum += left;
        }
    });
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_day_day_7() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two_day_day_7() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
