#![allow(dead_code)]
//! TileNet holds integer aligned tiles for broad phase continuous collision detection

mod defs;

pub use defs::{Index, Line, Point, Quadrant};

fn float_to_coordinate(p: Point) -> Option<Index> {
	if p.0 < 0.0 || p.1 < 0.0 {
		None
	} else {
		Some(Index(p.0.trunc() as usize, p.1.trunc() as usize))
	}
}

fn has_nan(p: &Point) -> bool {
	p.0.is_nan() || p.1.is_nan()
}

fn quadrant(p0: Point, p1: Point) -> Quadrant {
	if p1.0 >= p0.0 {
		if p1.1 >= p0.1 {
			Quadrant::One
		} else {
			Quadrant::Four
		}
	} else {
		if p1.1 >= p0.1 {
			Quadrant::Two
		} else {
			Quadrant::Three
		}
	}
}

fn first_quadrant_equivalent(p0: Point, p1: Point) -> (Point, Point) {
	if has_nan(&p0) || has_nan(&p1) || p1.0 >= p0.0 && p1.1 >= p1.1 {
			return (p0, p1);
	}
	let mut p0: (f32, f32) = (p0.0, p0.1);
	let mut p1: (f32, f32) = (p1.0, p1.1);
	// Mirror around vertical axis
	if p1.0 < p0.0 {
		let distance = p0.0 - p1.0;
		p0.0 = p0.0.floor() + 1.0 - p0.0.fract();
		p1.0 = p0.0 + distance;
	}
	// Mirror around horizontal axis
	if p1.1 < p0.1 {
		let distance = p0.1 - p1.1;
		p0.1 = p0.1.floor() + 1.0 - p0.1.fract();
		p1.1 = p0.1 + distance;
	}
	(Point(p0.0, p0.1), Point(p1.0, p1.1))
}

#[derive(Debug, Eq, PartialEq)]
pub enum Hit {
	Bottom,
	Middle,
	Right,
}

/// Take two points, and find the side of the box
/// that will be intersected.
///
/// The following graph
/// explains the idea.
///
/// ```notrust
/// |--------------------------------------|
/// |                                      |
/// |                                      |
/// |                                      |
/// |                                      |
/// |                                      |
/// |                                      |
/// |                                      |
/// |                        p0            |
/// |                  |  X                |
/// |               dy |   *               |
/// |                  |     *             |
/// |                  |   %   *           |
/// |                        %   *         |
/// |                          %   *       |
/// |                            %   *     |
/// |                              %   *   |
/// |                                %   * |
/// |                                  %   O
/// |                                    % |
/// |--------------------------------------%
/// ```
///
/// X represents our 'p0' point. 'O' the projected
/// intersection with the box. The slope is determined
/// by comparing 'p0' and 'p1'.
/// The slope is used to construct a line from the right
/// corner (the '%' line). This line is extrapolated
/// backwards along the horizontal axis.
/// Then, its height is compared to the height of 'p0'.
///
/// The algorithm works for any positive slope.
///
pub fn right_or_bottom_hit(l: Line) -> Hit {
	let middle = ((l.0).0.floor() + 1.0, (l.0).1.floor() + 1.0);
	let dx = (l.1).0 - (l.0).0;
	let dy = (l.1).1 - (l.0).1;
	let slope = dy/dx;
	let dist_x = middle.0 - (l.0).0;
	let slope_h = middle.1 - dist_x*slope;
	if f32::abs((l.0).1 - slope_h) < std::f32::EPSILON {
		Hit::Middle
	} else if (l.0).1 > slope_h {
		Hit::Bottom
	} else /*if p0.1 < slope_h*/ {
		Hit::Right
	}
}

