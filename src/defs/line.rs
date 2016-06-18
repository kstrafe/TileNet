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

	/// Create a line using its end-point, starting in (0, 0)
	pub fn from_origo(p: Point) -> Line {
		Line(Point(0.0, 0.0), p)
	}

	/// Show us which quadrant this vector points to
	///
	/// The quadrant is the second (end) point relative
	/// to the beginning point. See more in-depth rules
	/// about edge cacses at `Quadrant`.
	///
	/// ```
	/// use tile_net::{Line, Point, Quadrant};
	/// assert_eq!(Line::from_origo(Point(-3.0, 0.0)).quadrant(), Quadrant::Three);
	/// ```
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

	/// Create a supercover line
	///
	/// The supercover line covers all discrete blocks.
	/// It's similar to Bresenham's algorithm, but it includes the
	/// blocks that have been overlapped by a small portion of the line.
	/// The blocks are given by an integer boundary.
	///
	/// This particular algorithm is based off
	/// http://lodev.org/cgtutor/raycasting.html
	/// It is a ray tracer.
	///
	/// The created iterator leaps from the start to the end node.
	/// The intended use for this iterator is in finding a collision
	/// between a tile and a moving object.
	///
	pub fn supercover(&self) -> LineTiles {
		let (start, stop) = (self.0, self.1);
		let new = stop - start;
		let (vx, vy) = (new.0, new.1);
		let slope_x = 1.0 + vy*vy/vx/vx;
		let slope_y = 1.0 + vy*vy/vx/vx;
		let (dx, dy) = (slope_x.sqrt(), slope_y.sqrt());

		let (ix, iy) = (start.0.floor(), start.1.floor());

		let (sx, sy);
		let (ex, ey);

		if vx < 0.0 {
			sx = -1.0; ex = (start.0 - ix)*dx;
		} else {
			sx = 1.0; ex = (ix + 1.0 - start.0)*dx;
		}

		if vy < 0.0 {
			sy = -1.0; ey = (start.1 - iy)*dy;
		} else {
			sy = 1.0; ey = (iy + 1.0 - start.1)*dy;
		}

		let len = (vx*vx + vy*vy).sqrt();

		LineTiles {
			len: len, dx: dx, dy: dy, sx: sx, sy: sy,
			ex: ex, ey: ey, ix: ix, iy: iy
		}
	}

}

/// Iterator for traversing from one point on the line
/// to the end point
///
/// This iterator traverses according to the supercover
/// ray-tracing algorithm. It can be obtained via the
/// following example.
///
/// ```
/// use tile_net::{Line, Point};
/// let line = Line(Point(0.8, 10.3), Point(-30.0, 5.9));
/// let tiles = line.supercover();
/// for tile in tiles {
///		println!("{:?}", tile);
/// }
pub struct LineTiles {
	len: f32, dx: f32, dy: f32, sx: f32, sy: f32,
	ex: f32, ey: f32, ix: f32, iy: f32,
}

impl Iterator for LineTiles {
	type Item = (i32, i32);
	fn next(&mut self) -> Option<Self::Item> {
		if self.ex.min(self.ey) <= self.len {
			let old = Some((self.ix as i32, self.iy as i32));
			if self.ex < self.ey {
				self.ex = self.ex + self.dx;
				self.ix = self.ix + self.sx;
			} else {
				self.ey = self.ey + self.dy;
				self.iy = self.iy + self.sy;
			}
			old
		} else {
			None
		}
	}
}

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
			.map(|x| x.supercover())
			.count();

		(0i32..3600)
			.map(|x| x as f32)
			.map(|x| x*::std::f32::consts::PI/1800.0)
			.map(|x| Point(2000.0*x.cos(), 3000.0*x.sin()))
			.map(|x| Line::from_origo(x))
			.map(|x| x.supercover())
			.count();
	}

}
