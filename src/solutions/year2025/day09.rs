// Advent of Code 2025 - Day 09

#![allow(clippy::similar_names)]
#![allow(clippy::cast_possible_wrap)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate(usize, usize);

impl Coordinate {
    fn from_str(s: &str) -> Self {
        let parts: Vec<&str> = s.split(',').collect();
        Self(parts[0].parse().unwrap(), parts[1].parse().unwrap())
    }

    const fn area(&self, other: &Self) -> usize {
        let width = self.0.abs_diff(other.0) + 1;
        let height = self.1.abs_diff(other.1) + 1;
        width * height
    }

    fn get_area_and_corners(&self, other: &Self) -> (usize, [Coordinate; 4]) {
        let area = self.area(other);
        let x1 = self.0.min(other.0);
        let x2 = self.0.max(other.0);
        let y1 = self.1.min(other.1);
        let y2 = self.1.max(other.1);
        let corners = [
            Coordinate(x1, y1),
            Coordinate(x1, y2),
            Coordinate(x2, y1),
            Coordinate(x2, y2),
        ];
        (area, corners)
    }

    const fn as_isize(self) -> (isize, isize) {
        (self.0 as isize, self.1 as isize)
    }
}

#[inline]
const fn cross_product(origin: Coordinate, p1: Coordinate, p2: Coordinate) -> isize {
    // Cross product: (p1 - origin) × (p2 - origin)
    // = (p1.x - origin.x)(p2.y - origin.y) - (p1.y - origin.y)(p2.x - origin.x)
    let (ox, oy) = origin.as_isize();
    let (x1, y1) = p1.as_isize();
    let (x2, y2) = p2.as_isize();
    
    (x1 - ox) * (y2 - oy) - (y1 - oy) * (x2 - ox)
}

#[inline]
fn is_point_on_line(p: Coordinate, p1: Coordinate, p2: Coordinate) -> bool {
    // Point is on line if cross product is zero
    let cross = cross_product(p1, p2, p);
    if cross != 0 {
        return false;
    }
    // Also check if point is within bounding box of line segment
    (p.0 >= p1.0.min(p2.0) && p.0 <= p1.0.max(p2.0))
        && (p.1 >= p1.1.min(p2.1) && p.1 <= p1.1.max(p2.1))
}

#[inline]
fn ray_crosses_edge(p: Coordinate, p1: Coordinate, p2: Coordinate) -> bool {
    // Check if a horizontal ray from p to the right crosses the edge p1-p2
    // Pre-condition: p is between p1.y and p2.y (not equal)
    let (px, py) = p.as_isize();
    let (x1, y1) = p1.as_isize();
    let (x2, y2) = p2.as_isize();

    px < (x2 - x1) * (py - y1) / (y2 - y1) + x1
}

#[inline]
#[allow(clippy::cast_possible_truncation)]
const fn get_orient(a: Coordinate, b: Coordinate, c: Coordinate) -> i8 {
    // Orientation is the sign of the cross product
    cross_product(a, b, c).signum() as i8
}

#[inline]
const fn do_lines_intersect(
    a1: Coordinate,
    a2: Coordinate,
    b1: Coordinate,
    b2: Coordinate,
) -> bool {
    let o1 = get_orient(a1, a2, b1);
    let o2 = get_orient(a1, a2, b2);

    // Early return: if o1 and o2 have the same sign, lines can't intersect
    if o1 * o2 >= 0 {
        return false;
    }

    let o3 = get_orient(b1, b2, a1);
    let o4 = get_orient(b1, b2, a2);

    o3 * o4 < 0
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Floor {
    grid: Vec<Coordinate>,
}

impl Floor {
    fn from_str(s: &str) -> Self {
        let grid = s.lines().map(Coordinate::from_str).collect();
        Self { grid }
    }

    fn get_largest_area(&self) -> usize {
        let mut max_area = 0;
        let length = self.grid.len();
        for i in 0..length {
            for j in i + 1..length {
                let area = self.grid[i].area(&self.grid[j]);
                if area > max_area {
                    max_area = area;
                }
            }
        }
        max_area
    }

    fn is_point_inside(&self, p: Coordinate) -> bool {
        let mut inside = false;
        let n = self.grid.len();
        for i in 0..n {
            let p1 = self.grid[i];
            let p2 = self.grid[(i + 1) % n];

            if is_point_on_line(p, p1, p2) {
                return true;
            }
            if (p1.1 > p.1) == (p2.1 > p.1) || p1.1 == p2.1 {
                continue;
            }

            if ray_crosses_edge(p, p1, p2) {
                inside = !inside;
            }
        }
        inside
    }

    fn is_rect_inside(&self, corners: &[Coordinate; 4]) -> bool {
        // Check all 4 corners are inside the polygon (early exit on failure)
        for &corner in corners {
            if !self.is_point_inside(corner) {
                return false;
            }
        }

        // Check that no rectangle edge intersects with polygon edges
        let rect_edges = [
            (corners[0], corners[2]),
            (corners[2], corners[3]),
            (corners[3], corners[1]),
            (corners[1], corners[0]),
        ];

        let n = self.grid.len();
        for i in 0..n {
            let p1 = self.grid[i];
            let p2 = self.grid[(i + 1) % n];
            for (start, end) in rect_edges {
                if do_lines_intersect(start, end, p1, p2) {
                    return false;
                }
            }
        }
        true
    }

    fn get_largest_area_in_hull(&self) -> usize {
        let length = self.grid.len();

        // Build list of all possible rectangles with their areas
        let mut rectangles: Vec<(usize, [Coordinate; 4])> = (0..length - 1)
            .flat_map(|i| {
                (i + 1..length).map(move |j| {
                    let a = self.grid[i];
                    let b = self.grid[j];
                    a.get_area_and_corners(&b)
                })
            })
            .collect();

        // Sort by area descending - check largest first for early exit
        rectangles.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        // Find the first (largest) valid rectangle
        for (area, corners) in rectangles {
            if self.is_rect_inside(&corners) {
                return area;
            }
        }
        0
    }
}

/// # Errors
/// Returns an error if the file cannot be read or parsed.
pub fn solution_2025_09_01(file_path: String) -> anyhow::Result<usize> {
    Ok(Floor::from_str(&std::fs::read_to_string(file_path)?).get_largest_area())
}

/// # Errors
/// Returns an error if the file cannot be read or parsed.
pub fn solution_2025_09_02(file_path: String) -> anyhow::Result<usize> {
    Ok(Floor::from_str(&std::fs::read_to_string(file_path)?).get_largest_area_in_hull())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2025_09_01() {
        let file_path: String = String::from("inputs/2025/day09e.txt");
        let result = solution_2025_09_01(file_path).unwrap();
        assert_eq!(result, 50);
    }

    #[test]
    fn test_2025_09_02() {
        let file_path: String = String::from("inputs/2025/day09e.txt");
        let result = solution_2025_09_02(file_path).unwrap();
        assert_eq!(result, 24);
    }

    #[test]
    #[ignore]
    fn output_2025_09_01() {
        let file_path: String = String::from("inputs/2025/day09.txt");
        let result = solution_2025_09_01(file_path).unwrap();
        assert_eq!(result, 4763932976);
    }

    #[test]
    #[ignore]
    fn output_2025_09_02() {
        let file_path: String = String::from("inputs/2025/day09.txt");
        let result = solution_2025_09_02(file_path).unwrap();
        assert_eq!(result, 1501292304);
    }
}
