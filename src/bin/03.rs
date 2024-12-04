use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<i32> {
    let regex = Regex::new(r"(?m)mul\((\d+),(\d+)\)").unwrap();
    let total = regex
        .captures_iter(input)
        .map(|m| {
            let (_, [x, y]) = m.extract();
            let x = x.parse::<i32>().unwrap();
            let y = y.parse::<i32>().unwrap();
            x * y
        })
        .sum::<i32>();
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
