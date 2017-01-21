extern crate interleave;

use std::fmt;
use super::{TileView, TileSet};

/// `TileNet` is the main class in this library
///
/// It represents a 2D space with dimensions rows x columns.
/// Each index represents a point in space. Row 'n' and column 'm' denote the tile
/// from x from n inclusive to n+1 exclusive, and y from m inclusive to  m+1 exclusive.
///
/// ```
/// use tile_net::TileNet;
/// #[derive(Clone, Debug, Default)]
/// struct Example(i32);
/// let my_net = TileNet::<Example>::new(10, 10);
/// println!("{:?}", my_net);
/// ```
#[derive(Clone)]
pub struct TileNet<T> {
	map: Vec<T>,
	cols: usize,
}

impl<T: fmt::Debug> fmt::Debug for TileNet<T> {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
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
	/// Create a simple sample grid
	pub fn sample() -> TileNet<usize> {
		TileNet::from_iter(10, (1..101).map(|x| if x > 50 { x } else { 0 }))
	}
}

/// Proxy for editing the `TileNet`
///
/// Useful when editing and requiring a span of changed tiles.
/// The span can be requested at any time. This can be used to
/// update other states. One particular example is to upload just
/// that square to the GPU to render on a texture.
pub struct TileNetProxy<'a, T: 'a> {
	tilenet: &'a mut TileNet<T>,
	min_x: usize,
	max_x: usize,
	min_y: usize,
	max_y: usize,
}

/// Actual span of changed tiles, given as a square
pub type Span = (usize, usize, usize, usize);
impl<'a, T> TileNetProxy<'a, T>
    where T: Clone
{
	/// Get the span of the changes made
	pub fn get_span(&self) -> Span {
		(self.min_x, self.min_y, self.max_x, self.max_y)
	}

	/// Set a box
	///
	/// Start should be less than stop
	pub fn set_box(&mut self, value: &T, start: (usize, usize), stop: (usize, usize)) -> Span {
		self.tilenet.set_box(value, start, stop);
		if start.0 < self.min_x {
			self.min_x = start.0;
		}
		if start.1 < self.min_y {
			self.min_y = start.1;
		}
		if stop.0 > self.max_x {
			self.max_x = stop.0;
		}
		if stop.1 > self.max_y && stop.1 < self.tilenet.get_size().1 {
			self.max_y = stop.1;
		}
		self.get_span()
	}

	/// Set an entire row
	pub fn set_row(&mut self, value: &T, row: usize) -> Span {
		self.tilenet.set_row(value, row);
		self.min_x = 0;
		self.max_x = self.tilenet.get_size().0;
		if row < self.min_y {
			self.min_y = row;
		}
		if row > self.max_y && row < self.tilenet.get_size().1 {
			self.max_y = row;
		}
		self.get_span()
	}

	/// Set an entire column
	pub fn set_col(&mut self, value: &T, col: usize) -> Span {
		self.tilenet.set_col(value, col);
		self.min_y = 0;
		self.max_y = self.tilenet.get_size().1;
		if col < self.min_x {
			self.min_x = col;
		}
		if col > self.max_x && col < self.tilenet.get_size().0 {
			self.max_x = col;
		}
		self.get_span()
	}

	/// Set a single grid point
	pub fn set(&mut self, value: &T, p: (usize, usize)) -> Span {
		self.tilenet.set(value, p);
		if p.0 < self.min_x {
			self.min_x = p.0;
		}
		if p.1 < self.min_y {
			self.min_y = p.1;
		}
		if p.0 > self.max_x && p.0 < self.tilenet.get_size().0 {
			self.max_x = p.0;
		}
		if p.1 > self.max_y && p.1 < self.tilenet.get_size().1 {
			self.max_y = p.1;
		}
		self.get_span()
	}
}

/// Ensure that
fn positivize_isize(value: isize) -> usize {
	if value < 0 {
		0
	} else {
		value as usize
	}
}

fn positivize_2_tuple(value: (isize, isize)) -> (usize, usize) {
	(positivize_isize(value.0), positivize_isize(value.1))
}

impl<T> TileNet<T>
    where T: Clone
{
	/// Create a proxy to be able to get the span of a change
	pub fn prepare(&mut self) -> TileNetProxy<T> {
		let size = self.get_size();
		TileNetProxy {
			tilenet: self,
			min_x: size.0,
			max_x: 0,
			min_y: size.1,
			max_y: 0,
		}
	}

	/// Get the raw array behind the tilenet
	pub fn get_raw(&self) -> &[T] {
		self.map.as_slice()
	}


	/// Use isizes to denote indices to prevent underflow
	pub fn set_box_isize(&mut self, value: &T, start: (isize, isize), stop: (isize, isize)) {
		let new_start = positivize_2_tuple(start);
		let new_stop = positivize_2_tuple(stop);
		self.set_box(value, new_start, new_stop);
	}

	/// Set a box in the tilenet
	pub fn set_box(&mut self, value: &T, start: (usize, usize), stop: (usize, usize)) {
		for i in start.1..stop.1 {
			for j in start.0..stop.0 {
				self.set(value, (j, i));
			}
		}
		for j in start.0..stop.0 {
			self.set(value, (j, stop.1));
		}
		for i in start.1..stop.1 {
			self.set(value, (stop.0, i));
		}
		self.set(value, (stop.0, stop.1));
	}

	/// Set a row
	pub fn set_row(&mut self, value: &T, row: usize) {
		for i in 0..self.col_count() {
			self.set(value, (i, row));
		}
	}

	/// Set a column
	pub fn set_col(&mut self, value: &T, col: usize) {
		for i in 0..self.row_count() {
			self.set(value, (col, i));
		}
	}

	/// Set a single grid point
	pub fn set(&mut self, value: &T, p: (usize, usize)) {
		if let Some(old) = self.get_mut(p) {
			*old = value.clone();
		}
	}
}

