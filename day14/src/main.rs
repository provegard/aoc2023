use anyhow::Result;
use itertools::Itertools;
use indexmap::IndexMap;

use util::Input;
use std::fmt;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Coordinate {
    x: u16,
    y: u16,
}

#[derive(Clone, PartialEq, Eq)]
struct Map {
    coordinates: IndexMap<Coordinate, char>,
    width: u16,
    height: u16
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let coord = Coordinate { x, y };
                match self.coordinates.get(&coord) {
                    Some(ch) => write!(f, "{}", ch)?,
                    None => write!(f, ".")?
                };
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    fn new(coordinates: IndexMap<Coordinate, char>, width: u16, height: u16) -> Map {
        Map { coordinates, width, height }
    }
}

impl Map {
    fn tilt_north(&self) -> Map {
        let mut new_coords = IndexMap::<Coordinate, char>::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let coord = Coordinate { x, y, };
                match self.coordinates.get(&coord) {
                    Some('#') => {
                        new_coords.insert(coord.clone(), '#');
                    },
                    Some(_) => {
                        // we have a round rock, see how far north it can go.
                        let closest_rock_at_col = new_coords
                            .iter()
                            .filter(|e| e.0.x == coord.x)
                            .last();
                        let new_y = closest_rock_at_col.map(|e| e.0.y + 1).unwrap_or(0);
                        if new_y < self.height {
                            new_coords.insert(Coordinate { x: coord.x, y: new_y }, 'O');
                        } else {
                            // cannot move
                            new_coords.insert(coord.clone(), 'O');
                        }
                    },
                    None => {},
                }
            }
        }

        Map::new(new_coords, self.width, self.height)
    }

    fn rotate_cw(&self, deg: i8) -> Map {
        if deg == 90 {
            let rotated: IndexMap<_, _> = self.coordinates.iter().map(|(coord, ch)| {
                let nc = Coordinate { x: self.height - coord.y - 1, y: coord.x };
                (nc, ch.clone())
            }).collect();
            let rot = Map::new(rotated, self.height, self.width);
            rot
        } else if deg == -90 {
            let rotated: IndexMap<_, _> = self.coordinates.iter().map(|(coord, ch)| {
                let nc = Coordinate { x: coord.y, y: self.width - coord.x - 1 };
                (nc, ch.clone())
            }).collect();
            let rot = Map::new(rotated, self.height, self.width);
            rot
        } else {
            panic!("Not implemented: {}", deg)
        }
    }

    fn tilt_east(&self) -> Map {
        self.rotate_cw(-90).tilt_north().rotate_cw(90)
    }

    fn tilt_west(&self) -> Map {
        self.rotate_cw(90).tilt_north().rotate_cw(-90)
    }

    fn tilt_south(&self) -> Map {
        self.rotate_cw(90).rotate_cw(90).tilt_north().rotate_cw(-90).rotate_cw(-90)
    }

    fn cycle(&self) -> Map {
        self.tilt_north().tilt_west().tilt_south().tilt_east()
    }

    fn calc_load(&self) -> u32 {
        self.coordinates.iter().fold(0, |acc, e| {
            let val = match e.1 {
                'O' => self.height - e.0.y,
                _ => 0,
            };
            acc + val as u32
        })
    }
}

fn to_map(input: &Input) -> Map {
    let lines = input.as_lines().collect_vec();
    let coordinates: IndexMap<_, _> = lines.iter().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().filter(|(_, ch)| *ch != '.').map(move |(x, ch)| (Coordinate { x: x as u16, y: y as u16 }, ch))
    }).collect();
    let height = lines.len() as u16;
    let width = lines.first().unwrap().len() as u16;
    Map { coordinates, width, height }
}

fn part1(input: &Input) -> Result<u32> {
    let map = to_map(input);
    let new_map = map.tilt_north();
    let load = new_map.calc_load();
    Ok(load)
}

fn part2(input: &Input) -> Result<u32> {
    let m = to_map(input);
    let mut maps = vec![m]; // original map has index 0, meaning subsequent indexes match no. of cycles
    let mut remaining: Option<usize> = None;
    loop {
        let last = maps.last().unwrap();
        let next = last.cycle();
        let cycles = maps.len();

        match remaining {
            None => {
                let eq_map = maps.iter().enumerate().find(|(_, mm)| next == **mm);
                if let Some(x) = eq_map {
                    let period = cycles - x.0;
                    remaining = Some((1000000000 - x.0) % period - 1); // -1 as we'll cycle once before processing remaining
                }
            },
            Some(r) if r > 0 => remaining = Some(r - 1),
            _ => {
                let load = next.calc_load();
                return Ok(load as u32);
            }
        }
        maps.push(next);
    }
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};
    use anyhow::Result;
    use util::Input;

    #[test]
    pub fn test_part1() -> Result<()> {
        let input = Input::load("example")?;
        assert_eq!(part1(&input).unwrap(), 136);
        Ok(())
    }

    #[test]
    pub fn test_part1_input() -> Result<()> {
        let input = Input::load("input")?;
        assert_eq!(part1(&input).unwrap(), 108935);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let input = Input::load("example")?;
        assert_eq!(part2(&input).unwrap(), 64);
        Ok(())
    }

    #[test]
    pub fn test_part2_input() -> Result<()> {
        let input = Input::load("input")?;
        assert_eq!(part2(&input).unwrap(), 100876); // 38 seconds!!
        Ok(())
    }
}
