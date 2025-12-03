#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePointError;

impl std::str::FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair: Vec<usize> = s.split(',').map(|x| x.parse::<usize>().unwrap()).collect();
        Ok(Point {
            x: pair[0],
            y: pair[1],
        })
    }
}

#[derive(Debug)]
pub struct Line {
    pub a: Point,
    pub b: Point,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseLineError;

impl std::str::FromStr for Line {
    type Err = ParseLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pair: Vec<Point> = s.split(" -> ").map(|x| x.parse().unwrap()).collect();
        Ok(Line {
            a: pair.remove(0),
            b: pair.remove(0),
        })
    }
}

impl Line {
    pub fn new(a: Point, b: Point) -> Self {
        Line { a, b }
    }

    pub fn create_line_series(&self, allow_diagonal: bool) -> Vec<Point> {
        let mut series: Vec<Point> = vec![];
        if self.a.x == self.b.x {
            let (mut start, mut end) = (self.a.y, self.b.y);
            if start > end {
                end = start;
                start = self.b.y;
            }
            for i in start..end + 1 {
                series.push(Point { x: self.a.x, y: i })
            }
        } else if self.a.y == self.b.y {
            let (mut start, mut end) = (self.a.x, self.b.x);
            if start > end {
                end = start;
                start = self.b.x;
            }
            for i in start..end + 1 {
                series.push(Point { x: i, y: self.a.y })
            }
        } else {
            if !allow_diagonal {
                return series;
            }
            let (mut reverse_x, mut reverse_y, mut start_x, mut end_x, mut start_y, mut end_y) =
                (false, false, self.a.x, self.b.x, self.a.y, self.b.y);

            if start_y > end_y {
                end_y = start_y;
                start_y = self.b.y;
                reverse_y = true;
            }
            if start_x > end_x {
                end_x = start_x;
                start_x = self.b.x;
                reverse_x = true;
            }

            let mut xs: Vec<usize> = (start_x..end_x + 1).collect();
            let mut ys: Vec<usize> = (start_y..end_y + 1).collect();

            if reverse_x {
                xs.reverse();
            }
            if reverse_y {
                ys.reverse();
            }
            for i in 0..ys.len() {
                series.push(Point { x: xs[i], y: ys[i] })
            }
        }
        series
    }
}
