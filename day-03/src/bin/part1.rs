fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> u32 {
    let lines = input.lines();
    let mut all_chars = Vec::new();
    let mut numbers = Vec::new();

    for line in lines {
        let chars = line.chars();
        let mut char_vec = Vec::new();
        for ch in chars {
            char_vec.push(ch);
        }
        all_chars.push(char_vec);
    }
    for i in 0..all_chars.len() {
        let mut curr_num = 0;
        let mut adj = false;
        for j in 0..all_chars[i].len() {
            let ch = all_chars[i][j];
            let is_digit = ch.is_digit(10);
            if is_digit {
                curr_num = curr_num * 10 + ch.to_digit(10).unwrap();
                adj = adj || is_symbol_adj((i, j), &all_chars);
            }
            if j == all_chars[i].len() - 1 || !is_digit {
                if adj {
                    numbers.push(curr_num);
                }
                curr_num = 0;
                adj = false;
            }
        }
    }
    numbers.iter().sum()
}

fn is_symbol_adj((i, j): (usize, usize), two_d_vec: &Vec<Vec<char>>) -> bool {
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
        let newi = i as isize + dx;
        let newj = j as isize + dy;
        if newi >= two_d_vec.len() as isize
            || newi < 0
            || newj >= two_d_vec[0].len() as isize
            || newj < 0
        {
            continue;
        }
        let ch = two_d_vec[newi as usize][newj as usize];
        if !ch.is_digit(10) && ch != '.' {
            return true;
        }
    }
    return false;
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
        assert_eq!(res, 4361);
    }
}
