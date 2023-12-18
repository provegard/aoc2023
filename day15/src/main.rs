use std::collections::HashMap;

use anyhow::Result;
use itertools::Itertools;
use util::Input;
use regex::Regex;

fn hash(s: &str) -> u32 {
    let mut hash = 0;
    for ch in s.chars() {
        hash = (hash + ch as u32) * 17 % 256;
    }
    hash
}


fn part1(input: &Input) -> Result<u32> {
    let first_line = input.as_lines().nth(0).unwrap();
    let parts = first_line.split(",").fold(0, |acc, s| acc + hash(s));

    Ok(parts)
}

fn part2(input: &Input) -> Result<u32> {
    let first_line = input.as_lines().nth(0).unwrap();
    let instructions = first_line.split(",").collect_vec();
    let mut boxes: HashMap<u8, Vec<(String, u8)>> = HashMap::new();

    for ins in instructions {

        let re = Regex::new(r"([a-z]+)([=-])([0-9]+)?").unwrap();
        match re.captures(ins) {
            Some(caps) => {
                let label = caps[1].to_string();
                let op = caps[2].to_string();

                let box_idx = hash(&label) as u8;

                let boxx = boxes.entry(box_idx).or_insert(Vec::new());

                if op == "-" {
                    // remove
                    let position = boxx.iter().position(|v| v.0 == label);
                    if let Some(pos) = position {
                        boxx.remove(pos);
                    }
                } else if op == "=" {
                    let focal_length_r = &caps[3].parse::<u8>()?;
                    let focal_length: u8 = focal_length_r.clone();

                    // add or replace
                    let position = boxx.iter().position(|v| v.0 == label);
                    if let Some(pos) = position {
                        boxx[pos] = (label, focal_length);
                    } else {
                        boxx.push((label, focal_length));
                    }
                }

            },
            _ => panic!("Unrecognized: {}", ins),
        }
    }

    let power = boxes.iter().fold(0, |acc, (box_idx, boxx)| {
        let focusing_power = boxx.iter().enumerate().fold(0, |acc2, (lens_idx, (_, focal_length))| {
            acc2 + (1 + *box_idx as u32) * (1 + lens_idx as u32) * *focal_length as u32
        });
        acc + focusing_power
    });

    Ok(power)
}

#[cfg(test)]
mod test {
    use crate::{part1, part2, hash};
    use anyhow::Result;
    use util::Input;

    #[test]
    pub fn test_hash() -> Result<()> {
        assert_eq!(hash("HASH"), 52);
        Ok(())
    }

    #[test]
    pub fn test_part1() -> Result<()> {
        let input = Input::from_lines([
            "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7",
            ""
        ]);
        assert_eq!(part1(&input).unwrap(), 1320);
        Ok(())
    }

    #[test]
    pub fn test_part1_input() -> Result<()> {
        let input = Input::load("input")?;
        assert_eq!(part1(&input).unwrap(), 507291);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let input = Input::from_lines([
            "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7",
            ""
        ]);
        assert_eq!(part2(&input).unwrap(), 145);
        Ok(())
    }

    #[test]
    pub fn test_part2_inpit() -> Result<()> {
        let input = Input::load("input")?;
        assert_eq!(part2(&input).unwrap(), 296921);
        Ok(())
    }
}
