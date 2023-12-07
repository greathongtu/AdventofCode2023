fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize {
    let mut lines = input.lines();
    let time_line = lines.next().unwrap();
    let dis_line = lines.next().unwrap();
    let time = time_line
        .split_once(':')
        .unwrap()
        .1
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let dist = dis_line
        .split_once(':')
        .unwrap()
        .1
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    println!("{time} and {dist}");
    let mut res = 0;
    for t in 0..time {
        if t * (time - t) > dist {
            res += 1;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let res = process(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(res, 71503);
    }
}
