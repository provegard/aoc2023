use anyhow::Result;
use itertools::Itertools;

use util::Input;

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
        .filter(|tup| !(tup.0 == 0 && tup.1 == 0))
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
            _ => {
                if let Some(ref grp) = current_group {
                    // Stash the current group
                    groups.push(grp.to_vec());
                }
                // New group
                let grp: Vec<Coord> = vec![coord];
                current_group = Some(grp);
            }
        }
    }

    // Stash final group, if any
    if let Some(ref grp) = current_group {
        groups.push(grp.to_vec());
    }

    groups
}

fn part1(input: &Input) -> Result<u32> {
    let grid = to_grid(input);

    let sum: u32 = find_number_groups(&grid)
        .iter()
        .map(|grp| {
            let surrounding: Vec<_> = grp.iter().flat_map(|c| surrounding_coords(&grid, c)).unique().collect();
            let has_symbol = surrounding.iter().any(|c| is_symbol(char_at(&grid, c)));
            let number = grp.iter().fold(0, |acc, c| 10 * acc + char_at(&grid, c).to_digit(10).unwrap());

            if has_symbol { number } else { 0 }
        })
        .sum();

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
