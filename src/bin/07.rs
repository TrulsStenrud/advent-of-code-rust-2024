advent_of_code::solution!(7);

fn is_valid(result: &u64, numbers: &Vec<u64>) -> bool {
    let numbers = numbers.iter().map(|x| *x).collect::<Vec<u64>>();
    // println!("{:?}", numbers);
    let number = numbers[0];
    let index = 1;

    return helper(number, index, &numbers, *result, false);
}

fn helper(n: u64, index: usize, numbers: &Vec<u64>, cut_off: u64, use_merge: bool) -> bool {
    if n > cut_off {
        return false;
    }
    if numbers.len() == index {
        return n == cut_off;
    }

    return helper(n + numbers[index], index + 1, numbers, cut_off, use_merge)
        || helper(n * numbers[index], index + 1, numbers, cut_off, use_merge)
        || (use_merge
            && helper(
                format!("{}{}", n, numbers[index]).parse().unwrap(),
                index + 1,
                numbers,
                cut_off,
                use_merge,
            ));
}

fn is_valid_2(result: &u64, numbers: &mut Vec<u64>) -> bool {
    let numbers = numbers.iter().map(|x| *x).collect::<Vec<u64>>();
    // println!("{:?}", numbers);
    let number = numbers[0];
    let index = 1;

    return helper(number, index, &numbers, *result, true);
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
