use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let (seeds, maps) = process("./input.txt");
    let mut min_location = usize::MAX;

    for mut location in seeds {
        for map in &maps {
            for entry in &map.entries {
                if entry.source <= location && location < entry.source + entry.length {
                    location = entry.destination + location - entry.source;
                    break;
                }
            }
        }
        min_location = min_location.min(location);
    }
    dbg!(min_location);
}

#[derive(Clone)]
struct MapEntry {
    destination: usize,
    source: usize,
    length: usize,
}

#[derive(Default, Clone)]
struct Map {
    entries: Vec<MapEntry>,
}

fn read_lines_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let content = BufReader::new(file);
    let lines: Vec<String> = content
        .lines()
        .map(|line| line.expect("Something went wrong"))
        .collect();
    lines
}

fn process(filename: &str) -> (Vec<usize>, Vec<Map>) {
    let input = read_lines_from_file(filename);
    let (_, seeds) = input[0].split_once(": ").unwrap();
    let seeds: Vec<&str> = seeds.split_whitespace().collect();
    let seeds: Vec<usize> = seeds.iter().map(|n| n.parse::<usize>().unwrap()).collect();

    let mut maps = Vec::new();
    maps.resize(7, Map::default());
    let mut i_map = 0;

    for i in 2..input.len() {
        let line = &input[i];
        if line.is_empty() {
            i_map += 1;
            continue;
        }

        if !line.chars().nth(0).unwrap().is_digit(10) {
            continue;
        }

        read_tokens(&line, i_map, &mut maps);
    }

    return (seeds, maps);
}

fn read_tokens(line: &str, i_map: usize, maps: &mut Vec<Map>) {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    let tokens: Vec<usize> = tokens
        .into_iter()
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    maps[i_map].entries.push(MapEntry {
        destination: tokens[0],
        source: tokens[1],
        length: tokens[2],
    });
}
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let res = process(
//             "1abc2
// pqr3stu8vwx
// a1b2c3d4e5f
// treb7uchet",
//         );
//         assert_eq!(res, 142);
//     }
// }
