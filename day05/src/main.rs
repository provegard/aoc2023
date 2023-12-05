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

fn part2(input: &Input) -> Result<u64> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use crate::{part1, part2, parse_almanac, Map, MapPart, Almanac, ComponentKind};
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

    // #[test]
    // pub fn test_part2() -> Result<()> {
    //     let input = Input::from_lines([
    //     ]);
    //     assert_eq!(part2(&input).unwrap(), 0);
    //     Ok(())
    // }
}
