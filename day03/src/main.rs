use anyhow::Result;
use itertools::Itertools;

use util::Input;

fn main() -> Result<()> {
    let input = Input::load("day03/input")?;

    println!("Part 1:");
    println!("{}", part1(&input)?);

    println!("Part 2:");
    println!("{}", part2(&input)?);
    Ok(())
}

struct Grid {
    lines: Vec<String>,
    row_count: usize,
    col_count: usize,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Clone)]
#[derive(Hash)]
struct Coord(usize, usize);

fn to_grid(input: &Input) -> Grid {
    let lines: Vec<_> = input.as_lines().map(|s| s.to_string()).collect();
    let row_count = lines.len();
    assert!(row_count > 0);
    let col_count = lines.get(0).map(|s| s.len()).unwrap();

    return Grid {
        lines,
        row_count,
        col_count,
    }
}

fn char_at(grid: &Grid, coord: &Coord) -> char {
    match grid.lines.get(coord.0).map(|s| s.chars().skip(coord.1).next()).flatten() {
        Some(ch) => ch,
        None => panic!("Failed to get char at coordinate {:?}", coord)
    }
}

fn is_symbol(ch: char) -> bool {
    ch != '.' && !ch.is_digit(10)
}

fn surrounding_coords(grid: &Grid, coord: &Coord) -> Vec<Coord> {
    let mut vec: Vec<Coord> = Vec::new();

    let rc = grid.row_count as isize;
    let cc = grid.col_count as isize;

    for rd in -1..=1 {
        for cd in -1..=1 {
            if !(rd == 0 && cd == 0) {
                let new_r = (coord.0 as isize) + rd;
                let new_c = (coord.1 as isize) + cd;

                if new_r >= 0 && new_c >= 0 && new_r < rc && new_c < cc {
                    vec.push(Coord(new_r as usize, new_c as usize))
                }
            }
        }
    }

    vec
}

fn all_coords(grid: &Grid) -> Vec<Coord> {
    let vec: Vec<Coord> = (0..grid.row_count).flat_map(|r| (0..grid.col_count).map(move |c| Coord(r, c))).collect();
    vec
}

fn all_digit_coords(grid: &Grid) -> Vec<Coord> {
    let vec: Vec<_> = all_coords(grid).into_iter().filter(|c| char_at(grid, c).is_digit(10)).collect();
    vec
}

fn find_number_groups(grid: &Grid) -> Vec<Vec<Coord>> {
    let mut groups: Vec<Vec<Coord>> = Vec::new();
    let mut current_group: Option<Vec<Coord>> = None;
    for coord in all_digit_coords(grid) {
        match current_group {
            Some(ref mut grp) if coord.1 > 0 && grp.last().unwrap().1 == coord.1 - 1 => grp.push(coord),
            Some(ref grp) => {
                // new group
                groups.push(grp.to_vec());
                let grp: Vec<Coord> = vec![coord];
                current_group = Some(grp);
            },
            None => {
                let grp: Vec<Coord> = vec![coord]; // TODO: Fix duplication
                current_group = Some(grp);
            },
        }
    }

    if let Some(ref grp) = current_group {
        groups.push(grp.to_vec());
    }

    groups
}

fn part1(input: &Input) -> Result<u32> {
    let grid = to_grid(input);
    let mut sum = 0;
    for grp in find_number_groups(&grid) {
        let surrounding: Vec<_> = grp.iter().flat_map(|c| surrounding_coords(&grid, c)).unique().collect();
        let has_symbol = surrounding.iter().any(|c| is_symbol(char_at(&grid, c)));

        if has_symbol {
            let number = grp.iter().fold(0, |acc, c| 10 * acc + char_at(&grid, c).to_digit(10).unwrap());
            sum += number;
        }
    }

    Ok(sum)
}

fn part2(input: &Input) -> Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use crate::{part1, part2, surrounding_coords, to_grid, Coord};
    use anyhow::Result;
    use util::Input;

    #[test]
    pub fn test_surrounding_coords() -> Result<()> {
        let input = Input::from_lines([
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]);
        let grid = to_grid(&input);
        assert_eq!(surrounding_coords(&grid, &Coord(4, 0)), vec![
            Coord(3, 0),
            Coord(3, 1),
            Coord(4, 1),
            Coord(5, 0),
            Coord(5, 1),
        ]);
        Ok(())
    }

    #[test]
    pub fn test_part1() -> Result<()> {
        let input = Input::from_lines([
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]);
        assert_eq!(part1(&input).unwrap(), 4361);
        Ok(())
    }

    #[test]
    pub fn test_part1_input() -> Result<()> {
        let input = Input::load("input")?;
        assert_eq!(part1(&input).unwrap(), 527364);
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
