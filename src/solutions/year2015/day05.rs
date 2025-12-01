struct Coordinate(isize, isize);

impl Coordinate {
    fn from_string(s: &str) -> Self {
        let mut parts = s.split(',');
        let x = parts.next().unwrap().parse::<isize>().unwrap();
        let y = parts.next().unwrap().parse::<isize>().unwrap();
        Self(x, y)
    }
}

struct Range(Coordinate, Coordinate);

impl Range {
    fn from_string(s: &str) -> Self {
        let mut parts = s.split(" through ");
        let start = Coordinate::from_string(parts.next().unwrap());
        let end = Coordinate::from_string(parts.next().unwrap());
        Self(start, end)
    }
    fn iterate(&self) -> impl Iterator<Item = Coordinate> {
        let x_range = if self.0 .0 <= self.1 .0 {
            self.0 .0..=self.1 .0
        } else {
            self.1 .0..=self.0 .0
        };
        let y_range = if self.0 .1 <= self.1 .1 {
            self.0 .1..=self.1 .1
        } else {
            self.1 .1..=self.0 .1
        };
        x_range.flat_map(move |x| y_range.clone().map(move |y| Coordinate(x, y)))
    }
}
