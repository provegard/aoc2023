use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead};

static DIGIT_NAMES: &[&str] = &["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn read_input(path: &str) -> Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let lines: Vec<_> = reader.lines().collect::<Result<_, _>>()?;
    return Ok(lines);
}

fn main() -> Result<()> {
    let input = read_input("day01/input")?;

    println!("Part 1:");
    println!("{}", part1(&input)?); // 56397

    println!("Part 2:");
    println!("{}", part2(&input)?); // 55701
    Ok(())
}

fn digits_in_including_names(str: &String) -> Vec<u32> {
    str
        .chars()
        .enumerate()
        .map(|(index, ch)| {
            // Try to parse the char as digit. If it fails, try to find if the current position contains a digit by name.
            ch.to_digit(10).or({
                DIGIT_NAMES.iter().enumerate().find(|&(_i, name)| {
                    str[index..].find(name) == Some(0)
                })
                .map(|(name_index, _s)| name_index as u32)
            })
        })
        .filter_map(|x| x)
        .collect()
}

fn digits_in(str: &String) -> Vec<u32> {
    str
        .chars()
        .map(|c| c.to_digit(10))
        .filter_map(|x| x)
        .collect()
}

fn part<F>(input: &Vec<String>, digit_finder: F) -> Result<u32> where F: Fn(&String) -> Vec<u32> {
    let value = input.iter().fold(0, |acc, s| {
        let digits = digit_finder(s);
        let first_digit = digits.first().unwrap();
        let last_digit = digits.last().unwrap();

        acc + 10 * first_digit + last_digit
    });
    Ok(value)
}

fn part1(input: &Vec<String>) -> Result<u32> {
    part(input, digits_in)
}

fn part2(input: &Vec<String>) -> Result<u32> {
    part(input, digits_in_including_names)
}

#[cfg(test)]
mod test {
    use crate::{part1, part2, read_input};
    use anyhow::Result;

    #[test]
    pub fn test_part1() -> Result<()> {
        let input: Vec<String> = vec!(
            String::from("1abc2"),
            String::from("pqr3stu8vwx"),
            String::from("a1b2c3d4e5f"),
            String::from("treb7uchet"),
        );
        assert_eq!(part1(&input).unwrap(), 142);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let input: Vec<String> = vec!(
            String::from("two1nine"),
            String::from("eightwothree"),
            String::from("abcone2threexyz"),
            String::from("xtwone3four"),
            String::from("4nineeightseven2"),
            String::from("zoneight234"),
            String::from("7pqrstsixteen"),
        );
        assert_eq!(part2(&input).unwrap(), 281);
        Ok(())
    }

    #[test]
    pub fn test_part1_input() -> Result<()> {
        let input = read_input("input")?;
        assert_eq!(part1(&input).unwrap(), 56397);
        Ok(())
    }

    #[test]
    pub fn test_part2_input() -> Result<()> {
        let input = read_input("input")?;
        assert_eq!(part2(&input).unwrap(), 55701);
        Ok(())
    }
}
