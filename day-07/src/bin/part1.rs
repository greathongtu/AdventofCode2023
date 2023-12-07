use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

struct Hand {
    cards: Vec<char>,
    bid: usize,
}

impl Hand {
    fn hand_type(&self) -> usize {
        let mut map = HashMap::new();
        for i in 0..5 {
            *map.entry(self.cards[i]).or_insert(0) += 1;
        }
        if self.cards[0] == self.cards[1]
            && self.cards[0] == self.cards[2]
            && self.cards[0] == self.cards[3]
            && self.cards[0] == self.cards[4]
        {
            return 7;
        } else if *map.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1 == 4 {
            return 6;
        } else if *map.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1 == 3 {
            for (_, &v) in map.iter() {
                if v == 2 {
                    return 5;
                }
            }
            return 4;
        } else if *map.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1 == 2 {
            let mut cnt = 0;
            for (_, &v) in map.iter() {
                if v == 2 {
                    cnt += 1;
                }
            }
            if cnt == 2 {
                return 3;
            }
            return 2;
        } else {
            return 1;
        }
    }
}

fn convert_card(c: char) -> char {
    match c {
        '2' => 'B',
        '3' => 'C',
        '4' => 'D',
        '5' => 'E',
        '6' => 'F',
        '7' => 'G',
        '8' => 'H',
        '9' => 'I',
        'T' => 'J',
        'J' => 'K',
        'Q' => 'L',
        'K' => 'M',
        'A' => 'N',
        _ => {
            eprintln!("{c}");
            panic!("Unknown card");
        }
    }
}

fn process(input: &str) -> usize {
    let lines = input.lines();
    let mut hands: Vec<Hand> = Vec::new();
    for line in lines {
        let (cards, bid) = line.split_once(" ").unwrap();
        let cards = cards.chars().map(|c| convert_card(c));
        let hand = Hand {
            cards: cards.collect(),
            bid: bid.parse().unwrap(),
        };
        hands.push(hand);
    }
    hands.sort_unstable_by(|a, b| {
        if a.hand_type() != b.hand_type() {
            return a.hand_type().cmp(&b.hand_type());
        }
        for i in 0..5 {
            if a.cards[i] != b.cards[i] {
                return a.cards[i].cmp(&b.cards[i]);
            }
        }
        std::cmp::Ordering::Equal
    });
    let mut res = 0;
    for (i, hand) in hands.iter().enumerate() {
        let bid = hand.bid;
        let rank = i + 1;
        println!("{}:{bid} and {rank}", hand.cards.iter().collect::<String>());
        res += hand.bid * (i + 1);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let res = process(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );
        assert_eq!(res, 6440);
    }
}
