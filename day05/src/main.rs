use std::cmp::{min, max};

use anyhow::{anyhow, Result};
use itertools::Itertools;
use regex::Regex;

use util::Input;

#[derive(PartialEq, Debug, Clone, Copy)]
enum ComponentKind {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location
}

#[derive(PartialEq, Debug)]
struct MapPart {
    source_range_start: u64,
    dest_range_start: u64,
    length: u64,
}

/// Represents a map from one type to another
#[derive(PartialEq, Debug)]
struct Map {
    from_type: ComponentKind,
    to_type: ComponentKind,
    parts: Vec<MapPart>,
}

#[derive(PartialEq, Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Range {
    start: u64,
    length: u64
}

fn intersection(r1: &Range, r2: &Range) -> Option<Range> {
    let smaller = if r1.start < r2.start { r1 } else { r2 };
    let larger = if smaller == r1 { r2 } else { r1 };
    let start = max(smaller.start, larger.start);
    let end_exclusive = min(smaller.start + smaller.length, larger.start + larger.length);
    if end_exclusive <= start {
        None
    } else {
        let length = end_exclusive - start;
        Some(Range { start, length })
    }
}

fn union(r1: &Range, r2: &Range) -> Option<Range> {
    let smaller = if r1.start < r2.start { r1 } else { r2 };
    let larger = if smaller == r1 { r2 } else { r1 };

    if smaller.start + smaller.length < larger.start {
        None
    } else {
        let length = larger.start + larger.length - smaller.start;
        Some(Range { start: smaller.start, length })
    }
}

fn split_range_for_map_part(range: &Range, part: &MapPart) -> Vec<Range> {
    let mut vec = Vec::<Range>::new();
    // Below
    if range.start < part.source_range_start {
        let len = min(part.source_range_start - range.start, range.length);
        vec.push(Range { start: range.start, length: len })
    }

    // Overlap
    let r2 = Range { start: part.source_range_start, length: part.length };
    let diff = (part.dest_range_start as i64) - (part.source_range_start as i64);
    match intersection(range, &r2) {
        Some(r) => vec.push(Range { start: ((r.start as i64) + diff) as u64, length: r.length }),
        None => {}
    };


    // Above
    if range.start >= part.source_range_start + part.length {
        vec.push(range.clone())
    }

    //println!("Split {:?} with {:?} -> {:?}", range, part, vec);

    vec
}

fn to_component_kind(s: &str) -> ComponentKind {
    match s {
        "seed" => ComponentKind::Seed,
        "soil" => ComponentKind::Soil,
        "fertilizer" => ComponentKind::Fertilizer,
        "water" => ComponentKind::Water,
        "light" => ComponentKind::Light,
        "temperature" => ComponentKind::Temperature,
        "humidity" => ComponentKind::Humidity,
        "location" => ComponentKind::Location,
        _ => panic!("Unknown kind: {}", s),
    }
}

fn parse_map_label(line: &str) -> (ComponentKind, ComponentKind) {
    let label_re: Regex = Regex::new("(?<from>[a-z]+)-to-(?<to>[a-z]+) map:").unwrap();
    match label_re.captures(line) {
        Some(caps) => {
            let from = to_component_kind(&caps["from"]);
            let to = to_component_kind(&caps["to"]);
            (from, to)
        }
        None => panic!("Not a label line: {}", line),
    }
}

