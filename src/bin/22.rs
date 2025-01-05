use std::ops::BitXor;

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;

    for line in input.lines() {
        let mut n = line.parse::<u64>().unwrap();
        for _ in 0..2000 {
            let result = n * 64;
            n = result.bitxor(n) % 16777216;
            let result = n / 32;
            n = result.bitxor(n) % 16777216;
            let result = n * 2048;
            n = result.bitxor(n) % 16777216;
        }

        sum += n;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_day_22() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two_day_22() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
