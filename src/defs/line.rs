use super::{Point, Quadrant};

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

}
