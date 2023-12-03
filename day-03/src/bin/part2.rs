use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> u32 {
    let two_d_vec = read_to_matrix(input);

    let mut gear2number: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    let mut adjacent_gears = HashSet::new();
    for i in 0..two_d_vec.len() {
        let mut curr_num = 0;
        for j in 0..two_d_vec[i].len() {
            let ch = two_d_vec[i][j];
            let is_digit = ch.is_digit(10);
            if is_digit {
                curr_num = curr_num * 10 + ch.to_digit(10).unwrap();

                adjacent_gears.extend(&get_adjacent_gears(&two_d_vec, (i, j)));
            }
            if j == two_d_vec[i].len() - 1 || !is_digit {
                for &gear in &adjacent_gears {
                    gear2number.entry(gear).or_default().push(curr_num);
                }
                adjacent_gears.clear();
                curr_num = 0;
            }
        }
    }
    let mut res = 0;
    for (_, numbers) in gear2number {
        if numbers.len() == 2 {
            let tmp = numbers[0] * numbers[1];
            res += tmp;
        }
    }
    res
}

fn get_adjacent_gears(
    two_d_vec: &Vec<Vec<char>>,
    (i, j): (usize, usize),
) -> HashSet<(usize, usize)> {
    let mut adj_gears = HashSet::new();
    let dirs = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for (dx, dy) in dirs {
        let newx = dx + i as isize;
        let newy = dy + j as isize;
        if newx < 0
            || newx >= two_d_vec.len() as isize
            || newy < 0
            || newy >= two_d_vec[0].len() as isize
        {
            continue;
        }
        let c = two_d_vec[newx as usize][newy as usize];
        if c == '*' {
            adj_gears.insert((newx as usize, newy as usize));
        }
    }
    adj_gears
}

fn read_to_matrix(input: &str) -> Vec<Vec<char>> {
    let lines = input.lines();
    let mut two_d_vec = Vec::new();

    for line in lines {
        let chars = line.chars();
        let mut char_vec = Vec::new();
        for ch in chars {
            char_vec.push(ch);
        }
        two_d_vec.push(char_vec);
    }
    two_d_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let res = process(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(res, 467835);
    }
}
