use anyhow::{Result};
use std::fs::File;
use std::io::{self, BufRead};

fn read_input() -> Result<Vec<String>> {
    let path = "day01/input";
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let lines: Vec<_> = reader.lines().collect::<Result<_, _>>()?;
    return Ok(lines);
}

fn digit_value(str: &String, idx: usize) -> u32 {
    let ch = str.as_bytes()[idx] as char;
    ch.to_digit(10).unwrap()
}

fn main() -> Result<()> {
    let input = read_input()?;

    println!("Part 1:");
    println!("{}", part1(&input)?);

    println!("Part 2:");
    println!("{}", part2(&input)?);
    Ok(())
}


fn part1(input: &Vec<String>) -> Result<u32> {
    let value = input.iter().fold(0, |acc, s| {
        let first_digit_idx = s.find(|c: char| c.is_digit(10)).unwrap();
        let last_digit_idx = s.rfind(|c: char| c.is_digit(10)).unwrap();

        acc + 10 * digit_value(&s, first_digit_idx) + digit_value(&s, last_digit_idx)
    } );
    Ok(value)
}

fn part2(input: &Vec<String>) -> Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};
    use anyhow::Result;
    use util::Input;

    #[test]
    pub fn test_part1() -> Result<()> {
        let input = Input::from_lines([
        ]);
        assert_eq!(part1(&input).unwrap(), 0);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let input = Input::from_lines([
        ]);
        assert_eq!(part2(&input).unwrap(), 0);
        Ok(())
    }
}
