advent_of_code::solution!(1);

// fn take_smalles(list: &Vec<i32>) -> Option<i32> {
//     if list.len() == 0 {
//         return None;
//     }
//     let min_value = *list.iter().min().unwrap();

//     return Some(min_value);
// }

pub fn part_one(input: &str) -> Option<u32> {
    let mut lefts: Vec<i32> = vec![];
    let mut rights: Vec<i32> = vec![];

    input.lines().for_each(|line| {
        let thing = line.split_whitespace().collect::<Vec<&str>>();
        let left = thing[0]
            .parse::<i32>()
            .expect("Failed to parse i32 from string.");
        let right = thing[1]
            .parse::<i32>()
            .expect("Failed to parse i32 from string.");
        lefts.push(left);
        rights.push(right);
    });

    lefts.sort();
    rights.sort();

    let mut distance = 0;

    for n in 0..lefts.len() {
        distance += (lefts[n] - rights[n]).abs() as u32;
    }

    Some(distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lefts: Vec<i32> = vec![];
    let mut rights: Vec<i32> = vec![];

    input.lines().for_each(|line| {
        let thing = line.split_whitespace().collect::<Vec<&str>>();
        let left = thing[0]
            .parse::<i32>()
            .expect("Failed to parse i32 from string.");
        let right = thing[1]
            .parse::<i32>()
            .expect("Failed to parse i32 from string.");
        lefts.push(left);
        rights.push(right);
    });

    let mut sum = 0;
    for n in 0..lefts.len() {
        let curr = lefts[n];
        let count = rights
            .iter()
            .fold(0, |count, n| if n == &curr { count + 1 } else { count });
        sum += count * curr
    }

    Some(sum as u32)
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
