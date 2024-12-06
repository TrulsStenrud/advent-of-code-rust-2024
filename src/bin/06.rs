use std::{cmp::max, collections::HashSet};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut pos: (isize, isize) = (0, 0);
    let mut direction: (isize, isize) = (1, 1);
    let mut trees = HashSet::new();
    let mut max_x: isize = 0;
    let mut max_y: isize = 0;
    input.lines().enumerate().for_each(|(x, line)| {
        line.chars().enumerate().for_each(|(y, char)| {
            if char == '#' {
                trees.insert((x as isize, y as isize));
            } else if char == '<' {
                pos = (x as isize, y as isize);
                direction = (0, -1);
            } else if char == '^' {
                pos = (x as isize, y as isize);
                direction = (-1, 0);
            } else if char == '>' {
                pos = (x as isize, y as isize);
                direction = (0, 1);
            } else if char == 'v' {
                pos = (x as isize, y as isize);
                direction = (1, 0);
            }
            max_x = max(max_x, x as isize);
            max_y = max(max_y, y as isize);
        });
    });

    let mut walked_locations = HashSet::new();

    let mut next = (pos.0 + direction.0, pos.1 + direction.1);
    while pos.0 > 0 && pos.0 <= max_x && pos.1 > 0 && pos.1 <= max_y {
        walked_locations.insert(pos);
        if trees.contains(&next) {
            direction = (direction.1, -direction.0);
        } else {
            pos = next
        }
        next = (pos.0 + direction.0, pos.1 + direction.1);
    }

    // input.lines().enumerate().for_each(|(x, line)| {
    //     line.chars().enumerate().for_each(|(y, char)| {
    //         if walked_locations.contains(&(x as isize, y as isize)) {
    //             print!("X");
    //         } else if trees.contains(&(x as isize, y as isize)) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     });
    //     println!();
    // });

    Some(walked_locations.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one1() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
