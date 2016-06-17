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
		let (mut start, stop) = (self.0, self.1);
		let new = stop - start;
		let (dx, dy) = (new.0, new.1);
		let (step_x, step_y);
		let margin = 1.001;
		// Second octant
		if dx > 0.0 && dy > 0.0 && dy >= dx {
			step_x = dx/dy/margin;
			step_y = 1.0/margin;
		// Third octant
		} else if dx <= 0.0 && dy > 0.0 && dy > -dx {
			step_x = dx/dy/margin;
			step_y = 1.0/margin;
			println!("Detected third octant {},{}", step_x, step_y);
		// Fourth octant
		} else if dx < 0.0 && dy > 0.0 && -dx >= dy {
			step_x = -1.0/margin;
			step_y = -dy/dx/margin;
			println!("Detected fourth octant {},{}", step_x, step_y);
		// Fifth octant
		} else if dx < 0.0 && dy <= 0.0 && dx < dy {
			step_x = -1.0/margin;
			step_y = -dy/dx/margin;
		// Sixth octant
		} else if dx < 0.0 && dy < 0.0 && dy <= dx {
			step_x = -dx/dy/margin;
			step_y = -1.0/margin;
		// Seventh octant
		} else if dx >= 0.0 && dy < 0.0 && -dy > dx {
			step_x = -dx/dy/margin;
			step_y = -1.0/margin;
			println!("Detected seventh octant {},{}", step_x, step_y);
		// Eight octant
		} else if dx > 0.0 && dy < 0.0 && dx > -dy {
			step_x = 1.0/margin;
			step_y = dy/dx/margin;
		} // First octant
		else /*dx > 0.0 && dy >= 0.0 && dx > dy*/ {
			step_x = 1.0/margin;
			step_y = dy/dx/margin;
		}
		while start.to_index() != stop.to_index() {
			println!("current: {:?}, end: {:?}", start, stop);
			start.0 += step_x;
			start.1 += step_y;
			abort_on_high_locations(start.0, start.1);
		}
	}

}

#[cfg(test)]
fn abort_on_high_locations(x: f32, y: f32) {
	if x.abs() > 4000.0 || y.abs() > 4000.0 {
		panic!("Loop reaches point ({}, {}), I don't think this is correct. Aborting test.", x, y);
	}
}

#[cfg(not(test))]
fn abort_on_high_locations(_: f32, _: f32) { }

#[cfg(test)]
mod tests {
	use super::{Line, Point};

	#[test]
	fn supercover() {
		(0i32..360)
			.map(|x| x as f32)
			.map(|x| x*::std::f32::consts::PI/180.0)
			.map(|x| Point(x.cos(), x.sin()))
			.map(|x| Line::from_origo(x))
			.inspect(|x| { println!("Testing: {:?}", x); x.supercover() })
			.count();

		(0i32..360)
			.map(|x| x as f32)
			.map(|x| x*::std::f32::consts::PI/180.0)
			.map(|x| Point(2000.0*x.cos(), 3000.0*x.sin()))
			.map(|x| Line::from_origo(x))
			.inspect(|x| { println!("Testing: {:?}", x); x.supercover() })
			.count();
	}
}
