use std::ops::{Add, Sub};

/// Describe a point in 2-space
///
/// Use two floats to denote the x and y coordinates
/// respectively in the tuple.
///
/// ```
/// use tile_net::Point;
/// let point = Point(0.5, 1.0);
/// assert_eq!(point.0, 0.5);
/// assert_eq!(point.1, 1.0);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point(pub f32, pub f32);

impl Add for Point {
	type Output = Point;

	fn add(self, other: Point) -> Point {
		Point(self.0 + other.0, self.1 + other.1)
	}
}

impl Sub for Point {
	type Output = Point;

	fn sub(self, other: Point) -> Point {
		Point(self.0 - other.0, self.1 - other.1)
	}
}
