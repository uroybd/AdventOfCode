use num::Bounded;
use num_traits::Num;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Range<T: Num + Copy + PartialOrd + Ord + Bounded>(T, T);

impl<T: Num + Copy + PartialOrd + Ord + Bounded> Range<T> {
    pub fn new(start: T, end: T) -> Self {
        Range(start, end)
    }

    pub fn min_stop(&self) -> T {
        self.0.min(self.1)
    }

    pub fn max_stop(&self) -> T {
        self.0.max(self.1)
    }

    pub fn from_string(s: &str, delimiter: char) -> anyhow::Result<Self>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::error::Error + 'static,
    {
        let parts: Vec<&str> = s.split(delimiter).collect();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!("Invalid range string"));
        }
        let start: T = parts[0]
            .trim()
            .parse()
            .map_err(|e| anyhow::anyhow!("Failed to parse start: {}", e))?;
        let end: T = parts[1]
            .trim()
            .parse()
            .map_err(|e| anyhow::anyhow!("Failed to parse end: {}", e))?;
        Ok(Range::new(start, end))
    }

    pub fn contains(&self, value: T) -> bool {
        value >= self.min_stop() && value <= self.max_stop()
    }

    pub fn length(&self) -> T {
        self.max_stop() - self.min_stop()
    }

    pub fn mergeable_with(&self, other: &Range<T>) -> bool {
        let min_self = self.min_stop();
        let max_self = self.max_stop();
        let min_other = other.min_stop();
        let max_other = other.max_stop();
        (min_self >= min_other && min_self <= max_other)
            || (max_self >= min_other && max_self <= max_other)
    }

    pub fn merge(&self, other: &Range<T>) -> anyhow::Result<Range<T>> {
        if !self.mergeable_with(other) {
            return Err(anyhow::anyhow!("Ranges are not mergeable"));
        }
        Ok(Range::new(
            self.min_stop().min(other.min_stop()),
            self.max_stop().max(other.max_stop()),
        ))
    }

    pub fn length_inclusive(&self) -> T {
        self.max_stop() - self.min_stop() + T::one()
    }

    pub fn merged_ranges(ranges: &[Range<T>]) -> Vec<Range<T>> {
        let length = ranges.len();
        if length < 2 {
            return ranges.to_vec();
        }
        let mut nranges = vec![];
        let mut current = ranges[0];

        for next_range in ranges.iter().skip(1) {
            if let Ok(merged) = current.merge(next_range) {
                current = merged
            } else {
                nranges.push(current);
                current = *next_range;
            }
        }
        nranges.push(current);
        nranges.sort();
        nranges
    }

    pub fn compact(ranges: Vec<Range<T>>) -> Vec<Range<T>> {
        let mut ranges = ranges;
        let mut cur_length = ranges.len();
        loop {
            let new_set = Range::merged_ranges(&ranges);
            let new_len = new_set.len();
            if new_len == cur_length {
                break;
            }
            ranges = new_set;
            cur_length = new_len;
        }
        ranges
    }
}
