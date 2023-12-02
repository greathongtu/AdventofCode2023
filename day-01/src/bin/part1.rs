fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> u64 {
    let lines = input.lines();
    let mut res = 0;
    for line in lines {
        let mut tmp: u64 = 0;
        for ch in line.chars() {
            if ch.is_numeric() {
                tmp = ch.to_digit(10).unwrap() as u64 * 10;
                break;
            }
        }
        let mut last = 0;
        for ch in line.chars() {
            if ch.is_numeric() {
                last = ch.to_digit(10).unwrap() as u64;
            }
        }
        tmp += last;
        res += tmp;
    }
    res
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let res = process(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(res, 142);
    }
}
