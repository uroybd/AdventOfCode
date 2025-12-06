// Advent of Code 2023 - Day 24: Never Tell Me The Odds
//
// Part 1: Find 2D path intersections of particles within a test area
// Part 2: Find a rock trajectory that collides with all particles
//
// Key Concepts:
// 1. Parametric Line Equations: position(t) = p₀ + v·t
// 2. Linear Algebra: Cramer's rule for 2×2 systems, Gaussian elimination for larger systems
// 3. Path vs Collision: Paths can intersect even if particles arrive at different times

#[derive(Debug, Clone, Copy)]
struct Particle {
    position: (f64, f64, f64),
    velocity: (f64, f64, f64),
}

impl Particle {
    fn from_str(s: &str) -> Self {
        let mut parts = s.splitn(2, " @ ");
        let positions: Vec<f64> = parts
            .next()
            .unwrap()
            .split(", ")
            .map(|x| x.trim().parse().unwrap())
            .collect();
        let velocities: Vec<f64> = parts
            .next()
            .unwrap()
            .split(", ")
            .map(|x| x.trim().parse().unwrap())
            .collect();
        Self {
            position: (positions[0], positions[1], positions[2]),
            velocity: (velocities[0], velocities[1], velocities[2]),
        }
    }

    /// Check if this particle's 2D path intersects with another's within the test area.
    ///
    /// Uses parametric equations to find where paths cross:
    ///   position₁ + velocity₁·t₁ = position₂ + velocity₂·t₂
    ///
    /// Solves the linear system using Cramer's rule:
    ///   [v₁ₓ, -v₂ₓ] [t₁]   [x₂ - x₁]
    ///   [v₁ᵧ, -v₂ᵧ] [t₂] = [y₂ - y₁]
    fn intersects_in_test_area(&self, other: &Particle, test_area: (f64, f64)) -> bool {
        let (v1x, v1y) = (self.velocity.0, self.velocity.1);
        let (v2x, v2y) = (other.velocity.0, other.velocity.1);

        let det = v1x * (-v2y) - v1y * (-v2x);

        if det.abs() < 1e-10 {
            return false; // Parallel paths
        }

        let dx = other.position.0 - self.position.0;
        let dy = other.position.1 - self.position.1;

        // Cramer's rule: t = numerator / denominator
        // For future intersection: numerator and denominator must have same sign
        let t1_numerator = dx * (-v2y) - dy * (-v2x);
        let t2_numerator = v1x * dy - v1y * dx;

        if t1_numerator * det < 0.0 || t2_numerator * det < 0.0 {
            return false; // Intersection in the past
        }

        let t1 = t1_numerator / det;
        let intersection_x = self.position.0 + self.velocity.0 * t1;
        let intersection_y = self.position.1 + self.velocity.1 * t1;

        intersection_x >= test_area.0
            && intersection_x <= test_area.1
            && intersection_y >= test_area.0
            && intersection_y <= test_area.1
    }

    /// Create a row for the linear system to find rock trajectory.
    ///
    /// Builds coefficients for the linearized collision equation:
    ///   xₛ(vᵧ₁-vᵧ₀) + yₛ(vₓ₀-vₓ₁) + vₓₛ(y₀-y₁) + vᵧₛ(x₁-x₀)
    fn make_collision_row(&self, other: &Particle) -> [f64; 4] {
        [
            other.velocity.1 - self.velocity.1, // coefficient for xₛ
            self.velocity.0 - other.velocity.0, // coefficient for yₛ
            self.position.1 - other.position.1, // coefficient for vₓₛ
            other.position.0 - self.position.0, // coefficient for vᵧₛ
        ]
    }

    /// Create the right-hand side for the linear system.
    ///
    /// Computes: x₁vᵧ₁ - x₀vᵧ₀ + y₀vₓ₀ - y₁vₓ₁
    fn make_collision_rhs(&self, other: &Particle) -> f64 {
        other.position.0 * other.velocity.1 - self.position.0 * self.velocity.1
            + self.position.1 * self.velocity.0
            - other.position.1 * other.velocity.0
    }

    /// Calculate collision time with a rock traveling at given position/velocity
    fn collision_time(&self, rock_x: f64, rock_vx: f64) -> f64 {
        (rock_x - self.position.0) / (self.velocity.0 - rock_vx)
    }

    /// Get Z position at a given time
    fn z_at_time(&self, t: f64) -> f64 {
        self.position.2 + self.velocity.2 * t
    }
}

/// Solve a 2×2 linear system using Cramer's rule
fn solve_2x2(matrix: [[f64; 2]; 2], rhs: [f64; 2]) -> Option<[f64; 2]> {
    let det = matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0];

    if det.abs() < 1e-10 {
        return None;
    }

    let x = (rhs[0] * matrix[1][1] - rhs[1] * matrix[0][1]) / det;
    let y = (matrix[0][0] * rhs[1] - matrix[1][0] * rhs[0]) / det;

    Some([x, y])
}

