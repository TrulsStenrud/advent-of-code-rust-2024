advent_of_code::solution!(1);

// fn take_smalles(list: &Vec<i32>) -> Option<i32> {
//     if list.len() == 0 {
//         return None;
//     }
//     let min_value = *list.iter().min().unwrap();

//     return Some(min_value);
// }
//
fn count_occurrances(lsit: &Vec<u32>, value: &u32) -> u32 {
    let mut count = 0;
    for n in lsit {
        if n == value {
            count += 1;
        }
    }
    return count;
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lefts: Vec<u32> = vec![];
    let mut rights: Vec<u32> = vec![];

    input.lines().for_each(|line| {
        let mut numbers = line.split_whitespace();
        let left = numbers
            .next()
            .expect("Not enough numbers in line.")
            .parse::<u32>()
            .expect("Failed to parse u32 from string.");
        let right = numbers
            .next()
            .expect("Not enough numbers in line.")
            .parse::<u32>()
            .expect("Failed to parse u32 from string.");
        lefts.push(left);
        rights.push(right);
    });

    lefts.sort();
    rights.sort();

    Some(
        lefts
            .iter()
            .zip(rights.iter())
            .map(|(left, right)| left.abs_diff(*right))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lefts: Vec<u32> = vec![];
    let mut rights: Vec<u32> = vec![];

    input.lines().for_each(|line| {
        let mut numbers = line.split_whitespace();
        let left = numbers
            .next()
            .expect("Not enough numbers in line.")
            .parse::<u32>()
            .expect("Failed to parse u32 from string.");
        let right = numbers
            .next()
            .expect("Not enough numbers in line.")
            .parse::<u32>()
            .expect("Failed to parse u32 from string.");
        lefts.push(left);
        rights.push(right);
    });

    Some(
        lefts
            .iter()
            .map(|curr| curr * count_occurrances(&rights, &curr))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
