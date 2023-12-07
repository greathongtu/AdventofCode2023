use std::cmp;
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Interval {
    first: i64,
    last: i64,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Mapping {
    source: i64,
    destination: i64,
    length: i64,
}

struct Conversion {
    mappings: Vec<Mapping>,
}

fn build_map(lineptr: &mut std::str::Lines) -> Conversion {
    let mut conversion_map: Conversion = Conversion { mappings: vec![] };
    lineptr.next();
    for line in lineptr.by_ref() {
        if line.is_empty() {
            break;
        } else {
            let values: Vec<i64> = line
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
            conversion_map.mappings.push(Mapping {
                source: values[1],
                destination: values[0],
                length: values[2],
            });
        }
    }
    conversion_map
}

fn process(contents: &str) -> i64 {
    let mut lineptr = contents.lines();
    let mut lowest: i64 = i64::MAX;
    let seeds: Vec<i64> = lineptr
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    lineptr.next().expect("unexpected EOF");

    let mut conversions: Vec<Conversion> = vec![];
    for _ in 0..7 {
        conversions.push(build_map(&mut lineptr));
    }

    for i in (0..seeds.len()).step_by(2) {
        let first: i64 = seeds[i];
        let length: i64 = seeds[i + 1];
        let mut ranges: VecDeque<Interval> = VecDeque::from([Interval {
            first,
            last: first + length - 1,
        }]);
        for conversion in conversions.iter() {
            let mut new_ranges: Vec<Interval> = vec![];
            while !ranges.is_empty() {
                let range: Interval = ranges.pop_front().unwrap();
                let mut found: bool = false;
                for mapping in conversion.mappings.iter() {
                    if range.first >= mapping.source && range.last < mapping.source + mapping.length
                    {
                        new_ranges.push(Interval {
                            first: range.first - mapping.source + mapping.destination,
                            last: range.last - mapping.source + mapping.destination,
                        });
                        found = true;
                    } else if range.first < mapping.source
                        && range.last >= mapping.source
                        && range.last < mapping.source + mapping.length
                    {
                        ranges.push_back(Interval {
                            first: range.first,
                            last: mapping.source - 1,
                        });
                        new_ranges.push(Interval {
                            first: mapping.destination,
                            last: mapping.destination + range.last - mapping.source,
                        });
                        found = true;
                    } else if range.first < mapping.source + mapping.length
                        && range.last >= mapping.source + mapping.length
                        && range.first >= mapping.source
                    {
                        ranges.push_back(Interval {
                            first: mapping.source + mapping.length,
                            last: range.last,
                        });
                        new_ranges.push(Interval {
                            first: mapping.destination + range.first - mapping.source,
                            last: mapping.destination + mapping.length - 1,
                        });
                        found = true;
                    } else if range.first < mapping.source
                        && range.last >= mapping.source + mapping.length
                    {
                        ranges.push_back(Interval {
                            first: range.first,
                            last: mapping.source - 1,
                        });
                        new_ranges.push(Interval {
                            first: mapping.destination,
                            last: mapping.destination + mapping.length - 1,
                        });
                        ranges.push_back(Interval {
                            first: mapping.source + mapping.length,
                            last: range.last,
                        });
                        found = true;
                    }
                    if found {
                        break;
                    }
                }
                if !found {
                    new_ranges.push(range);
                }
            }
            ranges = VecDeque::from(new_ranges);
        }
        lowest = cmp::min(lowest, ranges.iter().min_by_key(|r| r.first).unwrap().first);
    }
    lowest
}

fn main() {
    let contents = include_str!("./input.txt");
    let result = process(&contents);
    println!("result = {result}");
}
