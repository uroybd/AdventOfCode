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

    pub fn min_max_stop(&self) -> (T, T) {
        if self.0 <= self.1 {
            (self.0, self.1)
        } else {
            (self.1, self.0)
        }
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
        let (min, max) = self.min_max_stop();
        value >= min && value <= max
    }

    pub fn length(&self) -> T {
        let (min, max) = self.min_max_stop();
        max - min
    }

    pub fn overlaps_with(&self, other: &Range<T>) -> bool {
        let (min_self, max_self) = self.min_max_stop();
        let (min_other, max_other) = other.min_max_stop();
        (min_self >= min_other && min_self <= max_other)
            || (max_self >= min_other && max_self <= max_other)
    }

    pub fn merge(&self, other: &Range<T>) -> anyhow::Result<Range<T>> {
        let (min_self, max_self) = self.min_max_stop();
        let (min_other, max_other) = other.min_max_stop();
        if !((min_self >= min_other && min_self <= max_other)
            || (max_self >= min_other && max_self <= max_other))
        {
            return Err(anyhow::anyhow!("Ranges are not mergeable"));
        }
        Ok(Range::new(min_self.min(min_other), max_self.max(max_other)))
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
