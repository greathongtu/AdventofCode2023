fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize {
    let mut lines = input.lines();
    let time_line = lines.next().unwrap();
    let dis_line = lines.next().unwrap();
    let times = time_line
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let dists = dis_line
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut res = 1;
    for (idx, &time) in times.iter().enumerate() {
        let mut cnt = 0;
        for t in 0..time {
            if t * (time - t) > dists[idx] {
                cnt += 1;
            }
        }
        res *= cnt;
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
        assert_eq!(res, 288);
    }
}
