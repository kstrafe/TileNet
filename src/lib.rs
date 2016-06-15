/// Map for holding a generic tile
///
/// Uses an internal Vec and column-count to store
/// the map in a single array. Uses T::default() for empty
/// elements
struct Map<T> {
	map: Vec<T>,
	cols: usize,
}

impl<T> Map<T> {
	fn get(&self, p: (usize, usize)) -> Option<&T> {
		self.map.get(p.0 + p.1*self.cols)
	}

	fn get_mut(&mut self, p: (usize, usize)) -> Option<&mut T> {
		self.map.get_mut(p.0 + p.1*self.cols)
	}

	fn is_occupied(&self, p: (usize, usize)) {
		self.get(p).is_some();
	}
}

struct RangedMap<'a, T: 'a> {
	map: &'a Map<T>,
}

impl<T> Map<T>
where T: std::default::Default + std::clone::Clone {
	fn new() -> Map<T> {
		Map {
			map: vec![],
			cols: 0,
		}
	}

	fn resize(&mut self, rows: usize, cols: usize) {
		self.map = vec![T::default(); rows*cols];
		self.cols = cols;
	}

	fn range(&self, left: usize, right: usize, bottom: usize, top: usize) -> RangedMap<T> {
		RangedMap {
			map: self,
		}
	}
}

#[derive(Clone)]
enum Tile {
	Empty,
	Dirt,
}

impl std::default::Default for Tile {
	fn default() -> Tile {
		Tile::Empty
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test() {
	}
}
