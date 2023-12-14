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
    fn tilt_north(&self) -> Map {
        let mut new_coords = IndexMap::<Coordinate, char>::new();

        for (coord, ch) in self.coordinates.iter() {
            if *ch == '#' {
                new_coords.insert(coord.clone(), ch.clone());
                continue
            }
            // we have a round rock, see how far north it can go.
            let closest_rock_at_col = new_coords
                .iter()
                .filter(|e| e.0.x == coord.x)
                .last();
            let new_y = closest_rock_at_col.map(|e| e.0.y + 1).unwrap_or(0);

            new_coords.insert(Coordinate { x: coord.x, y: new_y }, ch.clone());
        }

        Map { coordinates: new_coords, width: self.width, height: self.height }
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
    Ok(0)
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

    // #[test]
    // pub fn test_part2() -> Result<()> {
    //     let input = Input::from_lines([
    //     ]);
    //     assert_eq!(part2(&input).unwrap(), 0);
    //     Ok(())
    // }
}
