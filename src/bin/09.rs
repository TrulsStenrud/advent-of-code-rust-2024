advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    let mut file_id = 0;
    let mut chars = Vec::with_capacity(input.len() * 10);
    for (i, char) in input.chars().enumerate() {
        let n = char.to_digit(10).expect(&format!("Whaaat {}", char));

        if i % 2 == 0 {
            for _ in 0..n {
                chars.push(file_id);
            }
            file_id += 1;
        } else {
            for _ in 0..n {
                chars.push(-1);
            }
        }
    }

    let mut i = 0;
    let mut j = chars.len() - 1;

    while i < j {
        if chars[i] != -1 {
            i += 1;
        } else if chars[j] == -1 {
            j -= 1;
        } else {
            let temp = chars[i];
            chars[i] = chars[j];
            chars[j] = temp;
        }
    }

    Some(
        chars
            .iter()
            .enumerate()
            .map(|(i, c)| if *c != -1 { *c as i64 * i as i64 } else { 0 })
            .sum::<i64>(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut file_id = 0;
    let mut chars = Vec::with_capacity(input.len() * 10);
    for (i, char) in input.chars().enumerate() {
        let n = char.to_digit(10).expect(&format!("Whaaat {}", char));

        if i % 2 == 0 {
            for _ in 0..n {
                chars.push(file_id);
            }
            file_id += 1;
        } else {
            for _ in 0..n {
                chars.push(-1);
            }
        }
    }

    let mut i = 0;
    let mut j = chars.len() - 1;

    while i < j {
        if chars[i] != -1 {
            i += 1;
        } else if chars[j] == -1 {
            j -= 1;
        } else {
            let temp = chars[i];
            chars[i] = chars[j];
            chars[j] = temp;
        }
    }

    Some(
        chars
            .iter()
            .enumerate()
            .map(|(i, c)| if *c != -1 { *c as i64 * i as i64 } else { 0 })
            .sum::<i64>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_day_9() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two_day_9() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
