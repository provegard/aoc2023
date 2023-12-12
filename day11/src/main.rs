use itertools::Itertools;
use anyhow::Result;
use util::Input;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Coord { x: usize, y: usize }

#[derive(Debug)]
struct Image {
    galaxies: Vec<Coord>
}

impl Coord {
    fn manhattan(&self, other: &Coord) -> usize {
        other.x.abs_diff(self.x) + other.y.abs_diff(self.y)
    }
}

fn to_image(input: &Input) -> Image {
    let lines: Vec<_> = input.as_lines().map(|s| s.to_string()).collect();

    let galaxies = lines.iter().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().filter_map(move |(x, ch)| if ch == '#' { Some(Coord { x, y }) } else { None })
    }).collect_vec();

    return Image { galaxies }
}

fn expand(image: &Image) -> Image {
    fn expand_vert(image: &Image) -> Image {
        // make y distances larger
        // find all empty rows
        let max_y = image.galaxies.iter().map(|c| c.y).max().unwrap_or_else(|| panic!("No max Y"));
        let empty_ys = (0..=max_y).rev().filter(|y| {
            let first_gs_at_y = image.galaxies.iter().find(|g| g.y == *y);
            first_gs_at_y.is_none()
        }).collect_vec();

        let galaxies = image.galaxies.to_vec();
        let new_gs = empty_ys.iter().fold(galaxies, |old_gs, empty_y| {
            // add 1 to all y coordinates > empty_y
            let below = old_gs.iter().filter(|g| g.y < *empty_y).map(|c| c.clone()).collect_vec();
            let above = old_gs.iter().filter_map(|g| if g.y > *empty_y { Some(Coord { x: g.x, y: g.y + 1 }) } else { None }).collect_vec();

            let v = below.into_iter().chain(above.into_iter()).collect_vec();
            v
        });
        Image { galaxies: new_gs }
    }

    fn expand_horiz(image: &Image) -> Image { 
        // make x distances larger
        // find all empty cols
        let max_x = image.galaxies.iter().map(|c| c.x).max().unwrap_or_else(|| panic!("No max X"));
        let empty_xs = (0..=max_x).rev().filter(|x| {
            let first_gs_at_x = image.galaxies.iter().find(|g| g.x == *x);
            first_gs_at_x.is_none()
        }).collect_vec();

        let galaxies = image.galaxies.to_vec();
        let new_gs = empty_xs.iter().fold(galaxies, |old_gs, empty_x| {
            // add 1 to all x coordinates > empty_x
            let below = old_gs.iter().filter(|g| g.x < *empty_x).map(|c| c.clone()).collect_vec();
            let above = old_gs.iter().filter_map(|g| if g.x > *empty_x { Some(Coord { x: g.x + 1, y: g.y }) } else { None }).collect_vec();

            let v = below.into_iter().chain(above.into_iter()).collect_vec();
            v
        });
        Image { galaxies: new_gs }
    }

    let i1 = expand_vert(image);
    expand_horiz(&i1)
}

fn galaxy_pairs<'a>(image: &'a Image) -> Vec<(&'a Coord, &'a Coord)> {
    let mut pairs: Vec<(&'a Coord, &'a Coord)> = Vec::new();
    for (idx, g) in image.galaxies.iter().enumerate() {
        for g2 in image.galaxies.iter().skip(idx + 1) {
            pairs.push((g, g2))
        }
    }
    pairs
}

fn part1(input: &Input) -> Result<u32> {
    let image = to_image(input);
    let expanded = expand(&image);
    let result: usize = galaxy_pairs(&expanded).iter().map(|(a, b)| a.manhattan(b)).sum();
    Ok(result as u32)
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
        assert_eq!(part1(&input).unwrap(), 374);
        Ok(())
    }

    #[test]
    pub fn test_part1_input() -> Result<()> {
        let input = Input::load("input")?;
        assert_eq!(part1(&input).unwrap(), 9605127);
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
