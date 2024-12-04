use std::cmp::min;

use regex::Regex;

advent_of_code::solution!(4);

fn count_horizontal(input: &Vec<Vec<char>>) -> u32 {
    let re = Regex::new(r"XMAS").unwrap();

    (0..input.len())
        .map(|i| {
            let chars = input[i].iter().collect::<String>();
            let charsr = input[i].iter().rev().collect::<String>();
            let res1 = re.captures_iter(&chars).count();
            let res2 = re.captures_iter(&charsr).count();

            (res1 + res2) as u32
        })
        .sum()
}

fn count_vertical(input: &Vec<Vec<char>>) -> u32 {
    let re = Regex::new(r"XMAS").unwrap();

    (0..input[0].len())
        .map(|i| {
            let chars = (0..input.len()).map(|j| input[j][i]).collect::<String>();
            let charsr = (0..input.len())
                .map(|j| input[j][i])
                .rev()
                .collect::<String>();

            let res1 = re.captures_iter(&chars).count();
            let res2 = re.captures_iter(&charsr).count();

            (res1 + res2) as u32
        })
        .sum()
}
fn count_diagonal_down(input: &Vec<Vec<char>>) -> u32 {
    let re = Regex::new(r"XMAS").unwrap();
    (3..input.len() - 1)
        .map(|start| {
            let m = min(start + 1, input[start].len());
            let chars = (0..m).map(|i| input[start - i][i]).collect::<String>();

            let charsr = (0..m)
                .map(|i| input[start - i][i])
                .rev()
                .collect::<String>();
            let res1 = re.captures_iter(&chars).count();
            let res2 = re.captures_iter(&charsr).count();

            (res1 + res2) as u32
        })
        .sum::<u32>()
        + (0..input[0].len() - 3)
            .map(|start| {
                let m = min(input[0].len() - start, input.len());
                let chars = (0..m)
                    .map(|i| input[input.len() - i - 1][start + i])
                    .collect::<String>();

                let charsr = (0..m)
                    .map(|i| input[input.len() - i - 1][start + i])
                    .rev()
                    .collect::<String>();
                let res1 = re.captures_iter(&chars).count();
                let res2 = re.captures_iter(&charsr).count();

                (res1 + res2) as u32
            })
            .sum::<u32>()
}
fn count_diagonal_up(input: &Vec<Vec<char>>) -> u32 {
    let re = Regex::new(r"XMAS").unwrap();
    (1..input.len() - 3)
        .rev()
        .map(|start| {
            let m = min(input.len() - start, input[start].len());
            let chars = (0..m).map(|i| input[start + i][i]).collect::<String>();

            let charsr = (0..m)
                .map(|i| input[start + i][i])
                .rev()
                .collect::<String>();
            let res1 = re.captures_iter(&chars).count();
            let res2 = re.captures_iter(&charsr).count();

            (res1 + res2) as u32
        })
        .sum::<u32>()
        + (0..input[0].len() - 3)
            .map(|start| {
                let m = min(input[0].len() - start, input[start].len());
                let chars = (0..m).map(|i| input[i][start + i]).collect::<String>();

                let charsr = (0..m)
                    .map(|i| input[i][start + i])
                    .rev()
                    .collect::<String>();
                let res1 = re.captures_iter(&chars).count();
                let res2 = re.captures_iter(&charsr).count();

                (res1 + res2) as u32
            })
            .sum::<u32>()
}
fn has_x_mas(input: &Vec<Vec<char>>, x: usize, y: usize, compare: &Vec<Vec<char>>) -> bool {
    for i in 0..compare.len() {
        for j in 0..compare[i].len() {
            if compare[i][j] != '.' && compare[i][j] != input[x + i][y + j] {
                return false;
            }
        }
    }
    true
}
pub fn part_one(input: &str) -> Option<u32> {
    let letters = input
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let horizontal = count_horizontal(&letters);
    let vertical = count_vertical(&letters);
    let diagonal_down = count_diagonal_down(&letters);
    let diagonal_up = count_diagonal_up(&letters);
    Some(horizontal + vertical + diagonal_down + diagonal_up)
}

pub fn part_two(input: &str) -> Option<u32> {
    let letters = input
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let matches = vec![
        vec![
            vec!['M', '.', 'S'],
            vec!['.', 'A', '.'],
            vec!['M', '.', 'S'],
        ],
        vec![
            vec!['M', '.', 'M'],
            vec!['.', 'A', '.'],
            vec!['S', '.', 'S'],
        ],
        vec![
            vec!['S', '.', 'M'],
            vec!['.', 'A', '.'],
            vec!['S', '.', 'M'],
        ],
        vec![
            vec!['S', '.', 'S'],
            vec!['.', 'A', '.'],
            vec!['M', '.', 'M'],
        ],
    ];

    Some(
        (0..letters.len() - 2)
            .map(|x| {
                (0..letters[x].len() - 2)
                    .map(|y| {
                        let work = matches
                            .iter()
                            .filter(|thing| has_x_mas(&letters, x, y, thing));

                        work.count() as u32
                    })
                    .sum::<u32>()
            })
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two1() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
