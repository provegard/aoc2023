use anyhow::Result;
use itertools::Itertools;
use util::Input;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Coord { x: usize, y: usize }

#[derive(Debug)]
struct Pattern {
    rows: Vec<String>,
    columns: Vec<String>
    //rocks: Vec<Coord>
}

fn to_patterns(input: &Input) -> Vec<Pattern> {
    let lines = input.as_lines().map(|s| s.to_string()).collect_vec();
    let chunks = lines.split(|line| *line == "").map(|chunk| chunk.to_vec()).collect_vec();

    let v = chunks.iter().map(|chunk| {
        let width = chunk.first().unwrap().len();

        let columns = (0..width).map(|i| {
            let chars: String = chunk.iter().map(|l| match l.chars().nth(i) {
                Some(ch) => ch,
                None => panic!("No char at index {} in {}", i, l),
            }).collect();
            chars
        }).collect_vec();
    
        Pattern { rows: chunk.to_vec(), columns }
    }).collect_vec();
    v
}

fn count_before(items: &Vec<String>) -> usize {
    let all_mirrored_idx = items.iter().enumerate().filter(|(idx, item)| {
        match items.get(idx + 1) {
            Some(other) => **item == *other,
            None => false,
        }
    }).map(|opt| opt.0).collect_vec();

    let max_count = all_mirrored_idx.iter().map(|mirrored_idx| {
        // Step "outwards" from mirrored_idx and test that the mirror item matches or doesn't exist.
        let all_match = (0..*mirrored_idx).rev().all(|idx| {
            let d = mirrored_idx - idx;
            let mir_idx = mirrored_idx + d + 1;
            let item_opt = items.get(idx);
            let mir_opt = items.get(mir_idx);
            match (item_opt, mir_opt) {
                (Some(item), Some(mirrored)) => *mirrored == *item,
                (Some(_), None) => true, // mirror item doesn't exist
                _ => false,
            }
        });

        if all_match { mirrored_idx + 1 } else { 0 }
    }).max();

    max_count.unwrap_or(0)
}

fn cols_to_the_left(pattern: &Pattern) -> usize {
    count_before(&pattern.columns)
}

fn rows_above(pattern: &Pattern) -> usize {
    count_before(&pattern.rows)
}

fn value_of(pattern: &Pattern) -> usize {
    let rc = rows_above(pattern);
    let cc = cols_to_the_left(pattern);

    rc * 100 + cc
}

fn part1(input: &Input) -> Result<u32> {
    let patterns = to_patterns(input);
    let res = patterns.iter().fold(0, |acc, p| acc + value_of(p));
    Ok(res as u32)
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
        assert_eq!(part1(&input).unwrap(), 405);
        Ok(())
    }

    #[test]
    pub fn test_part1_ex1() -> Result<()> {
        let input = Input::load("ex1")?;
        assert_eq!(part1(&input).unwrap(), 1000);
        Ok(())
    }

    #[test]
    pub fn test_part1_ex2() -> Result<()> {
        let input = Input::load("ex2")?;
        assert_eq!(part1(&input).unwrap(), 100);
        Ok(())
    }

    #[test]
    pub fn test_part1_input() -> Result<()> {
        let input = Input::load("input")?;
        let res = part1(&input).unwrap();
        assert_eq!(res, 36015);
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
