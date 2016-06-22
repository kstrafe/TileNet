#![allow(dead_code)]
//! TileNet holds integer aligned tiles for broad phase continuous collision detection
//!

mod defs;

pub use defs::{Rect, Line, Point, Quadrant};

/// Tile iterator returning tiles from the `tile_net::TileNet`.
///
/// ```
/// use tile_net::{Line, Point, TileNet};
///
/// let map: TileNet<usize> = TileNet::sample();
/// let iter = (0..10).map(|x| (x, 3));
/// let set = map.collide_set(iter);
/// set.map(|x| println!("{:?}", x)).count();
///
/// let map = TileNet::sample();
/// let line = Line::from_origin(Point(10.0, 5.0));
/// let cover = line.supercover();
/// let set = map.collide_set(cover);
/// for tile in set {
/// 	println!("{:?}", tile);
/// }
/// ```
///
/// The ideal version of this library is quite simple:
///
/// ```ignore
/// use tile_net::{TileNet, Rect, Line};
/// let map: TileNet<MyTile> = TileNet::new((1000, 1000));
/// initialize_map(&mut map);
/// 'main: loop {
/// 	handle_events(&mut world);
/// 	// Physics for collidable units
/// 	for coll in world.collidables_mut() {
/// 		match map.collides(coll) {
/// 			TileNet::NoCollision => coll.allow_move(),
/// 			TileNet::XCollision => coll.allow_move_y(),
/// 			TileNet::YCollision => coll.allow_move_x(),
/// 			TileNet::FullCollision => ,
/// 		}
///   }
/// }
/// ```
#[derive(Clone)]
pub struct TileSet<'a, T, I>
	where T: 'a + Clone + std::fmt::Debug,
	      I: Iterator<Item = (i32, i32)>
{
	tilenet: &'a TileNet<T>,
	points: I,
}

impl<'a, T, I> Iterator for TileSet<'a, T, I>
	where T: 'a + Clone + std::fmt::Debug,
	      I: Iterator<Item = (i32, i32)>
{
	type Item = &'a Option<T>;
	fn next(&mut self) -> Option<Self::Item> {
		if let Some(point) = self.points.next() {
			if point.0 >= 0 && point.1 >= 0 {
				self.tilenet.get((point.0 as usize, point.1 as usize))
			} else {
				None
			}
		} else {
			None
		}
	}
}

impl<'a, T, I> std::fmt::Debug for TileSet<'a, T, I>
	where T: 'a + Clone + std::fmt::Debug,
	      I: Clone + Iterator<Item = (i32, i32)>
{
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		let viewer = self.clone();
		for tile in viewer {
			try!(write!(formatter, "{:?} ", tile));
		}
		Ok(())
	}
}

/// Tile iterator for a rectangular view of the `tile_net::TileNet`.
#[derive(Clone)]
pub struct TileView<'a, T>
	where T: 'a + Clone + std::fmt::Debug
{
	tilenet: &'a TileNet<T>,
	rectangle: (usize, usize, usize, usize),
	current: (usize, usize),
}

impl<'a, T> Iterator for TileView<'a, T>
    where T: 'a + Clone + std::fmt::Debug
{
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

impl<'a, T: 'a + std::fmt::Debug> std::fmt::Debug for TileView<'a, T>
    where T: Clone
{
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
				(0..biggest - length).map(|_| current.push(' ')).count();
			}
			try!(write!(formatter, "{} ", current));
		}
		Ok(())
	}
}

/// `TileNet` is the main class in this library
///
/// It represents a 2D space with dimensions rows x columns.
/// Each index represents a point in space. Row 'n' and column 'm' denote the tile
/// from x from n inclusive to n+1 exclusive, and y from m inclusive to  m+1 exclusive.
///
/// ```
/// use tile_net::TileNet;
/// #[derive(Clone)]
/// struct Example(i32);
/// let my_net = TileNet::<Example>::new((10, 10));
/// ```
#[derive(Clone)]
pub struct TileNet<T> {
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
				(0..biggest - length).map(|_| current.push(' ')).count();
			}
			try!(write!(formatter, "{} ", current));
		}
		Ok(())
	}
}

