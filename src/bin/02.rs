use itertools::Itertools;

advent_of_code::solution!(2);

fn is_safe(input: &str) -> bool {
    input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .tuple_windows()
        .try_fold(0, |acc, (a, b)| {
            if acc >= 0 && (1..=3).contains(&(b - a)) {
                Ok(1)
            } else if acc <= 0 && (1..=3).contains(&(a - b)) {
                Ok(-1)
            } else {
                Err(())
            }
        })
        .is_ok()
}

pub fn part_one(input: &str) -> Option<i32> {
    input
        .lines()
        .filter(|line| is_safe(line))
        .count()
        .try_into()
        .ok()
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
