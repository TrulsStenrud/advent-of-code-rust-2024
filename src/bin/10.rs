use std::collections::HashSet;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let numbers = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let sum = numbers
        .iter()
        .enumerate()
        .map(|(x, line)| {
            line.iter()
                .enumerate()
                .map(|(y, number)| {
                    if *number == 0 {
                        let a = find_trails(&numbers, x, y);

                        a.len() as u32
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        })
        .sum::<u32>();
    Some(sum)
}

fn find_trails(numbers: &Vec<Vec<u32>>, x: usize, y: usize) -> HashSet<(usize, usize)> {
    if numbers[x][y] == 9 {
        // println!("Fund 9 at {} {}", x, y);
        let mut result = HashSet::new();
        result.insert((x, y));
        return result;
    }
    let mut sum = HashSet::new();
    if x > 0 && numbers[x - 1][y] == numbers[x][y] + 1 {
        find_trails(numbers, x - 1, y).iter().for_each(|(a, b)| {
            sum.insert((*a, *b));
        });
    }
    if x < numbers.len() - 1 && numbers[x + 1][y] == numbers[x][y] + 1 {
        find_trails(numbers, x + 1, y).iter().for_each(|(a, b)| {
            sum.insert((*a, *b));
        });
    }
    if y > 0 && numbers[x][y - 1] == numbers[x][y] + 1 {
        find_trails(numbers, x, y - 1).iter().for_each(|(a, b)| {
            sum.insert((*a, *b));
        });
    }
    if y < numbers[x].len() - 1 && numbers[x][y + 1] == numbers[x][y] + 1 {
        find_trails(numbers, x, y + 1).iter().for_each(|(a, b)| {
            sum.insert((*a, *b));
        });
    }
    sum
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let sum = numbers
        .iter()
        .enumerate()
        .map(|(x, line)| {
            line.iter()
                .enumerate()
                .map(|(y, number)| {
                    if *number == 0 {
                        let a = find_trails2(&numbers, x, y);
                        a
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        })
        .sum::<u32>();
    Some(sum)
}

fn find_trails2(numbers: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    if numbers[x][y] == 9 {
        return 1;
    }
    let mut sum = 0;
    if x > 0 && numbers[x - 1][y] == numbers[x][y] + 1 {
        sum += find_trails2(numbers, x - 1, y);
    }
    if x < numbers.len() - 1 && numbers[x + 1][y] == numbers[x][y] + 1 {
        sum += find_trails2(numbers, x + 1, y);
    }
    if y > 0 && numbers[x][y - 1] == numbers[x][y] + 1 {
        sum += find_trails2(numbers, x, y - 1);
    }
    if y < numbers[x].len() - 1 && numbers[x][y + 1] == numbers[x][y] + 1 {
        sum += find_trails2(numbers, x, y + 1);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_day_10() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two_day_10() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
