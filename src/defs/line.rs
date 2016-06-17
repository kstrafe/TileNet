pub use super::{Index, Point, Quadrant};

/// Describe a line by its start and end `Point` respectively
///
/// A line can be constructed and used with tuples
///
/// ```
/// use tile_net::{Line, Point};
/// let start = Point(0.5, 1.0);
/// let finish = Point(1.2, -1.0);
/// let line = Line(start, finish);
/// assert_eq!(line.0, start);
/// assert_eq!(line.1, finish);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line(pub Point, pub Point);

impl Line {
	pub fn from_origo(p: Point) -> Line {
		Line(Point(0.0, 0.0), p)
	}

	pub fn quadrant(&self) -> Quadrant {
		if (self.1).0 <= (self.0).0 &&
		(self.1).1 > (self.0).1 {
			Quadrant::Two
		} else if (self.1).0 < (self.0).0 &&
		(self.1).1 <= (self.0).1 {
			Quadrant::Three
		} else if (self.1).0 >= (self.0).0 &&
		(self.1).1 < (self.0).1 {
			Quadrant::Four
		} else {
			Quadrant::One
		}
	}

	pub fn mirror_x(&mut self) {
		let distance = (self.0).1 - (self.1).1;
		(self.0).1 = (self.0).1.floor() + 1.0 - (self.0).1.fract();
		(self.1).1 = (self.0).1 + distance;
	}

	pub fn mirror_y(&mut self) {
		let distance = (self.0).0 - (self.1).0;
		(self.0).0 = (self.0).0.floor() + 1.0 - (self.0).0.fract();
		(self.1).0 = (self.0).0 + distance;
	}

	/// Create a supercover line
	///
	/// The supercover line covers all discrete blocks.
	/// It's similar to Bresenham's algorithm, but it includes the
	/// blocks that have been overlapped by a small portion of the line.
	/// The blocks are given by an integer boundary.
	pub fn supercover(&self) -> () {
		// First octant
		let (mut start, stop) = (self.0, self.1);
		let new = self.1 - self.0;
		let (dx, dy) = (new.0, new.1);
		let (step_x, step_y);
		// First octant
		if dx > 0.0 && dy >= 0.0 && dx > dy {
			step_x = 1.0;
			step_y = dy/dx;
		// Second octant
		} else if dx > 0.0 && dy > 0.0 && dy >= dx {
			step_x = dx/dy;
			step_y = 1.0;
		// Third octant
		} else if dx <= 0.0 && dy > 0.0 && dy > -dx {
			step_x = dx/dy;
			step_y = 1.0;
		// Fourth octant
		} else if dx < 0.0 && dy > 0.0 && dx >= -dy {
			step_x = -1.0;
			step_y = -dy/dx;
		// Fifth octant
		} else if dx < 0.0 && dy <= 0.0 && dx < dy {
			step_x = -1.0;
			step_y = -dy/dx;
		// Sixth octant
		} else if dx < 0.0 && dy <= 0.0 && dy > -dx {
			step_x = -dx/dy;
			step_y = -1.0;
		// Seventh octant
		} else if dx <= 0.0 && dy < 0.0 && dx > -dy {
			step_x = dx/dy;
			step_y = -1.0;
		// Eight octant
		} else /*if dx < 0.0 && dy <= 0.0 && dx > -dy*/ {
			step_x = 1.0;
			step_y = -dy/dx;
		}
		while start.to_index() != stop.to_index() {
			start.0 += step_x;
			start.1 += step_y;
			println!("NewPos: {:?}", start);
		}
	}

}

#[cfg(test)]
mod tests {
	use super::{Line, Point};

	#[test]
	fn supercover() {
		let line = Line(Point(0.0, 0.0), Point(-100.0, 0.0));
		line.supercover();
	}
}
