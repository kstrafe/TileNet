use std::slice::Iter;
/// Map for holding a generic tile
///
/// Uses an internal Vec and column-count to store
/// the map in a single array. Uses T::default() for empty
/// elements
struct Map<T> {
	map: Vec<Option<T>>,
	cols: usize,
}

impl<T> Map<T> where T: Clone {
	fn new(m: (usize, usize)) -> Map<T> {
		Map {
			map: vec![None; m.0*m.1],
			cols: m.1,
		}
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

	fn any_occupied<'a, I>(&self, mut list: I) -> bool
		where I: Iterator<Item=(usize, usize)> {
		list.any(|p| self.is_occupied(p))
	}
}

#[cfg(test)]
mod tests {
	use super::Map;

	#[test]
	fn get() {
		let map: Map<usize> = Map::new((10, 10));
		assert_eq!(Some(&None), map.get((9, 9)));
		assert_eq!(None, map.get((10, 9)));
		assert_eq!(None, map.get((9, 10)));
	}

	#[test]
	fn is_occupied() {
		let mut map: Map<usize> = Map::new((10, 10));
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
		let mut map: Map<usize> = Map::new((10, 10));
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
}