fn parse_almanac(input: &Input) -> Almanac {
    let lines_vec = input.as_lines().collect_vec();
    let chunks = lines_vec.split(|line| *line == "").map(|chunk| chunk.to_vec()).collect_vec();

    let a = Almanac { seeds: vec![], maps: vec![] };
    chunks.iter().fold(a, |almanac, chunk| {
        let first_line = *chunk.first().unwrap();
        if first_line.starts_with("seeds:") {
            // Parse the seeds line by splitting the numbers.
            let col_idx = first_line.find(":").unwrap();
            let seeds = first_line[(col_idx+1)..].trim().split_ascii_whitespace().map(|s| s.parse::<u64>().unwrap()).collect_vec();
            Almanac {
                seeds,
                maps: almanac.maps,
            }
        } else {
            // Find the components of the map line.
            let components = parse_map_label(first_line);
            // Read each line to get a single mapping.
            let parts = chunk.iter().skip(1).map(|line| {
                let nums = line.split_ascii_whitespace().map(|s| s.parse::<u64>().unwrap()).collect_vec();
                MapPart {
                    dest_range_start: *nums.get(0).unwrap(),
                    source_range_start: *nums.get(1).unwrap(),
                    length: *nums.get(2).unwrap(),
                }
            }).collect_vec();
            let map = Map {
                from_type: components.0,
                to_type: components.1,
                parts,
            };
            Almanac {
                seeds: almanac.seeds,
                maps: almanac.maps.into_iter().chain(vec![map].into_iter()).collect_vec(), // ugly
            }
        }
    })
}

// Maps from a component number using the given Map.
fn map_from(map: &Map, num: u64) -> u64 {
    let part = map.parts.iter().find(|p| num >= p.source_range_start && num < p.source_range_start + p.length);
    match part {
        Some(p) => {
            num + p.dest_range_start - p.source_range_start
        },
        None => num
    }
}

/// Maps from one ComponentType to another, until it is no longer possible.
fn map_from_recursive(almanac: &Almanac, from: (ComponentKind, u64)) -> (ComponentKind, u64) {
    let map = almanac.maps.iter().find(|m| m.from_type == from.0);
    match map {
        Some(m) => {
            let new_from = (m.to_type, map_from(m, from.1));
            map_from_recursive(almanac, new_from)
        },
        None => from,
    }
}

fn part1(input: &Input) -> Result<u64> {
    let almanac = parse_almanac(input);
    let lowest = almanac.seeds
        .iter()
        .map(|s| {
            let mapped = map_from_recursive(&almanac, (ComponentKind::Seed, *s));
            assert!(mapped.0 == ComponentKind::Location);
            mapped.1
        })
        .min();
    match lowest {
        Some(l) => Ok(l),
        None => Err(anyhow!("No min value"))
    }
}

fn merge_ranges(ranges: &Vec<Range>) -> Vec<Range> {
    let mut ranges_copy = ranges.clone();
    ranges_copy.sort_by_key(|r| r.start);
    let merged = ranges_copy.iter().fold(Vec::<Range>::new(), |mut acc, r| {
        match acc.last_mut() {
            Some(last) => {
                match union(last, r) {
                    Some(u_range) => {
                        *last = u_range;
                    },
                    None => acc.push(r.clone()),
                }
            },
            None => acc.push(r.clone()),
        }
        acc
    });
    merged
}

fn map_ranges_recursive(almanac: &Almanac, ranges: Vec<Range>, from: ComponentKind) -> Vec<Range> {

    println!("from: {:?}, ranges = {:?}", from, ranges);

    let mp = almanac.maps.iter().find(|m| m.from_type == from);
    match mp {
        Some(m) => {
            // Find new ranges by splitting them on each map part.
            let new_ranges = m.parts.iter()
                .flat_map(|p| {
                    let rr = ranges.iter().flat_map(|r| split_range_for_map_part(r, p)).collect_vec();
                    rr
                })
                .collect_vec();

            println!("to  : {:?}, ranges = {:?}", m.to_type, new_ranges);

            // Merge the resulting ranges
            let merged = merge_ranges(&new_ranges);

            println!("to  : {:?}, ranges = {:?}", m.to_type, merged);
            println!("");

            map_ranges_recursive(almanac, merged, m.to_type)
        },
        None => ranges, // done
    }
}

fn part2(input: &Input) -> Result<u64> {
    let almanac = parse_almanac(input);
    let ranges = almanac.seeds.chunks(2).map(|arr| Range { start: arr[0], length: arr[1] }).collect_vec();
    let rr = map_ranges_recursive(&almanac, ranges, ComponentKind::Seed);

    println!("{:?}", rr);

    Ok(0)
}