/// Take a line and find the closest integer boundary from
/// the first point between the second point.
/// You have a point 'X', and another point outside
/// the box. This algorithm finds the closest boundary
/// 'O'. Boxes are integer-aligned.
///
/// |--------------------------------------|
/// |                                      |
/// |                                      |
/// |                                      |
/// |                                      |
/// |                                      |
/// |                                      |
/// |                                      |
/// |                        p0            |-
/// |                     X                || dy
/// |                      *               ||
/// |                        *             ||
/// |                          *           ||
/// |                            *         ||
/// |                              *       ||
/// |                                *     ||
/// |                                  *   ||
/// |                                    * ||
/// |                                      O-
/// |                                      |
/// |--------------------------------------|
///                       |----------------| dx
///
fn truncate_to_boundary(p0: Point, p1: Point) -> Point {
	let dx = p1.0 - p0.0;
	let dy = p1.1 - p0.1;
	// Divide problem into octants
	// Each quadrant has two parts
	// Each part has either dx >= dy or dx < dy
	// Consider edges as their own cases

	if dx > 0.0 && !dy.is_normal() {
		// Edge 0
		let right = p0.0.floor() + 1.0;
		Point(right, p0.1)
	} else if dx > 0.0 && dy > 0.0 && dx > dy {
		// Quadrant 0 - Octant 0
		let right = p0.0.floor() + 1.0;
		let distance = right - p0.0;
		let slope = dy/dx;
		Point(right, distance*slope + p0.1)
	} else if dx > 0.0 && dy > 0.0 && dx == dy {
		Point(0.0, 0.0)
		// Edge quadrant 0 diagonal
	} else if dx > 0.0 && dy > 0.0 && dx < dy {
		// Octant 1
		Point(0.0, 0.0)
	} else {
		Point(0.0, 0.0)
	}
}

fn line_to_indices(p0: Point, p1: Point) {
	let dx = p1.0 - p0.0;
	let dy = p1.1 - p0.1;
	if dy.abs() >= dx.abs() {
		/*
		|--------------------------> x
		|*   |
		|*   |
		| *  |
		| *  |
		|  * | dy
		|--- dx
		v
		y
		*/
	} else {
		/*
		|--------------------------> x
		| ********                |
		|         ********        |
		|                 ********| dy
		v
		y --------------------------dx
		*/
	}
}

#[derive(Clone)]
struct TileSet<'a, T, I> where T: 'a + Clone + std::fmt::Debug, I: Clone + Iterator<Item=(usize, usize)> {
	tilenet: &'a TileNet<T>,
	points: I,
}

impl<'a, T, I> Iterator for TileSet<'a, T, I> where T: 'a + Clone + std::fmt::Debug,
I: Clone + Iterator<Item=(usize, usize)> {
	type Item = &'a Option<T>;
	fn next(&mut self) -> Option<Self::Item> {
		if let Some(point) = self.points.next() {
			self.tilenet.get(point)
		} else {
			None
		}
	}
}

impl<'a, T, I> std::fmt::Debug for TileSet<'a, T, I>
where T: 'a + Clone + std::fmt::Debug, I: Clone + Iterator<Item=(usize, usize)> {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		let viewer = self.clone();
		for tile in viewer {
			try!(write!(formatter, "{:?} ", tile));
		}
		Ok(())
	}
}

#[derive(Clone)]
struct TileView<'a, T> where T: 'a + Clone + std::fmt::Debug {
	tilenet: &'a TileNet<T>,
	rectangle: (usize, usize, usize, usize),
	current: (usize, usize),
}

impl<'a, T> Iterator for TileView<'a, T> where T: 'a + Clone + std::fmt::Debug {
	type Item = &'a Option<T>;
	fn next(&mut self) -> Option<Self::Item> {
		if self.current.1 >= self.rectangle.3 {
			return None;
		}
		let tile = self.tilenet.get(self.current);

		self.current.0 += 1;
		if self.current.0 >= self.rectangle.1 {
			self.current.1 += 1;
			self.current.0 = self.rectangle.0;
		}
		tile
	}
}

impl<'a, T: 'a + std::fmt::Debug> std::fmt::Debug for TileView<'a, T> where T: Clone {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		let biggest = self.clone().map(|x| format!("{:?}", x).len()).max();
		let viewer = self.clone();
		let width = viewer.rectangle.1 - viewer.rectangle.0;
		for (index, tile) in viewer.enumerate() {
			if index % width == 0 && index != 0 {
				try!(formatter.write_str("\n"));
			}
			let mut current = format!("{:?}", tile);
			let length = current.len();
			if let Some(biggest) = biggest {
				(0..biggest-length).map(|_| current.push(' ')).count();
			}
			try!(write!(formatter, "{} ", current));
		}
		Ok(())
	}
}

