use std::panic;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize {
    let lines = input.lines();
    let mut sum_of_mx = 0;
    for line in lines {
        let mut max_turn = Turn::default();
        let (_, turns) = line.split_once(": ").unwrap();

        let turns: Vec<&str> = turns.split("; ").collect();
        for t in turns {
            let cubes: Vec<&str> = t.split(", ").collect();
            for ch in cubes {
                let (amount, color) = ch.split_once(' ').unwrap();
                let amount = amount.parse::<usize>().unwrap();
                match color {
                    "red" => max_turn.red = amount.max(max_turn.red),
                    "green" => max_turn.green = amount.max(max_turn.green),
                    "blue" => max_turn.blue = amount.max(max_turn.blue),
                    _ => panic!("impossible"),
                }
            }
        }
        let temp = max_turn.blue * max_turn.green * max_turn.red;
        sum_of_mx += temp;
    }
    sum_of_mx
}

#[derive(Default, Debug)]
struct Turn {
    red: usize,
    green: usize,
    blue: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let res = process(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(res, 2286);
    }
}
