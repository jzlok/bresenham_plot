# bresenham_plot
Bresenham's line algorithm implementation in Rust.

This library provides a function to plot a line between two points using Bresenham's line algorithm, which is an efficient way to determine which points on a grid should be plotted to form a close approximation to a straight line between two points.

## Example
```rust
use bresenham_plot::plot_line;
plot_line(0, 0, 5, 3, |x, y| {
    println!("Plotting point: ({}, {})", x, y);
});
 ```
## Features
### `reversible`
Enables endpoint-order-stable line plotting.

When enabled, plotting a line from `A -> B` produces the same points as plotting
`B -> A`, but in reverse order.

```sh
cargo add bresenham_plot --features reversible
```

## Tests
```sh
cargo test
cargo test --features reversible
```

## Optimizations
- To reduce code bloat for limited memory environments, the algorithm works by determining the octant of the line and normalizing the coordinates to the first octant.
- inlining is hinted the compiler's discretion, allowing for better optimization based on the target architecture and usage patterns.

## no_std Support
Builds for no_std environments, making it suitable for embedded systems and other environments where the Rust standard library is not available.

## LICENSE
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.