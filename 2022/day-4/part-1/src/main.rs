use itertools::Itertools;
use std::ops::RangeInclusive;

fn main() {
    let mut sum = 0;
    for line in include_str!("../input.txt").lines() {
        let ranges = line.split(',').map(parse_range);
        let pair = ranges
            .collect_tuple::<(RangeInclusive<i32>, RangeInclusive<i32>)>()
            .expect("Lines should be a pair of ranges");

        if pair.0.fully_contains_range(&pair.1) || pair.1.fully_contains_range(&pair.0) {
            sum += 1;
        }
    }

    println!("Result = {}", sum);
}

trait ContainsRange {
    fn fully_contains_range(&self, other: &Self) -> bool;
}

impl<T> ContainsRange for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn fully_contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }
}

fn parse_range(part: &str) -> RangeInclusive<i32> {
    part.split('-')
        .map(|n| n.parse().expect("Range parts must be integers"))
        .collect_tuple::<(i32, i32)>()
        .map(|(s, e)| s..=e)
        .expect("Range didn't have a start and end")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_range_test() {
        assert_eq!(false, (2..=4).fully_contains_range(&(6..=8)));
        assert_eq!(false, (2..=3).fully_contains_range(&(4..=5)));
        assert_eq!(false, (5..=7).fully_contains_range(&(7..=9)));
        assert_eq!(true, (2..=8).fully_contains_range(&(3..=7)));

        assert_eq!(false, (6..=6).fully_contains_range(&(4..=6)));
        assert_eq!(true, (4..=6).fully_contains_range(&(6..=6)));

        assert_eq!(false, (2..=6).fully_contains_range(&(4..=8)));
    }
}
