#![cfg_attr(not(test), no_std)]

//! Bresenham's line algorithm implementation in Rust.
//!
//! This crate provides a function to plot a line between two points using
//! Bresenham's line algorithm, an efficient integer algorithm for rasterizing
//! lines. The implementation normalizes coordinates into the first octant,
//! runs the algorithm, then denormalizes the results back to the original
//! octant so it works for lines in any direction.
//!
//! Example
//! ```rust
//! use bresenham_plot::plot_line;
//!
//! plot_line(0, 0, 5, 3, |x, y| {
//!     println!("Plotting point: ({}, {})", x, y);
//! });
//! ```

// added due to 6502 mos-llvm target
use core::unreachable;
use core::ops::FnMut;

/// Plots a reversible line from (x0, y0) to (x1, y1) using Bresenham's line algorithm.
/// This library provides a function to plot a line between two points using Bresenham's line algorithm, which is an efficient way to determine which points on a grid should be plotted to form a close approximation to a straight line between two points.
/// The algorithm works by determining the octant of the line and normalizing the coordinates to the first octant, allowing for a consistent way to calculate the points on the line regardless of its direction.
/// inlined by hint to reduce function call overhead, as this function is likely to be called frequently in performance-critical code, such as in graphics rendering loops.
/// # Arguments
/// * `x0` - The x-coordinate of the starting point of the line.
/// * `y0` - The y-coordinate of the starting point of the line.
/// * `x1` - The x-coordinate of the ending point of the line.
/// * `y1` - The y-coordinate of the ending point of the line.
/// * `plot` - A function that takes the x and y coordinates of a point and
/// plots it. This function will be called for each point on the line.
/// # Example
/// ```rust
/// use bresenham_plot::plot_line;
/// plot_line(0, 0, 5, 3, |x, y| {
///   println!("Plotting point: ({}, {})", x, y);
/// });
/// ```
#[cfg(feature = "reversible")]
#[inline]
pub fn plot_line<F>(x0: isize, y0: isize, x1: isize, y1: isize, mut plot: F)
where
    F: FnMut(isize, isize),
{
    let octant = get_octant(x0, y0, x1, y1);

    // normalize into the first octant
    let (mut x, mut y) = normalize(octant, x0, y0);
    let (x_end, y_end) = normalize(octant, x1, y1);

    // delta values of normalized coordinates
    let dx = x_end - x;
    let dy2 = (y_end - y) * 2;

    let dx2 = 2 * dx;
    let mut error = dy2 - dx;

    loop {
        // plot the point in the denormalized octant
        let (plot_x, plot_y) = denormalize(octant, x, y);
        plot(plot_x, plot_y);

        if x == x_end {
            break;
        }

        if error >= 0 {
            y += 1;
            error -= dx2;
        }

        error += dy2;
        x += 1;
    }
}

/// Plots a non-reversible line from (x0, y0) to (x1, y1) using Bresenham's line algorithm.
/// The `plot` function is called for each point on the line, with the coordinates of the point as arguments.
/// The coordinates are normalized to the first octant for the algorithm, and then denormalized back to the original octant before plotting.
/// This allows the algorithm to work correctly for lines in any direction.
/// inlined by hint to reduce function call overhead, as this function is likely to be called frequently in performance-critical code, such as in graphics rendering loops.
/// # Arguments
/// * `x0` - The x-coordinate of the starting point of the line.
/// * `y0` - The y-coordinate of the starting point of the line.
/// * `x1` - The x-coordinate of the ending point of the line.
/// * `y1` - The y-coordinate of the ending point of the line.
/// * `plot` - A function that takes the x and y coordinates of a point and
/// plots it. This function will be called for each point on the line.
/// # Example
/// ```rust
/// use bresenham_plot::plot_line;
/// plot_line(0, 0, 5, 3, |x, y| {
///   println!("Plotting point: ({}, {})", x, y);
/// });
/// ```
#[cfg(not(feature = "reversible"))]
#[inline]
pub fn plot_line<F>(x0: isize, y0: isize, x1: isize, y1: isize, mut plot: F)
where
    F: FnMut(isize, isize),
{
    let octant = get_octant(x0, y0, x1, y1);

    // normalize into the first octant
    let (mut x, mut y) = normalize(octant, x0, y0);
    let (x_end, y_end) = normalize(octant, x1, y1);

    // delta values of normalized coordinates
    let dx = x_end - x;
    let dy = y_end - y;

    let mut error = dy - dx;

    loop {
        // plot the point in the denormalized octant
        let (plot_x, plot_y) = denormalize(octant, x, y);
        plot(plot_x, plot_y);

        if x == x_end {
            break;
        }

        if error >= 0 {
            y += 1;
            error -= dx;
        }

        error += dy;
        x += 1;
    }
}

