use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut chars_pos = HashMap::<char, Vec<(isize, isize)>>::new();
    input.lines().enumerate().for_each(|(x, line)| {
        line.chars().enumerate().for_each(|(y, c)| {
            if c == '.' {
                // do notjing
            } else {
                if let Some(pos) = chars_pos.get_mut(&c) {
                    pos.push((x as isize, y as isize));
                } else {
                    chars_pos.insert(c, vec![(x as isize, y as isize)]);
                }
            }
        })
    });

    let max_x: isize = input.lines().count() as isize - 1;
    let max_y: isize = input.lines().next().unwrap().len() as isize - 1;

    let mut antinodes = HashSet::new();

    for value in chars_pos.values() {
        for i in 0..value.len() - 1 {
            for j in (i + 1)..value.len() {
                let (anti_i, anti_j) = anti_node(value[i], value[j]);
                if anti_i <= max_x
                    && anti_i >= 0
                    && anti_j <= max_y
                    && anti_j >= 0
                    && !antinodes.contains(&(anti_i, anti_j))
                {
                    antinodes.insert((anti_i, anti_j));
                }
                let (anti_i, anti_j) = anti_node(value[j], value[i]);
                if anti_i <= max_x
                    && anti_i >= 0
                    && anti_j <= max_y
                    && anti_j >= 0
                    && !antinodes.contains(&(anti_i, anti_j))
                {
                    antinodes.insert((anti_i, anti_j));
                }
            }
        }
    }

    // for (x, line) in input.lines().enumerate() {
    //     for (y, char) in line.chars().enumerate() {
    //         if antinodes.contains(&(x as isize, y as isize)) {
    //             print!("#")
    //         } else {
    //             print!("{}", char)
    //         }
    //     }
    //     println!();
    // }
    Some(antinodes.iter().count() as u32)
}
fn anti_node(node_a: (isize, isize), node_b: (isize, isize)) -> (isize, isize) {
    return (
        node_b.0 - (node_a.0 - node_b.0),
        node_b.1 - (node_a.1 - node_b.1),
    );
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut chars_pos = HashMap::<char, Vec<(isize, isize)>>::new();
    input.lines().enumerate().for_each(|(x, line)| {
        line.chars().enumerate().for_each(|(y, c)| {
            if c == '.' {
                // do notjing
            } else {
                if let Some(pos) = chars_pos.get_mut(&c) {
                    pos.push((x as isize, y as isize));
                } else {
                    chars_pos.insert(c, vec![(x as isize, y as isize)]);
                }
            }
        })
    });

    let max_x: isize = input.lines().count() as isize - 1;
    let max_y: isize = input.lines().next().unwrap().len() as isize - 1;

    let mut antinodes = HashSet::new();

    for value in chars_pos.values() {
        for i in 0..value.len() - 1 {
            for j in (i + 1)..value.len() {
                let mut curr = value[i];
                let mut next = value[j];
                for _ in 0..400 {
                    let (anti_i, anti_j) = anti_node(curr, next);
                    if anti_i <= max_x
                        && anti_i >= 0
                        && anti_j <= max_y
                        && anti_j >= 0
                        && !antinodes.contains(&(anti_i, anti_j))
                    {
                        antinodes.insert((anti_i, anti_j));
                    }
                    curr = next;
                    next = (anti_i, anti_j);
                }

                let mut curr = value[j];
                let mut next = value[i];
                for _ in 0..400 {
                    let (anti_i, anti_j) = anti_node(curr, next);
                    if anti_i <= max_x
                        && anti_i >= 0
                        && anti_j <= max_y
                        && anti_j >= 0
                        && !antinodes.contains(&(anti_i, anti_j))
                    {
                        antinodes.insert((anti_i, anti_j));
                    }
                    curr = next;
                    next = (anti_i, anti_j);
                }
            }
        }
    }

    for (x, line) in input.lines().enumerate() {
        for (y, char) in line.chars().enumerate() {
            if antinodes.contains(&(x as isize, y as isize)) {
                print!("#")
            } else {
                print!("{}", char)
            }
        }
        println!();
    }
    let antinode_count = antinodes.iter().count() as u32;
    let antenna_count = chars_pos
        .iter()
        .map(|(_, v)| {
            v.iter()
                .filter(|(x, y)| !antinodes.contains(&(*x, *y)))
                .count()
        })
        .sum::<usize>() as u32;
    Some(antinode_count + antenna_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one21() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two3123() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
