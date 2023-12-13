use anyhow::{anyhow, Result};
use itertools::Itertools;
use tailcall::tailcall;

use util::Input;

struct Record {
    damaged: String,
    groups: Vec<u32>,
}

fn parse_line(line: &String) -> Record {
    match line.split_ascii_whitespace().collect_vec()[..] {
        [a, b] => {
            let groups = b.split(",").map(|s| s.parse::<u32>().unwrap()).collect_vec();
            Record { damaged: a.to_string(), groups }
        },
        _ => panic!("Unexpected: {}", line),
    }
}

fn strs_match(damaged: &String, candidate: &String) -> bool {
    damaged.chars().zip(candidate.chars()).all(|(dc, cc)| {
        dc == '?' || dc == cc
    })
}

fn combos(record: &Record) -> Vec<String> {
    #[tailcall]
    fn combos_inner(r: &Record, str: String) -> Vec<String> {
        if str.len() == r.damaged.len() {
            if strs_match(&r.damaged, &str) { vec![str] } else { vec![] }
        } else {
            // recurse
            
        }
    }

}

fn main() -> Result<()> {
    let input = Input::load("day12/input")?;

    println!("Part 1:");
    println!("{}", part1(&input)?);

    println!("Part 2:");
    println!("{}", part2(&input)?);
    Ok(())
}


fn part1(input: &Input) -> Result<u32> {
    Ok(0)
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