impl<T> TileNet<T>
    where T: Clone + Default
{
	/// Create a new tilenet of the size (cols, rows)
	///
	/// The tiles will be Default-created
	pub fn new(x: usize, y: usize) -> TileNet<T> {
		TileNet {
			map: vec![T::default(); x * y],
			cols: x,
		}
	}

	/// Resize the grid
	///
	/// If the grid grows, new tiles will be Default-created
	pub fn resize(&mut self, m: (usize, usize)) {
		let mut new_map: Vec<T> = vec![T::default(); m.0*m.1];
		let new_cols = m.1;
		let new_rows = new_map.len() / new_cols;

		self.map
			.iter()
			.enumerate()
			.map(|x| (x.0 % self.cols, x.0 / self.cols, x.1))
			.filter(|x| x.0 < new_cols && x.1 < new_rows)
			.inspect(|x| new_map[x.0 + x.1 * new_cols] = x.2.clone())
			.count();

		self.map = new_map;
		self.cols = new_cols;
	}
}

impl<T> TileNet<T>
    where T: Default
{
	/// Create a tilenet from an iterator
	///
	/// Takes a column count and an iterator.
	/// If the iterator does not describe the entire box
	/// the remaining elements are filled in by Default.
	pub fn from_iter<I>(columns: usize, iter: I) -> TileNet<T>
		where I: Iterator<Item = T>
	{
		let mut tilenet = TileNet {
			map: vec![],
			cols: columns,
		};
		tilenet.map.extend(iter);
		let remainder = tilenet.map.len() % tilenet.cols;
		if remainder != 0 {
			for _ in 0..tilenet.cols - remainder {
				tilenet.map.push(T::default());
			}
		}
		tilenet
	}
}

impl<T> TileNet<T> {
	/// Compute the row count
	pub fn row_count(&self) -> usize {
		self.map.len() / self.cols
	}

	/// Get the column count
	pub fn col_count(&self) -> usize {
		self.cols
	}

	/// Get a reference to a 2D index
	pub fn get(&self, p: (usize, usize)) -> Option<&T> {
		if p.0 >= self.cols {
			None
		} else {
			self.map.get(p.0 + p.1 * self.cols)
		}
	}

	/// Get a tuple that describes the size as (cols, rows)
	pub fn get_size(&self) -> (usize, usize) {
		(self.cols, self.row_count())
	}

	/// Create a proxy view that iterates over all tiles
	pub fn view_all(&self) -> TileView<T> {
		TileView::new(self, (0, self.cols, 0, self.map.len() / self.cols))
	}

	/// Create a proxy view with a span from the center using a float position
	pub fn view_center_f32(&self, position: (f32, f32), span: (usize, usize)) -> TileView<T> {
		let position = (position.0.max(0.0) as usize, position.1.max(0.0) as usize);
		self.view_center(position, span)
	}

	/// Create a proxy view with a span from the center using an integer position
	pub fn view_center(&self, position: (usize, usize), span: (usize, usize)) -> TileView<T> {
		let left = position.0.checked_sub(span.0).unwrap_or(0);
		let top = position.1.checked_sub(span.1).unwrap_or(0);
		let right = position.0 + span.0;
		let bottom = position.1 + span.1;
		TileView::new(self, (left, right, top, bottom))
	}

	/// Create a view box that iterates over tiles within that box
	pub fn view_box(&self, rectangle: (usize, usize, usize, usize)) -> TileView<T> {
		TileView::new(self, rectangle)
	}

	/// Get a mutable reference to a tile
	pub fn get_mut(&mut self, p: (usize, usize)) -> Option<&mut T> {
		if p.0 >= self.cols {
			None
		} else {
			self.map.get_mut(p.0 + p.1 * self.cols)
		}
	}

	/// Create an iterator of tiles from an iterator over indices
	pub fn collide_set<I>(&self, list: I) -> TileSet<T, I>
		where I: Iterator<Item = (i32, i32)>
	{
		TileSet {
			tilenet: self,
			points: list,
			last_coord: (0, 0),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_proxy() {
		let mut net: TileNet<usize> = TileNet::new(15, 10);
		let mut net = net.prepare();
		net.set_box(&1, (2, 2), (4, 6));
		let span = net.set_box(&2, (3, 3), (14, 6));
		assert_eq![span, (2, 2, 14, 6)];
	}
}
