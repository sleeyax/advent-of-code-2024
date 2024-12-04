use fancy_regex::Regex;

advent_of_code::solution!(3);

fn mul(input: &str, regex: Regex) -> Option<i32> {
    let total = regex
        .captures_iter(input)
        .map(|m| {
            let captures = m.unwrap();
            let x = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let y = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
            x * y
        })
        .sum::<i32>();
    Some(total)
}

pub fn part_one(input: &str) -> Option<i32> {
    mul(input, Regex::new(r"(?m)mul\((\d+),(\d+)\)").unwrap())
}

pub fn part_two(input: &str) -> Option<i32> {
    // TODO: this results in a wrong answer... idk why...
    mul(
        // the start of the input is enabled, so prepend a 'do()' instruction so that the regex matches.
        &format!("{}{}", "do()", input),
        Regex::new(r"(?m)do\(\)(?:(?!don't\(\)).)*?mul\((\d+),(\d+)\)").unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
