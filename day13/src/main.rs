use anyhow::Result;
use itertools::Itertools;
use util::Input;
use std::fmt;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Coord { x: usize, y: usize }

#[derive(Debug)]
struct Pattern {
    rows: Vec<String>,
    columns: Vec<String>
}

#[derive(PartialEq, Debug, Hash, Eq, Clone, Copy)]
enum Orientation {
    Horizontal,
    Vertical
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct ReflectionLine {
    lines_before: usize,
    line_type: Orientation
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.rows.iter() {
            writeln!(f, "{}", row)?;
        }
        Ok(())
    }
}

fn flip_char(ch: char) -> char {
    if ch == '#' { '.' } else { '#' }
}

fn flip(str: &String, index: usize) -> String {
    let new_str: String = str.chars().enumerate().map(|(idx, ch)| if idx == index { flip_char(ch) } else { ch }).collect();
    new_str
}

fn rows_to_cols(rows: &Vec<String>) -> Vec<String> {
    let width = rows.first().unwrap().len();

    let columns = (0..width).map(|i| {
        let chars: String = rows.iter().map(|l| match l.chars().nth(i) {
            Some(ch) => ch,
            None => panic!("No char at index {} in {}", i, l),
        }).collect();
        chars
    }).collect_vec();

    columns
}

fn new_pattern(rows: &Vec<String>) -> Pattern {
    let columns = rows_to_cols(rows);
    Pattern { rows: rows.to_vec(), columns }
}

fn to_patterns(input: &Input) -> Vec<Pattern> {
    let lines = input.as_lines().map(|s| s.to_string()).collect_vec();
    let chunks = lines.split(|line| *line == "").map(|chunk| chunk.to_vec()).collect_vec();

    let v = chunks.iter().map(|chunk| {
        let p = new_pattern(chunk);
        p
    }).collect_vec();
    v
}

fn has_reflection_at_index(items: &Vec<String>, mirrored_idx: usize) -> bool {
    for idx in (0..=mirrored_idx).rev() {
        let d = mirrored_idx - idx;
        let mir_idx = mirrored_idx + d + 1;
        let item_opt = items.get(idx);
        let mir_opt = items.get(mir_idx);
        match (item_opt, mir_opt) {
            (Some(item), Some(mirrored)) if *mirrored == *item => {},
            (Some(_), None) => {}, // mirror item doesn't exist, ignore
            _ => return false, // mismatch, no reflection
        }
    }

    true
}

fn reflection_lines_for_items(items: &Vec<String>, line_type: Orientation) -> Vec<ReflectionLine> {
    let v = (0..(items.len()-1)).into_iter().filter_map(|mirrored_idx| {
        let has_reflection = has_reflection_at_index(items, mirrored_idx);
        if has_reflection {
            Some(ReflectionLine { lines_before: mirrored_idx + 1, line_type })
        } else { None }
    }).collect_vec();
    v
}

fn line_value(rl: &ReflectionLine) -> usize {
    if rl.line_type == Orientation::Horizontal { rl.lines_before * 100 } else { rl.lines_before }
}

fn reflection_lines(pattern: &Pattern) -> Vec<ReflectionLine> {
    let horizontal = reflection_lines_for_items(&pattern.rows, Orientation::Horizontal);
    let vertical = reflection_lines_for_items(&pattern.columns, Orientation::Vertical);

    let v = horizontal.into_iter().chain(vertical.into_iter()).collect_vec();
    v
}

fn value_of(pattern: &Pattern) -> usize {
    let v = reflection_lines(pattern);
    assert_eq!(1, v.len());
    line_value(v.first().unwrap())
}

fn pattern_variants(pattern: &Pattern) -> Vec<Pattern> {
    fn pattern_with_flip(pattern: &Pattern, r: usize, c: usize) -> Pattern {
        let new_rows = pattern.rows.iter().enumerate().map(|(r_idx, row)| {
            if r_idx == r {
                flip(row, c)
            } else { row.to_string() }
        }).collect_vec();
        new_pattern(&new_rows)
    }

    let width = pattern.rows.get(0).unwrap().len();
    let v = (0..pattern.rows.len()).flat_map(|r| {
        (0..width).map(move |c| pattern_with_flip(pattern, r, c))
    }).collect_vec();
    v
}

