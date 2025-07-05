//! # Spiral Matrix
//! 
//! ## Challenge Description
//! 
//!  Given a matrix of m * n elements (m rows, n columns), return all elements of the matrix in spiral order.

/// Returns a vector of a matrix in spiral order
/// 
/// # Examples
/// 
/// ```
/// use slothbytes::spiral_order;
/// 
/// let grid = vec![
///     vec![1, 2, 3],
///     vec![4, 5, 6],
///     vec![7, 8, 9]
/// ];
/// let expected = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
/// assert_eq!(spiral_order(grid), expected);
/// 
/// let grid = vec![
///     vec![1, 2, 3, 4],
///     vec![5, 6, 7, 8],
///     vec![9, 10, 11, 12]
/// ];
/// let expected = vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7];
/// assert_eq!(spiral_order(grid), expected);
/// 
/// ```
pub fn spiral_order(mut grid: Vec<Vec<i32>>) -> Vec<i32> {
    if grid.len() == 0 {
        return Vec::new();
    }
    if grid[0].len() == 0 {
        return Vec::new();
    }
    let mut output = Vec::new();
    while grid[0].len() > 0 {
        output.push(grid[0].clone());
        grid.remove(0);
        grid = rot_ccw(grid);
    }
    output.concat()
}

/// Rotates a grid counter clockwise
/// 
/// # Examples
/// 
/// ```
/// let grid = vec![
///     vec![1, 2, 3],
///     vec![4, 5, 6],
///     vec![7, 8, 9]
/// ];
/// let expected = vec![
///     vec![3, 6, 9],
///     vec![2, 5, 8],
///     vec![1, 4, 7]
/// ];
/// ```
fn rot_ccw(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if grid.len() == 0 {
        return vec![Vec::new()];
    }
    if grid[0].len() == 0 {
        return vec![Vec::new()];
    }
    let mut new_grid = Vec::new();
    for y in (0..grid[0].len()).rev() {
        let mut new_row = Vec::new();
        for x in 0..grid.len() {
            new_row.push(grid[x][y]);
        }
        new_grid.push(new_row.clone());
        new_row.clear();
    }
    new_grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_by_three() {
        let grid = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ];
        let expected = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
        assert_eq!(spiral_order(grid), expected);
    }

    #[test]
    fn four_by_three() {
        let grid = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12]
        ];
        let expected = vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7];
        assert_eq!(spiral_order(grid), expected);
    }

    #[test]
    fn rot_three_by_three() {
        let grid = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ];
        let expected = vec![
            vec![3, 6, 9],
            vec![2, 5, 8],
            vec![1, 4, 7]
        ];
        assert_eq!(rot_ccw(grid), expected);
    }

    #[test]
    fn rot_three_by_one() {
        let grid = vec![
            vec![1, 2, 3]
        ];
        let expected = vec![
            vec![3],
            vec![2],
            vec![1]
        ];
        assert_eq!(rot_ccw(grid), expected);
    }

    #[test]
    fn rot_one_by_one() {
        let grid = vec![
            vec![1]
        ];
        let expected = vec![
            vec![1]
        ];
        assert_eq!(rot_ccw(grid), expected);
    }

    #[test]
    fn rot_empty() {
        let grid: Vec<Vec<i32>> = vec![
            Vec::new()
        ];
        let expected: Vec<Vec<i32>> = vec![
            Vec::new()
        ];
        assert_eq!(rot_ccw(grid), expected);
    }

}