#[cfg(test)]
mod test {
    use crate::{part1, part2, parse_almanac, Map, MapPart, Almanac, ComponentKind, split_range_for_map_part, Range, intersection, union, merge_ranges};
    use anyhow::Result;
    use util::Input;

    #[test]
    pub fn test_parse_almanac() -> Result<()> {
        let input = Input::from_lines([
            "seeds: 1 2 3",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "40 88 3",
            "",
            "soil-to-fertilizer map:",
            "0 15 37"
        ]);
        let a = parse_almanac(&input);

        assert_eq!(a, Almanac {
            seeds: vec![1, 2, 3],
            maps: vec![
                Map {
                    from_type: ComponentKind::Seed,
                    to_type: ComponentKind::Soil,
                    parts: vec![
                        MapPart {
                            dest_range_start: 50,
                            source_range_start: 98,
                            length: 2,
                        },
                        MapPart {
                            dest_range_start: 40,
                            source_range_start: 88,
                            length: 3,
                        },
                    ],
                },
                Map {
                    from_type: ComponentKind::Soil,
                    to_type: ComponentKind::Fertilizer,
                    parts: vec![
                        MapPart {
                            dest_range_start: 0,
                            source_range_start: 15,
                            length: 37,
                        },
                    ],
                },
            ]
        });

        Ok(())
    }

    #[test]
    pub fn test_part1() -> Result<()> {
        let input = Input::load("example")?;
        assert_eq!(part1(&input).unwrap(), 35);
        Ok(())
    }

    #[test]
    pub fn test_part1_input() -> Result<()> {
        let input = Input::load("input")?;
        assert_eq!(part1(&input).unwrap(), 240320250);
        Ok(())
    }

    #[test]
    pub fn test_split_range_for_map_part_below_1() -> Result<()> {
        let part = MapPart { dest_range_start: 20, source_range_start: 30, length: 2 };
        let range = Range { start: 28, length: 1 };
        let res = split_range_for_map_part(&range, &part);

        assert_eq!(res, vec![
            Range { start: 28, length: 1 }
        ]);
        Ok(())
    }

    #[test]
    pub fn test_split_range_for_map_part_below_2() -> Result<()> {
        let part = MapPart { dest_range_start: 20, source_range_start: 30, length: 2 };
        let range = Range { start: 28, length: 2 };
        let res = split_range_for_map_part(&range, &part);

        assert_eq!(res, vec![
            Range { start: 28, length: 2 }
        ]);
        Ok(())
    }

    #[test]
    pub fn test_split_range_for_map_part_overlap() -> Result<()> {
        let part = MapPart { dest_range_start: 20, source_range_start: 30, length: 2 };
        let range = Range { start: 28, length: 3 };
        let res = split_range_for_map_part(&range, &part);

        assert_eq!(res, vec![
            Range { start: 28, length: 2 },
            Range { start: 20, length: 1 }
        ]);
        Ok(())
    }

    #[test]
    pub fn test_split_range_for_map_part_above() -> Result<()> {
        let part = MapPart { dest_range_start: 20, source_range_start: 30, length: 2 };
        let range = Range { start: 32, length: 3 };
        let res = split_range_for_map_part(&range, &part);

        assert_eq!(res, vec![
            Range { start: 32, length: 3 },
        ]);
        Ok(())
    }

    #[test]
    pub fn test_split_range_for_map_part_2() -> Result<()> {
        let part = MapPart { dest_range_start: 52, source_range_start: 50, length: 48 };
        let range = Range { start: 82, length: 1 };
        let res = split_range_for_map_part(&range, &part);

        assert_eq!(res, vec![
            Range { start: 84, length: 1 },
        ]);
        Ok(())
    }

    #[test]
    pub fn test_intersection_1() -> Result<()> {
        let r1 = Range { start: 29, length: 1 };
        let r2 = Range { start: 30, length: 1 };

        assert_eq!(intersection(&r1, &r2), None);
        Ok(())
    }

