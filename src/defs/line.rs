pub use super::{Point, Quadrant};

const MAX: f32 = 16777216 as f32;
const MIN: f32 = -16777216 as f32;

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
	pub fn from_origin(p: Point) -> Line {
		Line(Point(0.0, 0.0), p)
	}

	/// Show us which quadrant this vector points to
	///
	/// The quadrant is the second (end) point relative
	/// to the beginning point. See more in-depth rules
	/// about edge cases in `tile_net::Quadrant`.
	///
	/// ```
	/// use tile_net::{Line, Point, Quadrant};
	/// assert_eq!(Line::from_origin(Point(-3.0, 0.0)).quadrant(), Quadrant::Three);
	/// ```
	pub fn quadrant(&self) -> Quadrant {
		if (self.1).0 <= (self.0).0 && (self.1).1 > (self.0).1 {
			Quadrant::Two
		} else if (self.1).0 < (self.0).0 && (self.1).1 <= (self.0).1 {
			Quadrant::Three
		} else if (self.1).0 >= (self.0).0 && (self.1).1 < (self.0).1 {
			Quadrant::Four
		} else {
			Quadrant::One
		}
	}

	/// Create a supercover line iterator
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
		let slope_x = 1.0 + vy * vy / vx / vx;
		let slope_y = 1.0 + vx * vx / vy / vy;
		let (dx, dy) = (slope_x.sqrt(), slope_y.sqrt());

		let (ix, iy) = (start.0.floor() as i32, start.1.floor() as i32);

		let (sx, sy);
		let (ex, ey);

		if vx < 0.0 {
			sx = -1;
			ex = start.0.fract() * dx;
		} else {
			sx = 1;
			ex = (1.0 - start.0.fract()) * dx;
		}

		if vy < 0.0 {
			sy = -1;
			ey = start.1.fract() * dy;
		} else {
			sy = 1;
			ey = (1.0 - start.1.fract()) * dy;
		}

		let len = vx.floor().abs() as usize + vy.floor().abs() as usize;

		LineTiles {
			it: 0,
			len: len,
			dx: dx,
			dy: dy,
			sx: sx,
			sy: sy,
			ex: ex,
			ey: ey,
			ix: ix,
			iy: iy,
			dest_x: stop.0.floor() as i32,
			dest_y: stop.1.floor() as i32,
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
/// 		println!("{:?}", tile);
/// }
/// ```
#[derive(Clone)]
pub struct LineTiles {
	it: usize,
	len: usize,
	dx: f32,
	dy: f32,
	sx: i32,
	sy: i32,
	ex: f32,
	ey: f32,
	ix: i32,
	iy: i32,
	dest_x: i32,
	dest_y: i32,
}

impl LineTiles {
	fn minimize_distance_from_zero(&mut self) {
		let minimal = self.ex.min(self.ey);
		self.ex -= minimal;
		self.ey -= minimal;
	}

	fn step_to_next_tile(&mut self) {
		if self.ex < self.ey {
			self.ex += self.dx;
			self.ix += self.sx;
		} else {
			self.ey += self.dy;
			self.iy += self.sy;
		}
	}
}

impl Iterator for LineTiles {
	type Item = (i32, i32);
	fn next(&mut self) -> Option<Self::Item> {
		if self.it < self.len {
			let old = Some((self.ix, self.iy));
			self.it += 1;
			self.step_to_next_tile();
			self.minimize_distance_from_zero();
			old
		} else if self.it == self.len {
			self.it += 1;
			Some((self.dest_x, self.dest_y))
		} else {
			None
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{Line, Point};
	const UPPER_FLOAT: f32 = 16777216 as f32;
	const LOWER_FLOAT: f32 = -16777216 as f32;

	fn seq<I>(point: (f32, f32), iter: I) -> bool
		where I: Iterator<Item = (i32, i32)>
	{
		Line::from_origin(Point(point.0, point.1)).supercover().eq(iter)
	}

	fn last(point: (i32, i32)) -> bool {
		match Line::from_origin(Point(point.0 as f32, point.1 as f32))
			.supercover()
			.last()
			.map(|x| point.0 == x.0 && point.1 == x.1) {
			Some(boolean) => boolean,
			None => false,
		}
	}

	#[test]
	fn supercover() {
		assert!(seq((0.0, -0.1), (0..2).map(|x| (0, -x))));
		assert!(seq((0.0, 0.1), (0..1).map(|_| (0, 0))));
		assert!(seq((0.9, 0.99), (0..1).map(|_| (0, 0))));
		assert!(seq((0.9999, 0.999999), (0..1).map(|_| (0, 0))));
		assert!(seq((0.9999, 1.00001), (0..2).map(|x| (0, x))));
		assert!(seq((10.0, 0.0), (0..11).map(|x| (x, 0))));
		assert!(seq((0.0, 10.0), (0..11).map(|x| (0, x))));
		assert!(last((1, 2)));
		assert!(last((1, 16777216)));
		assert!(last((1, 2)));
	}

}
