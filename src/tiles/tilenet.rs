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
/// let my_net = TileNet::<Example>::new((10, 10));
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
	pub fn sample() -> TileNet<usize> {
		TileNet::from_iter(10, (1..101).map(|x| if x > 50 { x } else { 0 }))
	}
}

impl<T> TileNet<T>
    where T: Clone
{
	pub fn get_raw(&self) -> &[T] {
		self.map.as_slice()
	}

	pub fn set_box(&mut self, value: &T, start: (usize, usize), stop: (usize, usize)) {
		for i in start.1..stop.1 {
			for j in start.0..stop.0 {
				self.set(value, (j, i));
			}
		}
	}

	pub fn set_row(&mut self, value: &T, row: usize) {
		for i in 0..self.col_count() {
			self.set(value, (i, row));
		}
	}

	pub fn set_col(&mut self, value: &T, col: usize) {
		for i in 0..self.row_count() {
			self.set(value, (col, i));
		}
	}

	pub fn set(&mut self, value: &T, p: (usize, usize)) {
		if let Some(old) = self.get_mut(p) {
			*old = value.clone();
		}
	}
}

impl<T> TileNet<T>
    where T: Clone + Default
{
	pub fn new(m: (usize, usize)) -> TileNet<T> {
		TileNet {
			map: vec![T::default(); m.0*m.1],
			cols: m.1,
		}
	}

	pub fn resize(&mut self, m: (usize, usize)) {
		let mut new_map: Vec<T> = vec![T::default(); m.0*m.1];
		let new_cols = m.1;
		let new_rows = new_map.len() / new_cols;

		self.map
			.iter()
			.enumerate()
			.map(|x| (x.0 % self.cols, x.0 / self.cols, x.1))
			.filter(|x| x.0 < new_cols && x.1 < new_rows)
			.inspect(|x| *new_map.get_mut(x.0 + x.1 * new_cols).unwrap() = x.2.clone())
			.count();

		self.map = new_map;
		self.cols = new_cols;
	}
}

impl<T> TileNet<T>
    where T: Default
{
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
	pub fn row_count(&self) -> usize {
		self.map.len() / self.cols
	}

	pub fn col_count(&self) -> usize {
		self.cols
	}

	pub fn get(&self, p: (usize, usize)) -> Option<&T> {
		if p.0 >= self.cols {
			None
		} else {
			self.map.get(p.0 + p.1 * self.cols)
		}
	}

	pub fn get_size(&self) -> (usize, usize) {
		(self.row_count(), self.cols)
	}

	pub fn view_all(&self) -> TileView<T> {
		TileView::new(self, (0, self.cols, 0, self.map.len() / self.cols))
	}

	pub fn view_center_f32(&self, position: (f32, f32), span: (usize, usize)) -> TileView<T> {
		let position = (position.0.max(0.0) as usize, position.1.max(0.0) as usize);
		self.view_center(position, span)
	}

	pub fn view_center(&self, position: (usize, usize), span: (usize, usize)) -> TileView<T> {
		let left = position.0.checked_sub(span.0).unwrap_or(0);
		let top = position.1.checked_sub(span.1).unwrap_or(0);
		let right = position.0 + span.0;
		let bottom = position.1 + span.1;
		TileView::new(self, (left, right, top, bottom))
	}

	pub fn view_box(&self, rectangle: (usize, usize, usize, usize)) -> TileView<T> {
		TileView::new(self, rectangle)
	}
	pub fn get_mut(&mut self, p: (usize, usize)) -> Option<&mut T> {
		if p.0 >= self.cols {
			None
		} else {
			self.map.get_mut(p.0 + p.1 * self.cols)
		}
	}

	pub fn collide_set<I>(&self, list: I) -> TileSet<T, I>
		where I: Iterator<Item = (i32, i32)>
	{
		TileSet {
			tilenet: self,
			points: list,
		}
	}
}
