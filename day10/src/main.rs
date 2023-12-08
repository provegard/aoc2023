use std::collections::HashMap;

use anyhow::Result;

use itertools::Itertools;
use util::Input;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Coordinate {
    x: i16,
    y: i16,
}

struct Map {
    coordinates: HashMap<Coordinate, char>,
}

fn to_map(input: &Input) -> Map {
    let coordinates: HashMap<_, _> = input.as_lines().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().map(move |(x, ch)| (Coordinate { x: x as i16, y: y as i16 }, ch))
    }).collect();
    Map { coordinates }
}

fn coordinates_from(tile: &char, c: &Coordinate) -> Vec<Coordinate> {
    match tile {
        '|' => vec![Coordinate { x: c.x, y: c.y - 1 }, Coordinate { x: c.x, y: c.y + 1 }],
        '-' => vec![Coordinate { x: c.x - 1, y: c.y }, Coordinate { x: c.x + 1, y: c.y }],
        'F' => vec![Coordinate { x: c.x + 1, y: c.y }, Coordinate { x: c.x, y: c.y + 1 }],
        'J' => vec![Coordinate { x: c.x - 1, y: c.y }, Coordinate { x: c.x, y: c.y - 1 }],
        '7' => vec![Coordinate { x: c.x - 1, y: c.y }, Coordinate { x: c.x, y: c.y + 1 }],
        'L' => vec![Coordinate { x: c.x + 1, y: c.y }, Coordinate { x: c.x, y: c.y - 1 }],
        'S' => vec![
            Coordinate { x: c.x - 1, y: c.y },
            Coordinate { x: c.x + 1, y: c.y },
            Coordinate { x: c.x, y: c.y - 1 },
            Coordinate { x: c.x, y: c.y + 1 },
        ],
        _ => vec![],
    }
}

fn find_starting_point(map: &Map) -> (Coordinate, Coordinate, Coordinate) {
    let s_opt = map.coordinates.iter().find(|c| *c.1 == 'S');
    match s_opt {
        Some(s) => {
            // Figure out which coordinates the starting point connects to.
            let coords_from = coordinates_from(s.1, s.0);
            let connected = coords_from
                .iter()
                .filter(|candidate| {
                    if let Some(tile) = map.coordinates.get(candidate) {
                        coordinates_from(tile, candidate).contains(s.0)    
                    } else { false }
                })
                .collect_vec();
            match connected[..] {
                [a, b] => {
                    let triple = (s.0.clone(), a.clone(), b.clone());
                    triple
                },
                _ => panic!("Failed to find connections from starting point")
            }
        },
        None => panic!("Failed to find a starting point"),
    }
}

fn find_pipe_coordinates(map: &Map) -> Vec<Coordinate> {
    let mut pipe: Vec<Coordinate> = Vec::new();
    let s = find_starting_point(map);
    pipe.push(s.0.clone());
    let mut from = s.0;
    let mut current = s.1; // arbitrary, could be s.2 as well

    while let Some(tile) = map.coordinates.get(&current) {
        match tile {
            'S' => break, // done
            _ => {
                let coords_from = coordinates_from(tile, &current);
                let next = coords_from.iter().find(|cc| **cc != from);
                match next {
                    Some(n) => {
                        from = current.clone(); // TODO: get rid of clone
                        current = n.clone();
                        pipe.push(n.clone());
                    },
                    None => panic!("No next coordinate"),
                }
            }
        }
    }

    pipe
}

fn find_max_dist(map: &Map) -> usize {
    let pipe = find_pipe_coordinates(map);
    pipe.len() / 2
}

fn part1(input: &Input) -> Result<u32> {
    let map = to_map(input);
    let dist = find_max_dist(&map);
    Ok(dist as u32)
}

fn part2(input: &Input) -> Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};
    use anyhow::Result;
    use util::Input;

    #[test]
    pub fn test_part1_1() -> Result<()> {
        let input = Input::from_lines([
            "-L|F7",
            "7S-7|",
            "L|7||",
            "-L-J|",
            "L|-JF",
        ]);
        assert_eq!(part1(&input).unwrap(), 4);
        Ok(())
    }

    #[test]
    pub fn test_part1_2() -> Result<()> {
        let input = Input::from_lines([
            "7-F7-",
            ".FJ|7",
            "SJLL7",
            "|F--J",
            "LJ.LJ",
        ]);
        assert_eq!(part1(&input).unwrap(), 8);
        Ok(())
    }

    #[test]
    pub fn test_part1_input() -> Result<()> {
        let input = Input::load("input")?;
        assert_eq!(part1(&input).unwrap(), 6828);
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