#[derive(Clone)]
struct TileNet<T> {
	map: Vec<Option<T>>,
	cols: usize,
}

impl<T: std::fmt::Debug> std::fmt::Debug for TileNet<T> {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		let biggest = self.map.iter().map(|x| format!("{:?}", x).len()).max();
		for (index, tile) in self.map.iter().enumerate() {
			if index % self.cols == 0 && index != 0 {
				try!(formatter.write_str("\n"));
			}
			let mut current = format!("{:?}", tile);
			let length = current.len();
			if let Some(biggest) = biggest {
				(0..biggest-length).map(|_| current.push(' ')).count();
			}
			try!(write!(formatter, "{} ", current));
		}
		Ok(())
	}
}

impl TileNet<usize> {
	fn sample() -> TileNet<usize> {
		TileNet::from_iter(10, (1..101).map(|x| if x > 50 { Some(x) } else { None }))
	}
}

impl<T> TileNet<T> where T: Clone + std::fmt::Debug {
	fn new(m: (usize, usize)) -> TileNet<T> {
		TileNet {
			map: vec![None; m.0*m.1],
			cols: m.1,
		}
	}

	fn from_iter<I>(columns: usize, iter: I) -> TileNet<T>
	where I: Iterator<Item=Option<T>> {
		let mut tilenet = TileNet {
			map: vec![],
			cols: columns,
		};
		tilenet.map.extend(iter);
		let remainder = tilenet.map.len() % tilenet.cols;
		if remainder != 0 {
			for _ in 0..tilenet.cols-remainder {
				tilenet.map.push(None);
			}
		}
		tilenet
	}

	fn get_size(&self) -> (usize, usize) {
		(self.map.len()/self.cols, self.cols)
	}

	fn view_box(&self, rectangle: (usize, usize, usize, usize)) -> TileView<T> {
		TileView {
			tilenet: self,
			rectangle: rectangle,
			current: (rectangle.0, rectangle.2),
		}
	}

	fn resize(&mut self, m: (usize, usize)) {
		let mut new_map: Vec<Option<T>> = vec![None; m.0*m.1];
		let new_cols = m.1;
		let new_rows = new_map.len() / new_cols;

		self.map.iter()
			.enumerate()
			.map(|x| (x.0 % self.cols, x.0 / self.cols, x.1))
			.filter(|x| x.0 < new_cols && x.1 < new_rows)
			// .inspect(|x| println!("{:?}", x))
			.inspect(|x| *new_map.get_mut(x.0 + x.1*new_cols).unwrap() = x.2.clone())
			.count();

		self.map = new_map;
		self.cols = new_cols;
	}

	fn get(&self, p: (usize, usize)) -> Option<&Option<T>> {
		if p.0 >= self.cols {
			None
		} else {
			self.map.get(p.0 + p.1*self.cols)
		}
	}

	fn get_mut(&mut self, p: (usize, usize)) -> Option<&mut Option<T>> {
		if p.0 >= self.cols {
			None
		} else {
			self.map.get_mut(p.0 + p.1*self.cols)
		}
	}

	fn is_occupied(&self, p: (usize, usize)) -> bool {
		match self.get(p) {
			Some(tile) => tile.is_some(),
			None => true,
		}
	}

	fn any_occupied<I>(&self, mut list: I) -> bool
	where I: Iterator<Item=(usize, usize)> {
		list.any(|p| self.is_occupied(p))
	}