    #[test]
    pub fn test_intersection_2() -> Result<()> {
        let r1 = Range { start: 31, length: 1 };
        let r2 = Range { start: 30, length: 1 };

        assert_eq!(intersection(&r1, &r2), None);
        Ok(())
    }
    
    #[test]
    pub fn test_intersection_3() -> Result<()> {
        let r1 = Range { start: 29, length: 2 };
        let r2 = Range { start: 30, length: 2 };

        assert_eq!(intersection(&r1, &r2), Some(Range { start: 30, length: 1 }));
        Ok(())
    }

    #[test]
    pub fn test_intersection_4() -> Result<()> {
        let r1 = Range { start: 29, length: 2 };
        let r2 = Range { start: 29, length: 3 };

        assert_eq!(intersection(&r1, &r2), Some(Range { start: 29, length: 2 }));
        Ok(())
    }

    #[test]
    pub fn test_union_1() -> Result<()> {
        let r1 = Range { start: 28, length: 1 };
        let r2 = Range { start: 30, length: 1 };

        assert_eq!(union(&r1, &r2), None);
        Ok(())
    }

    #[test]
    pub fn test_union_2() -> Result<()> {
        let r1 = Range { start: 28, length: 2 };
        let r2 = Range { start: 30, length: 1 };

        assert_eq!(union(&r1, &r2), Some(Range { start: 28, length: 3 }));
        Ok(())
    }

    #[test]
    pub fn test_union_3() -> Result<()> {
        let r1 = Range { start: 30, length: 2 };
        let r2 = Range { start: 30, length: 1 };

        assert_eq!(union(&r1, &r2), Some(Range { start: 30, length: 2 }));
        Ok(())
    }

    #[test]
    pub fn test_merge_ranges_1() -> Result<()> {
        let r1 = Range { start: 29, length: 2 };
        let ranges = vec![r1];
        let merged = merge_ranges(&ranges);

        assert_eq!(merged, vec![r1]);
        Ok(())
    }

    #[test]
    pub fn test_merge_ranges_2() -> Result<()> {
        let r1 = Range { start: 29, length: 2 };
        let r2 = Range { start: 31, length: 2 };
        let ranges = vec![r1, r2];
        let merged = merge_ranges(&ranges);

        assert_eq!(merged, vec![Range { start: 29, length: 4 }]);
        Ok(())
    }

    #[test]
    pub fn test_merge_ranges_3() -> Result<()> {
        let r1 = Range { start: 29, length: 2 };
        let r2 = Range { start: 32, length: 2 };
        let ranges = vec![r1, r2];
        let merged = merge_ranges(&ranges);

        assert_eq!(merged, vec![
            Range { start: 29, length: 2 },
            Range { start: 32, length: 2 },
        ]);
        Ok(())
    }

    #[test]
    pub fn test_merge_ranges_4() -> Result<()> {
        let r1 = Range { start: 29, length: 2 };
        let r2 = Range { start: 31, length: 2 };
        let r3 = Range { start: 33, length: 2 };
        let ranges = vec![r1, r2, r3];
        let merged = merge_ranges(&ranges);

        assert_eq!(merged, vec![
            Range { start: 29, length: 6 },
        ]);
        Ok(())
    }

    #[test]
    pub fn test_merge_ranges_5() -> Result<()> {
        let r1 = Range { start: 29, length: 2 };
        let r2 = Range { start: 31, length: 2 };
        let r3 = Range { start: 34, length: 2 };
        let r4 = Range { start: 36, length: 2 };
        let ranges = vec![r1, r2, r3, r4];
        let merged = merge_ranges(&ranges);

        assert_eq!(merged, vec![
            Range { start: 29, length: 4 },
            Range { start: 34, length: 4 },
        ]);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let input = Input::load("example")?;
        assert_eq!(part2(&input).unwrap(), 46);
        Ok(())
    }
}
