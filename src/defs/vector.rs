use std::ops::{Add, AddAssign, Sub};

/// Describe a point in 2-space
///
/// Use two floats to denote the x and y coordinates
/// respectively in the tuple.
///
/// ```
/// use tile_net::Vector;
/// let point = Vector(0.5, 1.0);
/// assert_eq!(point.0, 0.5);
/// assert_eq!(point.1, 1.0);
/// ```
///
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector(pub f32, pub f32);

impl Vector {
	/// Create a vector from a float tuple
	pub fn from_tuple(tuple: (f32, f32)) -> Vector {
		Vector(tuple.0, tuple.1)
	}

	/// Compute the 2-norm squared, used for checking the total speed
	pub fn norm2sq(&self) -> f32 {
		self.0 * self.0 + self.1 * self.1
	}

	/// Scale both vector elements linearly
	pub fn scale(&mut self, factor: f32) {
		self.0 *= factor;
		self.1 *= factor;
	}
}

impl Add for Vector {
	type Output = Vector;

	fn add(self, other: Vector) -> Vector {
		Vector(self.0 + other.0, self.1 + other.1)
	}
}

impl AddAssign for Vector {
	fn add_assign(&mut self, rhs: Vector) {
		self.0 += rhs.0;
		self.1 += rhs.1;
	}
}

impl Sub for Vector {
	type Output = Vector;

	fn sub(self, other: Vector) -> Vector {
		Vector(self.0 - other.0, self.1 - other.1)
	}
}