/// Solve a 4×4 linear system using Gaussian elimination with partial pivoting
fn solve_4x4(mut matrix: [[f64; 4]; 4], mut rhs: [f64; 4]) -> Option<[f64; 4]> {
    let n = 4;

    // Forward elimination with partial pivoting
    for i in 0..n {
        // Find pivot (largest absolute value in column)
        let mut max_row = i;
        for j in i + 1..n {
            if matrix[j][i].abs() > matrix[max_row][i].abs() {
                max_row = j;
            }
        }

        // Swap rows if needed
        if max_row != i {
            matrix.swap(i, max_row);
            rhs.swap(i, max_row);
        }

        let pivot = matrix[i][i];
        if pivot.abs() < 1e-10 {
            return None; // Singular matrix
        }

        // Eliminate column below pivot
        for j in i + 1..n {
            let factor = matrix[j][i] / pivot;
            for k in i..n {
                matrix[j][k] -= factor * matrix[i][k];
            }
            rhs[j] -= factor * rhs[i];
        }
    }

    // Back substitution
    let mut solution = [0.0; 4];
    for i in (0..n).rev() {
        let mut sum = rhs[i];
        for j in i + 1..n {
            sum -= matrix[i][j] * solution[j];
        }
        solution[i] = sum / matrix[i][i];
    }

    Some(solution)
}

/// Part 1: Count path intersections in 2D within test area
pub fn solution_2023_24_01(file_path: String, range: (f64, f64)) -> anyhow::Result<usize> {
    let particles: Vec<Particle> = std::fs::read_to_string(file_path)?
        .lines()
        .map(Particle::from_str)
        .collect();

    let mut intersection_count = 0;
    for i in 0..particles.len() {
        for j in (i + 1)..particles.len() {
            if particles[i].intersects_in_test_area(&particles[j], range) {
                intersection_count += 1;
            }
        }
    }

    Ok(intersection_count)
}

/// Part 2: Find rock position that collides with all particles
///
/// Mathematical Approach:
/// For a rock at position (xₛ, yₛ, zₛ) with velocity (vₓₛ, vᵧₛ, vᵤₛ),
/// it must collide with each particle i at some time tᵢ:
///   rock_pos + rock_vel·tᵢ = particle_i_pos + particle_i_vel·tᵢ
///
/// This is non-linear, but we can linearize by eliminating time:
/// For two particles, derive:
///   xₛ(vᵧ₁-vᵧ₀) + yₛ(vₓ₀-vₓ₁) + vₓₛ(y₀-y₁) + vᵧₛ(x₁-x₀) = x₁vᵧ₁ - x₀vᵧ₀ + y₀vₓ₀ - y₁vₓ₁
///
/// This creates 4 linear equations for (xₛ, yₛ, vₓₛ, vᵧₛ) using 5 particles.
/// Then solve for z separately using the computed times.
pub fn solution_2023_24_02(file_path: String) -> anyhow::Result<usize> {
    let input = std::fs::read_to_string(file_path)?;
    let particles: Vec<Particle> = input.lines().map(Particle::from_str).collect();

    if particles.len() < 5 {
        anyhow::bail!("Need at least 5 particles");
    }

    // Use particles 10-14 for better numerical stability if available, otherwise use 0-4
    let start_idx = if particles.len() >= 15 { 10 } else { 0 };
    let p = [
        particles[start_idx],
        particles[start_idx + 1],
        particles[start_idx + 2],
        particles[start_idx + 3],
        particles[start_idx + 4],
    ];

    // Build coefficient matrix for X-Y system using particle methods
    let matrix = [
        p[0].make_collision_row(&p[1]),
        p[1].make_collision_row(&p[2]),
        p[2].make_collision_row(&p[3]),
        p[3].make_collision_row(&p[4]),
    ];

    let rhs = [
        p[0].make_collision_rhs(&p[1]),
        p[1].make_collision_rhs(&p[2]),
        p[2].make_collision_rhs(&p[3]),
        p[3].make_collision_rhs(&p[4]),
    ];

    let xy_solution =
        solve_4x4(matrix, rhs).ok_or_else(|| anyhow::anyhow!("Failed to solve XY system"))?;

    let (xs, ys, vxs, _) = (
        xy_solution[0],
        xy_solution[1],
        xy_solution[2],
        xy_solution[3],
    );

    // Compute collision times with first two particles
    let t0 = p[0].collision_time(xs, vxs);
    let t1 = p[1].collision_time(xs, vxs);

    // Solve for Z using these times
    let z_matrix = [[1.0, t0], [1.0, t1]];
    let z_rhs = [p[0].z_at_time(t0), p[1].z_at_time(t1)];

    let z_solution =
        solve_2x2(z_matrix, z_rhs).ok_or_else(|| anyhow::anyhow!("Failed to solve Z system"))?;

    let zs = z_solution[0];

    // Round to get exact integer answer
    let answer = (xs + ys + zs).round() as usize;
    Ok(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2023_24_01() {
        let file_path: String = String::from("inputs/2023/day24e.txt");
        let result = solution_2023_24_01(file_path, (7.0, 27.0)).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2023_24_02() {
        let file_path: String = String::from("inputs/2023/day24e.txt");
        let result = solution_2023_24_02(file_path).unwrap();
        assert_eq!(result, 47);
    }

    #[test]
    #[ignore]
    fn output_2023_24_01() {
        let file_path: String = String::from("inputs/2023/day24.txt");
        let result =
            solution_2023_24_01(file_path, (200000000000000.0, 400000000000000.0)).unwrap();
        assert_eq!(result, 15558);
    }

    #[test]
    #[ignore]
    fn output_2023_24_02() {
        let file_path: String = String::from("inputs/2023/day24.txt");
        let result = solution_2023_24_02(file_path).unwrap();
        assert_eq!(result, 765636044333842);
    }
}
