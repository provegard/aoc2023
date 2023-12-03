use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashSet};
use util::{Input, group_by};

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

    return Grid { lines, row_count, col_count }
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
    let rc = grid.row_count as isize;
    let cc = grid.col_count as isize;

    let vec: Vec<_> = (-1..=1).flat_map(|r| (-1..=1).map(move |c| (r, c)))
        .filter(|tup| !(tup.0 == 0 && tup.1 == 0)) // skip the coordinate itself
        .filter_map(|tup| {
            let new_r = (coord.0 as isize) + tup.0;
            let new_c = (coord.1 as isize) + tup.1;

            if new_r >= 0 && new_c >= 0 && new_r < rc && new_c < cc {
                Some(Coord(new_r as usize, new_c as usize))
            } else { None }
        })
        .collect();

    vec
}

fn all_coords<'a>(grid: &'a Grid) -> impl Iterator<Item = Coord> + 'a {
    (0..grid.row_count).flat_map(|r| (0..grid.col_count).map(move |c| Coord(r, c)))
}

fn all_digit_coords<'a>(grid: &'a Grid) -> impl Iterator<Item = Coord> + 'a {
    all_coords(grid).filter(|c| char_at(grid, c).is_digit(10))
}

fn find_number_runs(grid: &Grid) -> Vec<Vec<Coord>> {
    let mut runs: Vec<Vec<Coord>> = Vec::new();
    let mut current_run: Option<Vec<Coord>> = None;
    for coord in all_digit_coords(grid) {
        match current_run {
            // If we have a current run, and the digit in question immediately succeeds the last digit in that run,
            // then add it to the run.
            Some(ref mut run) if coord.1 > 0 && run.last().unwrap().1 == coord.1 - 1 => run.push(coord),
            _ => {
                // If we have a current run, stash it so we can begin a new one.
                if let Some(ref grp) = current_run {
                    // Stash the current group
                    runs.push(grp.to_vec());
                }
                // Begin a new run.
                current_run = Some(vec![coord]);
            }
        }
    }

    // Stash final run, if any.
    if let Some(ref grp) = current_run {
        runs.push(grp.to_vec());
    }

    runs
}

fn run_to_number(grid: &Grid, run: &Vec<Coord>) -> u32 {
    run.iter().fold(0, |acc, c| 10 * acc + char_at(&grid, c).to_digit(10).unwrap())
}

fn part1(input: &Input) -> Result<u32> {
    let grid = to_grid(input);

    let sum: u32 = find_number_runs(&grid)
        .iter()
        .filter_map(|run| {
            let surrounding: Vec<_> = run.iter().flat_map(|c| surrounding_coords(&grid, c)).unique().collect();
            let first_symbol = surrounding.iter().find(|c| is_symbol(char_at(&grid, c)));
            let number = run_to_number(&grid, run);

            first_symbol.map(|_| number)
        })
        .sum();

    Ok(sum)
}

fn part2(input: &Input) -> Result<u32> {
    let grid = to_grid(input);
    let gear_coords: HashSet<_> = all_coords(&grid).filter(|c| char_at(&grid, c) == '*').collect();

    let numbers_with_gear_coord: Vec<_> = find_number_runs(&grid)
        .iter()
        .filter_map(|run| {
            let surrounding: Vec<_> = run.iter().flat_map(|c| surrounding_coords(&grid, c)).unique().collect();
            let gear_coord = surrounding.iter().find(|c| gear_coords.contains(c));
            let number = run_to_number(&grid, run);

            gear_coord.map(|c| (c.clone(), number))
        })
        .collect();

    // Group by coord
    let groups = group_by(&numbers_with_gear_coord, |tup| &tup.0, |tup| &tup.1);

    let sum: u32 = groups
        .iter()
        .filter(|e| e.1.len() == 2) // limit to groups with exactly two numbers
        .map(|e| e.1.iter().fold(1, |acc, x| acc * *x)) // multiply the numbers
        .sum();

    Ok(sum)
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
    
    #[test]
    pub fn test_part2() -> Result<()> {
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
        assert_eq!(part2(&input).unwrap(), 467835);
        Ok(())
    }

    #[test]
    pub fn test_part2_input() -> Result<()> {
        let input = Input::load("input")?;
        assert_eq!(part2(&input).unwrap(), 79026871);
        Ok(())
    }
}