// Normalizes the coordinates to the first octant based on the octant number.
// This function takes the octant number and the original coordinates (x, y) and returns the normalized coordinates corresponding to the first octant.
// The transformation is based on the octant number, which indicates how the original coordinates should be transformed to fit into the first octant (where x and y are both positive and x is greater than y).
// The octant number determines how the coordinates are transformed, such as swapping x and y, negating x or y, or both, to ensure that the line can be processed using Bresenham's line algorithm in the first octant.
// # Arguments
// * `octant` - The octant number (0 to 7) that indicates the original octant of the line.
// * `x` - The x-coordinate in the original octant.
// * `y` - The y-coordinate in the original octant.
// # Returns
// A tuple containing the normalized coordinates (x, y) corresponding to the first octant.
#[inline]
fn normalize(octant: u8, x: isize, y: isize) -> (isize, isize) {
    match octant {
        0 => (x, y),
        1 => (y, x),
        2 => (y, -x),
        3 => (-x, y),
        4 => (-x, -y),
        5 => (-y, -x),
        6 => (-y, x),
        7 => (x, -y),
        _ => unreachable!(),
    }
}

// Denormalizes the coordinates from the first octant back to the original octant based on the octant number.
// This function takes the octant number and the normalized coordinates (x, y) and returns
// the denormalized coordinates corresponding to the original octant. The transformation is based on the same logic used in the normalization process, but in reverse.
// The octant number determines how the coordinates were transformed during normalization, and this function applies the inverse transformation to return the coordinates to their original orientation.
// # Arguments
// * `octant` - The octant number (0 to 7) that indicates the original octant of the line.
// * `x` - The x-coordinate in the normalized first octant.
// * `y` - The y-coordinate in the normalized first octant.
// # Returns
// A tuple containing the denormalized coordinates (x, y) corresponding to the original oct
// ant.
#[inline]
fn denormalize(octant: u8, x: isize, y: isize) -> (isize, isize) {
    match octant {
        0 => (x, y),
        1 => (y, x),
        2 => (-y, x),
        3 => (-x, y),
        4 => (-x, -y),
        5 => (-y, -x),
        6 => (y, -x),
        7 => (x, -y),
        _ => unreachable!(),
    }
}

// Determines the octant of the line from (x0, y0) to (x1, y1).
// The octant is determined based on the signs and magnitudes of the differences in x and y coordinates.
// The octants are numbered from 0 to 7, starting from the positive x axis and moving counterclockwise.
// The octant is used to normalize the coordinates for the Bresenham's line algorithm, which is designed to work in the first octant (where x and y are both positive and x is greater than y). By determining the octant, we can transform the coordinates to fit this requirement, and then transform them back after plotting the points.
// # Arguments
// * `x0` - The x-coordinate of the starting point of the line.
// * `y0` - The y-coordinate of the starting point of the line.
// * `x1` - The x-coordinate of the ending point of the line.
// * `y1` - The y-coordinate of the ending point of the line.
// # Returns
// The octant of the line, represented as a number from 0 to 7.
#[inline]
fn get_octant(x0: isize, y0: isize, x1: isize, y1: isize) -> u8 {
    let mut dy = y1 - y0;

    let (mut dx, mut octant) = if dy < 0 {
        dy = -dy;
        (x0 - x1, 4u8)
    }
    else {
        (x1 - x0, 0u8)
    };

    if dx < 0 {
        let tmp = dx;
        dx = dy;
        dy = -tmp;
        octant += 2;
    }

    if dx < dy {
        octant + 1
    }
    else {
        octant
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec::Vec;

    #[test]
    fn test_get_octant() {
        assert_eq!(get_octant(0, 0, 5, 3), 0);
        assert_eq!(get_octant(0, 0, 3, 5), 1);
        assert_eq!(get_octant(0, 0, -3, 5), 2);
        assert_eq!(get_octant(0, 0, -5, 3), 3);
        assert_eq!(get_octant(0, 0, -5, -3), 4);
        assert_eq!(get_octant(0, 0, -3, -5), 5);
        assert_eq!(get_octant(0, 0, 3, -5), 6);
        assert_eq!(get_octant(0, 0, 5, -3), 7);
    }

    #[cfg(feature = "reversible")]
    #[test]
    fn test_plot_line_reversible() {
        let expected = vec![(0, 0), (1, 1), (2, 1), (3, 2), (4, 2), (5, 3)];

        let mut points = Vec::new();
        plot_line(0, 0, 5, 3, |x, y| {
            points.push((x, y));
        });
        assert_eq!(points, expected);

        let mut backward_points = Vec::new();
        plot_line(5, 3, 0, 0, |x, y| {
            backward_points.push((x, y));
        });

        let mut reversed = expected;
        reversed.reverse();

        assert_eq!(backward_points, reversed);
    }

    #[cfg(not(feature = "reversible"))]
    #[test]
    fn test_plot_line() {
        let expected = vec![(0, 0), (1, 0), (2, 1), (3, 1), (4, 2), (5, 3)];

        let mut points = Vec::new();
        plot_line(0, 0, 5, 3, |x, y| {
            points.push((x, y));
        });
        assert_eq!(points, expected);
    }
}