impl TileNet<usize> {
	pub fn sample() -> TileNet<usize> {
		TileNet::from_iter(10,
		                   (1..101).map(|x| if x > 50 {
			                   Some(x)
			                  } else {
			                   None
			                  }))
	}
}

impl<T> TileNet<T>
    where T: Clone
{
	pub fn new(m: (usize, usize)) -> TileNet<T> {
		TileNet {
			map: vec![None; m.0*m.1],
			cols: m.1,
		}
	}
}

impl<T> TileNet<T>
    where T: Clone + std::fmt::Debug
{
	pub fn from_iter<I>(columns: usize, iter: I) -> TileNet<T>
		where I: Iterator<Item = Option<T>>
	{
		let mut tilenet = TileNet {
			map: vec![],
			cols: columns,
		};
		tilenet.map.extend(iter);
		let remainder = tilenet.map.len() % tilenet.cols;
		if remainder != 0 {
			for _ in 0..tilenet.cols - remainder {
				tilenet.map.push(None);
			}
		}
		tilenet
	}

	pub fn get_size(&self) -> (usize, usize) {
		(self.map.len() / self.cols, self.cols)
	}

	pub fn view_box(&self, rectangle: (usize, usize, usize, usize)) -> TileView<T> {
		TileView {
			tilenet: self,
			rectangle: rectangle,
			current: (rectangle.0, rectangle.2),
		}
	}

	pub fn resize(&mut self, m: (usize, usize)) {
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

	pub fn get(&self, p: (usize, usize)) -> Option<&Option<T>> {
		if p.0 >= self.cols {
			None
		} else {
			self.map.get(p.0 + p.1 * self.cols)
		}
	}

	pub fn get_mut(&mut self, p: (usize, usize)) -> Option<&mut Option<T>> {
		if p.0 >= self.cols {
			None
		} else {
			self.map.get_mut(p.0 + p.1 * self.cols)
		}
	}

	pub fn is_occupied(&self, p: (usize, usize)) -> bool {
		match self.get(p) {
			Some(tile) => tile.is_some(),
			None => true,
		}
	}

	pub fn any_occupied<I>(&self, mut list: I) -> bool
		where I: Iterator<Item = (usize, usize)>
	{
		list.any(|p| self.is_occupied(p))
	}

	pub fn collide_set<'a, I>(&'a self, list: I) -> TileSet<'a, T, I>
		where I: Iterator<Item = (i32, i32)>
	{
		TileSet {
			tilenet: self,
			points: list,
		}
	}

	pub fn collision_between<'a, I>(&'a self, rect_a: &Rect, rect_b: &Rect) -> TileSet<'a, T, I>
		where I: Iterator<Item = (i32, i32)>
	{
		unimplemented!();
		// find all points in rect_a
		// find all points in rect_b
		// find all points from respective points of rectangles
		// Return the tileset of those points
	}
}

#[cfg(test)]
mod tests {
	use super::{Line, TileNet, Point};

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
		assert_eq!(true,
		           map.any_occupied(vec![
			(9, 9),
			(10, 0),
		]
			           .into_iter()));
		assert_eq!(false, map.any_occupied((0..10).map(|x| (0, x))));
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
		(14usize..19)
			.chain((24..29))
			.chain((34..39))
			.map(|x| assert_eq!(view.next().unwrap(), &Some(x)))
			.count();
	}

	#[test]
	fn from_iter_with_remainder() {
		let map: TileNet<usize> = TileNet::from_iter(10, (1..25).map(|x| Some(x)));
		let mut view = map.view_box((0, 10, 0, 3));
		for x in (1..31).map(|x| if x >= 25 {
			None
		} else {
			Some(x)
		}) {
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
		let map: TileNet<usize> = TileNet::from_iter(10,
		                                             (1..101).map(|x| if x > 50 {
			                                             Some(x)
			                                            } else {
			                                             None
			                                            }));
		let mut set = map.collide_set((3..7).map(|x| (4, x)));
		for _ in 1..3 {
			assert_eq!(set.next().unwrap(), &None);
		}
		for x in 0..2 {
			assert_eq!(set.next().unwrap(), &Some(55 + 10 * x));
		}
	}

	#[test]
	fn any_occupied_bounds() {
		let map = TileNet::sample();
		assert_eq!(true, map.any_occupied((3..).map(|x| (x, 3))));
	}

}
