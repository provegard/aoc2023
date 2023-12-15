use anyhow::Result;

use util::Input;

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
    Ok(0)
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

    // #[test]
    // pub fn test_part2() -> Result<()> {
    //     let input = Input::from_lines([
    //     ]);
    //     assert_eq!(part2(&input).unwrap(), 0);
    //     Ok(())
    // }
}