fn part1(input: &Input) -> Result<u32> {
    let patterns = to_patterns(input);
    let res = patterns.iter().fold(0, |acc, p| acc + value_of(p));
    Ok(res as u32)
}

fn part2(input: &Input) -> Result<u32> {
    let patterns = to_patterns(input);
    let res = patterns.iter().fold(0, |acc, p| {
        let original_lines: HashSet<ReflectionLine> = reflection_lines(p).into_iter().collect();
        let variants = pattern_variants(p);
        let fixed = variants.iter().filter_map(|v| {
            let variant_lines: HashSet<ReflectionLine> = reflection_lines(v).into_iter().collect();
            let diff_lines = variant_lines.difference(&original_lines).map(|rl| rl.clone()).collect_vec();
            if diff_lines.len() > 0 { Some(diff_lines) } else { None }
        }).next();

        let val = match fixed {
            Some(rls) if rls.len() == 1 => {
                line_value(rls.first().unwrap())
            },
            Some(rls) => panic!("Found {} different reflection lines", rls.len()),
            None => panic!("No pattern fix found for:\n{}", p),
        };

        acc + val
    });

    Ok(res as u32)
}

#[cfg(test)]
mod test {
    use crate::{part1, part2, reflection_lines, to_patterns, ReflectionLine, Orientation};
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

    #[test]
    pub fn test_part2() -> Result<()> {
        let input = Input::load("example")?;
        assert_eq!(part2(&input).unwrap(), 400);
        Ok(())
    }

    #[test]
    pub fn test_part2_input() -> Result<()> {
        let input = Input::load("input")?;
        assert_eq!(part2(&input).unwrap(), 35335);
        Ok(())
    }

    #[test]
    pub fn test_part2_ex3() -> Result<()> {
        let input = Input::load("ex3")?;
        assert_eq!(part2(&input).unwrap(), 600);
        Ok(())
    }

    #[test]
    pub fn test_reflection_lines_ex3() -> Result<()> {
        let input = Input::load("ex3")?;
        let patterns = to_patterns(&input);
        let pattern = patterns.first().unwrap();
        let rls = reflection_lines(pattern);
        assert_eq!(rls, vec![ReflectionLine { lines_before: 12, line_type: Orientation::Horizontal }]);
        Ok(())
    }

    #[test]
    pub fn test_reflection_lines_ex4() -> Result<()> {
        let input = Input::load("ex4")?;
        let patterns = to_patterns(&input);
        let pattern = patterns.first().unwrap();
        let rls = reflection_lines(pattern);
        assert_eq!(rls, vec![
            ReflectionLine { lines_before: 6, line_type: Orientation::Horizontal },
            ReflectionLine { lines_before: 12, line_type: Orientation::Horizontal }
        ]);
        Ok(())
    }

    #[test]
    pub fn test_reflection_lines_ex5() -> Result<()> {
        let input = Input::load("ex5")?;
        let patterns = to_patterns(&input);
        let pattern = patterns.first().unwrap();
        let rls = reflection_lines(pattern);
        assert_eq!(rls, vec![ReflectionLine { lines_before: 12, line_type: Orientation::Vertical }]);
        Ok(())
    }

    #[test]
    pub fn test_reflection_lines_ex6() -> Result<()> {
        let input = Input::load("ex6")?;
        let patterns = to_patterns(&input);
        let pattern = patterns.first().unwrap();
        let rls = reflection_lines(pattern);
        assert_eq!(rls, vec![
            ReflectionLine { lines_before: 1, line_type: Orientation::Vertical },
            ReflectionLine { lines_before: 12, line_type: Orientation::Vertical }
        ]);
        Ok(())
    }
}
