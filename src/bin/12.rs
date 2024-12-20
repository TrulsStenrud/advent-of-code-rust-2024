use std::collections::{HashMap, HashSet};

advent_of_code::solution!(12);

fn explore(
    pos: (usize, usize),
    fields: &Vec<Vec<char>>,
    used_locations: &mut HashSet<(usize, usize)>,
    fence: &mut u32,
    area: &mut u32,
) {
    if used_locations.contains(&pos) {
        return;
    }
    let curr = fields[pos.0][pos.1];
    *area += 1;

    used_locations.insert(pos);

    let (x, y) = pos;

    if x > 0 {
        if fields[x - 1][y] == curr {
            explore((x - 1, y), fields, used_locations, fence, area);
        } else {
            *fence += 1;
        }
    } else {
        *fence += 1
    }

    if y > 0 {
        if fields[x][y - 1] == curr {
            explore((x, y - 1), fields, used_locations, fence, area);
        } else {
            *fence += 1;
        }
    } else {
        *fence += 1
    }

    if x < fields.len() - 1 {
        if fields[x + 1][y] == curr {
            explore((x + 1, y), fields, used_locations, fence, area);
        } else {
            *fence += 1;
        }
    } else {
        *fence += 1
    }

    if y < fields[x].len() - 1 {
        if fields[x][y + 1] == curr {
            explore((x, y + 1), fields, used_locations, fence, area);
        } else {
            *fence += 1;
        }
    } else {
        *fence += 1
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let fields = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut used_locations = HashSet::new();

    let mut sum = 0;
    for x in 0..fields.len() {
        for y in 0..fields[x].len() {
            let mut fence = 0;
            let mut area = 0;
            explore((x, y), &fields, &mut used_locations, &mut fence, &mut area);
            // if fence > 0 || area > 0 {
            //     println!("Region {} of {} * {}", fields[x][y], area, fence);
            // }
            sum += area * fence
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_day_12() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two_day_12() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(368));
    }
}
