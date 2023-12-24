use anyhow::Result;
use itertools::Itertools;
use util::Input;

struct Record {
    damaged: String,
    groups: Vec<usize>,
}

fn parse_line(line: &str) -> Record {
    match line.split_ascii_whitespace().collect_vec()[..] {
        [a, b] => {
            let groups = b.split(",").map(|s| s.parse::<usize>().unwrap()).collect_vec();
            Record { damaged: a.to_string(), groups }
        },
        _ => panic!("Unexpected: {}", line),
    }
}

fn matches_damaged(damaged: &str, candidate: &str) -> bool {
    candidate.len() == damaged.len() && damaged.chars().zip(candidate.chars()).all(|(dc, cc)| {
        dc == '?' || dc == cc
    })
}

fn spaces_fit(record: &Record, spaces: &Vec<usize>) -> bool {
    assert!(spaces.len() == record.groups.len() + 2 - 1);
    let cap = spaces.iter().fold(0, |acc, s| acc + s) + record.groups.iter().fold(0, |acc, g| acc + g);
    let mut str = String::with_capacity(cap);

    // There must be a better way to do this...
    for (i, s) in spaces.iter().enumerate() {
        // add the space
        let space_chars = '.'.to_string().repeat(*s);
        str.push_str(&space_chars);
        if let Some(g) = record.groups.get(i) {
            let group_chars = '#'.to_string().repeat(*g);
            str.push_str(&group_chars);
        } 
    }

    matches_damaged(&record.damaged, &str)
}

fn combos(record: &Record) -> u32 {
    let group_count = record.groups.len();
    let group_size_tot = record.groups.iter().fold(0, |acc, g| acc + g);
    let damaged_len = record.damaged.len();
    let total_space_size = damaged_len - group_size_tot;
    let required_space_count = group_count - 1; // space between groups
    let space_count = 2 + group_count - 1; // space between groups + (optional) spaces at the edges
    let surplus = total_space_size - required_space_count;

    // Immutable linked list ought to be more efficient...
    fn combos_inner(space_count: usize, space_idx: usize, surplus_left: usize, comb: Vec<usize>) -> Vec<Vec<usize>> {
        let min = if space_idx == 0 || space_idx == space_count - 1 { 0 } else { 1 };
        let max = min + surplus_left;

        if space_idx == space_count - 1 {
            // last space
            let v3 = (min..=max).map(|x| {
                let v2 = comb.iter().chain(vec![x].iter()).map(|x| x.clone()).collect_vec();
                v2
            }).collect_vec();
            v3
        } else {
            // more spaces after this
            let v3 = (min..=max).flat_map(|x| {
                let v2 = comb.iter().chain(vec![x].iter()).map(|x| x.clone()).collect_vec();
                let surplus_used = x - min;
                combos_inner(space_count, space_idx + 1, surplus_left - surplus_used, v2)
            }).collect_vec();
            v3
        }
    }
    
    let cc = combos_inner(space_count, 0, surplus, vec![]);

    cc.iter().filter(|v| spaces_fit(record, v)).count() as u32
}

fn part1(input: &Input) -> Result<u32> {
    let res = input.as_lines().fold(0, |acc, line| {
        let rec = parse_line(line);
        acc + combos(&rec)
    });
    Ok(res)
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
    pub fn test_part1_1() -> Result<()> {
        let input = Input::from_lines([
            "???.### 1,1,3"
        ]);
        assert_eq!(part1(&input).unwrap(), 1);
        Ok(())
    }

    #[test]
    pub fn test_part1_2() -> Result<()> {
        let input = Input::from_lines([
            ".??..??...?##. 1,1,3"
        ]);
        assert_eq!(part1(&input).unwrap(), 4);
        Ok(())
    }

    #[test]
    pub fn test_part1_3() -> Result<()> {
        let input = Input::from_lines([
            "?###???????? 3,2,1"
        ]);
        assert_eq!(part1(&input).unwrap(), 10);
        Ok(())
    }

    #[test]
    pub fn test_part1_4() -> Result<()> {
        let input = Input::from_lines([
            "?##??? 2"
        ]);
        assert_eq!(part1(&input).unwrap(), 1);
        Ok(())
    }

    #[test]
    pub fn test_part1_5() -> Result<()> {
        let input = Input::from_lines([
            "???#??? 2"
        ]);
        assert_eq!(part1(&input).unwrap(), 2);
        Ok(())
    }

    #[test]
    pub fn test_part1_6() -> Result<()> {
        let input = Input::from_lines([
            "??##???????##?? 2,2"
        ]);
        assert_eq!(part1(&input).unwrap(), 1);
        Ok(())
    }

    #[test]
    pub fn test_part1_7() -> Result<()> {
        let input = Input::from_lines([
            "?#.???? 2,1"
        ]);
        assert_eq!(part1(&input).unwrap(), 4);
        Ok(())
    }

    #[test]
    pub fn test_part1_example() -> Result<()> {
        let input = Input::load("example")?;
        assert_eq!(part1(&input).unwrap(), 21);
        Ok(())
    }
    
    #[test]
    pub fn test_part1_input() -> Result<()> {
        let input = Input::load("input")?;
        assert_eq!(part1(&input).unwrap(), 6852);
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
