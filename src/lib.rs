/// TileNet for holding a generic tile
///
/// Uses an internal Vec and column-count to store
/// the map in a single array. Uses T::default() for empty
/// elements

struct TileView<'a, T> where T: 'a + Clone + std::fmt::Debug {
	tilenet: &'a TileNet<T>,
	rectangle: (usize, usize, usize, usize),
	current: (usize, usize),
}

impl<'a, T> Iterator for TileView<'a, T> where T: 'a + Clone + std::fmt::Debug {
	type Item = &'a Option<T>;
	fn next(&mut self) -> Option<Self::Item> {
		let tile = self.tilenet.get(self.current);

		self.current.0 += 1;
		if self.current.0 >= self.rectangle.1 {
			self.current.1 += 1;
			self.current.0 = 0;
		}
		if self.current.1 >= self.rectangle.3 {
			None
		} else {
			tile
		}
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
				(0..biggest-length).map(|x| current.push(' ')).count();
			}
			write!(formatter, "{}", current);
		}
		Ok(())
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
}

#[cfg(test)]
mod tests {
	use super::TileNet;

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
		let mut map: TileNet<usize> = TileNet::new((10, 10));
		assert_eq!((10, 10), map.get_size());
		let mut map: TileNet<usize> = TileNet::new((194, 483));
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
	fn from_iter() {
		let map: TileNet<usize> = TileNet::from_iter(10, (1..101).map(|x| Some(x)));
		println!("{:?}", map);
	}
}