	fn collide_set<'a, I>(&'a self, list: I) -> TileSet<'a, T, I>
	where I: Clone + Iterator<Item=(usize, usize)> {
		TileSet {
			tilenet: self,
			points: list,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{Index, Line, TileNet, Point};

	#[test]
	fn get() {
		let map: TileNet<usize> = TileNet::new((10, 10));
		assert_eq!(Some(&None), map.get((9, 9)));
		assert_eq!(None, map.get((10, 9)));
		assert_eq!(None, map.get((9, 10)));
	}

	#[test]
	fn get_mut() {
		let mut map: TileNet<usize> = TileNet::new((10, 10));
		*map.get_mut((9, 9)).unwrap() = Some(0);
		assert_eq!(Some(&Some(0)), map.get((9, 9)));
		*map.get_mut((9, 9)).unwrap() = None;
		assert_eq!(Some(&None), map.get((9, 9)));
	}

	#[test]
	fn get_size() {
		let map: TileNet<usize> = TileNet::new((10, 10));
		assert_eq!((10, 10), map.get_size());
		let map: TileNet<usize> = TileNet::new((194, 483));
		assert_eq!((194, 483), map.get_size());
	}

	#[test]
	fn is_occupied() {
		let mut map: TileNet<usize> = TileNet::new((10, 10));
		{
			let element = map.get_mut((9, 9));
			*element.unwrap() = Some(0);
		}
		assert_eq!(true, map.is_occupied((9, 9)));
		assert_eq!(false, map.is_occupied((0, 0)));
		assert_eq!(true, map.is_occupied((10, 0)));
	}

	#[test]
	fn any_occupied() {
		let mut map: TileNet<usize> = TileNet::new((10, 10));
		{
			let element = map.get_mut((9, 9));
			*element.unwrap() = Some(0);
		}
		assert_eq!(true, map.any_occupied(vec![
			(9, 9),
			(10, 0),
		].into_iter()));
		assert_eq!(false, map.any_occupied(
			(0..10).map(|x| (0, x))
		));
	}

	#[test]
	fn resize() {
		let mut map: TileNet<usize> = TileNet::new((10, 10));
		*map.get_mut((0, 0)).unwrap() = Some(0);
		map.resize((5, 5));
		map.resize((10, 10));
	}

	#[test]
	fn from_iter_and_view_box() {
		let map: TileNet<usize> = TileNet::from_iter(10, (1..101).map(|x| Some(x)));
		let mut view = map.view_box((3, 8, 1, 4));
		(14usize..19).chain((24..29)).chain((34..39)).map(|x| assert_eq!(view.next().unwrap(), &Some(x)))
			.count();

		// assert_eq!(
			// (14..19).chain((24..29)).chain((34..39)),
			// map.view_box((3, 8, 1, 4)));
	}

	#[test]
	fn from_iter_with_remainder() {
		let map: TileNet<usize> = TileNet::from_iter(10, (1..25).map(|x| Some(x)));
		let mut view = map.view_box((0, 10, 0, 3));
		for x in (1..31).map(|x| if x >= 25 { None } else { Some(x) }) {
			assert_eq!(view.next().unwrap(), &x);
		}

		let map: TileNet<usize> = TileNet::from_iter(10, (1..31).map(|x| Some(x)));
		let mut view = map.view_box((0, 10, 0, 3));
		for x in 1..31 {
			assert_eq!(view.next().unwrap(), &Some(x));
		}
	}

	#[test]
	fn collide_set() {
		fn vectorize(right: usize) -> (usize, usize) {
			(4, right)
		}
		type TypeFn = fn(right: usize) -> (usize, usize);
		let function: TypeFn = vectorize;
		let map: TileNet<usize> = TileNet::from_iter(10, (1..101).map(|x| if x > 50 { Some(x) } else { None }));
		let mut set = map.collide_set((3..7).map(function));
		for _ in 1..3 {
			assert_eq!(set.next().unwrap(), &None);
		}
		for x in 0..2 {
			assert_eq!(set.next().unwrap(), &Some(55 + 10*x));
		}
	}

	#[test]
	fn any_occupied_bounds() {
		let map = TileNet::sample();
		assert_eq!(true, map.any_occupied((3..).map(|x| (x, 3))));
	}

	#[test]
	fn float_to_coordinate() {
		let ftc = ::float_to_coordinate;
		(0..100)
			.map(|x| Point(x as f32, x as f32))
			.inspect(|x| assert_eq!(ftc(*x), Some(Index(x.0 as usize, x.1 as usize))))
			.count();
		assert_eq!(ftc(Point(0.0, 0.0)), Some(Index(0, 0)));
		assert_eq!(ftc(Point(0.5, 0.75)), Some(Index(0, 0)));
		assert_eq!(ftc(Point(1.5, 0.75)), Some(Index(1, 0)));
		assert_eq!(ftc(Point(1.5, 1.75)), Some(Index(1, 1)));
		assert_eq!(ftc(Point(1.5, 5.25)), Some(Index(1, 5)));
		assert_eq!(ftc(Point(-1.5, 5.25)), None);
		assert_eq!(ftc(Point(1.5, -5.25)), None);
	}

	#[test]
	fn truncate_to_boundary() {
		let tr = ::truncate_to_boundary;
		assert_eq!(tr(Point(0.5, 0.0), Point(1.5, 0.0)), Point(1.0, 0.0));
	}

	#[test]
	fn right_or_bottom_hit() {
		use super::Hit;
		let rob = ::right_or_bottom_hit;
		// Sample points
		assert_eq!(rob(Line(Point(0.5, 0.5), Point(1.5, 1.5))), Hit::Middle);
		assert_eq!(rob(Line(Point(0.5, 0.0), Point(1.5, 1.0))), Hit::Right);
		assert_eq!(rob(Line(Point(0.5, 0.75), Point(1.5, 1.75))), Hit::Bottom);
		assert_eq!(rob(Line(Point(0.0, 0.0), Point(1.1, 1.0))), Hit::Right);
		assert_eq!(rob(Line(Point(0.0, 0.0), Point(0.9, 1.0))), Hit::Bottom);
		assert_eq!(rob(Line(Point(0.5, 0.0), Point(1.0, 1.0))), Hit::Middle);

		// Check vectors through the midpoint
		for x in 0..100 {
			for y in 0..100 {
				let x = x as f32 / 100.0;
				let y = y as f32 / 100.0;
				assert_eq!(rob(Line(Point(x, y), Point(1.0, 1.0))), Hit::Middle);
			}
		}

		// Check vectors through the bottom using triangle
		// integrals. The fors should draw a triangle.
		for x in 0..100 {
			for y in x+1..100 {
				let x = x as f32 / 100.0;
				let y = y as f32 / 100.0;
				assert_eq!(rob(Line(Point(x, y), Point(2.0, 2.0))), Hit::Bottom);
			}
		}

		// Check from block 0 to point (1, 2), everything should
		// bottom out.
		for x in 0..100 {
			for y in 0..100 {
				let x = x as f32 / 100.0;
				let y = y as f32 / 100.0;
				assert_eq!(rob(Line(Point(x, y), Point(1.0, 2.0))), Hit::Bottom);
			}
		}

		// Check vectors through the top using triangle
		// integrals. The fors should draw a triangle.
		for x in 0..100 {
			for y in 0..x-1 {
				let x = x as f32 / 100.0;
				let y = y as f32 / 100.0;
				assert_eq!(rob(Line(Point(x, y), Point(2.0, 2.0))), Hit::Right);
			}
		}

		// Check from block 0 to point (2, 1), everything should
		// right out.
		for x in 0..100 {
			for y in 0..100 {
				let x = x as f32 / 100.0;
				let y = y as f32 / 100.0;
				assert_eq!(rob(Line(Point(x, y), Point(2.0, 1.0))), Hit::Right);
			}
		}

		// Slope dy/dx = 0, dx = 1
		assert_eq!(rob(Line(Point(0.5, 0.5), Point(1.5, 0.5))), Hit::Right);
	}

	#[test]
	fn first_quadrant_equivalent() {
		let fqe = ::first_quadrant_equivalent;
		assert_eq!(fqe(Point(0.0, 0.0), Point(-1.0, 0.0)), (Point(1.0, 0.0), Point(2.0, 0.0)));
		assert_eq!(fqe(Point(0.5, 0.5), Point(-0.5, -0.5)), (Point(0.5, 0.5), Point(1.5, 1.5)));
		assert_eq!(fqe(Point(0.5, 0.5), Point(-0.5, 0.5)), (Point(0.5, 0.5), Point(1.5, 0.5)));
	}

	#[test]
	fn quadrant() {
		use super::Quadrant;
		let q = ::quadrant;
		assert_eq!(q(Point(0.5, 0.5), Point(-0.5, 0.5)), Quadrant::Two);
		assert_eq!(q(Point(0.5, 0.5), Point(0.5, 0.5)), Quadrant::One);
		assert_eq!(q(Point(0.5, 0.5), Point(1.0, 0.5)), Quadrant::One);
		assert_eq!(q(Point(0.5, 0.5), Point(1.5, -0.5)), Quadrant::Four);
		assert_eq!(q(Point(0.5, 0.5), Point(0.5, 1.0)), Quadrant::One);
	}
}
