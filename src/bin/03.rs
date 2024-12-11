advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\(([0-9]{1,3},[0-9]{1,3})\)").unwrap();
    let test = re.captures_iter(input);

    Some(
        test.map(|x| {
            let mut nums = x[1].split(",");
            let a: u32 = nums.next().unwrap().parse().unwrap();
            let b: u32 = nums.next().unwrap().parse().unwrap();
            a * b
        })
        .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    // let re =
    //     Regex::new(r"(?<dont>don't\(\))|(?<do>do\(\))|mul\(?<mul>[0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let re = Regex::new(r"(don't\(\))|(do\(\))|mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let test = re.captures_iter(input);

    let mut doing = true;

    Some(
        test.map(|x| {
            let curr = &x[0];

            if curr == "do()" {
                doing = true;
                0
            } else if curr == "don't()" {
                doing = false;
                0
            } else if doing {
                let mut c = curr.chars();
                c.next_back();
                c.next();
                c.next();
                c.next();
                c.next();
                let s = c.as_str();
                let mut nums = s.split(",");
                let a: u32 = nums.next().unwrap().parse().unwrap();
                let b: u32 = nums.next().unwrap().parse().unwrap();
                a * b
            } else {
                0
            }
        })
        .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_day_3() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two2_day_3() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
