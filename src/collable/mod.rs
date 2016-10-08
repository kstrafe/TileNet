pub use super::{SuperCover, Line, Vector, TileNet, TileSet};

pub use interleave::{IterList, MultiIter};

/// A vertex iterator.
///
/// Used internally by the collision engine. It combines static
/// points with an offset. Every iteration returns the point + offset.
pub struct Points<'a> {
	index: usize,
	offset: Vector,
	points: &'a [(f32, f32)],
}

impl<'a> Points<'a> {
	pub fn new(offset: Vector, points: &'a [(f32, f32)]) -> Points {
		Points {
			index: 0,
			offset: offset,
			points: points,
		}
	}
}

impl<'a> Iterator for Points<'a> {
	type Item = (f32, f32);
	fn next(&mut self) -> Option<Self::Item> {
		let ret = self.points
			.get(self.index)
			.cloned()
			.map(|x| Vector::from_tuple(x) + self.offset)
			.map(|x| (x.0, x.1));
		self.index += 1;
		ret
	}
}

/// Trait for dynamic objects so they can easily check collisions with the `TileMap`
pub trait Collable<T> {
	/// Returns the set of points associated with this object. These points are used to
	/// draw lines to their respective next points. For a rectangle, the four courners
	/// may be points. For a circle, a whole bunch of points may be defined.
	fn points(&self) -> Points;

	/// Get the previously queued move. Should reasonably return what was given in `enqueue`,
	/// but you can do whatever makes sense in your application.
	fn queued(&self) -> Vector;

	/// Resolve the movement: you get a set of tiles and you decide what to do with them.
	/// If you aren't satisfied, you can change the move vector and return false, this means
	/// that we'll try again. Another set of tiles may then be given.
	/// If you're satisfied, return true and adjust your `Collable`'s position accordingly.
	///
	/// IMPORTANT: You should add the move from queued_move to your point set. The ray tracer
	/// also adds to find the next points. This will prevent you from getting stuck in a wall.
	fn resolve<I>(&mut self, set: TileSet<T, I>) -> bool where I: Iterator<Item = (i32, i32)>;

	/// Called at the beginning of `solve`
	///
	/// This method is useful when resetting internal variables of state.
	/// An example of this is when you have to set a has-jumped variable.
	fn presolve(&mut self) {}

	/// Called at the end of `solve`.
	///
	/// Used to process the result from the resolve loop.
	fn postsolve(&mut self, _collided_once: bool, _resolved: bool) {}

	/// Convenienve function for the resolve loop
	///
	/// Calls presolve at the beginning and postsolve at the end.
	/// Runs the resolve function in a loop of at max 30 iterations.
	/// This is to avoid potential deadlock if the resolve function
	/// is poorly coded and returns false all the time.
	fn solve(&mut self, net: &TileNet<T>) {
		self.presolve();
		static MAX_ITERATIONS: usize = 30;
		let mut collided_once = false;
		let mut resolved = false;
		for _ in 0..MAX_ITERATIONS {
			let tiles = net.collide_set(self.tiles());
			if self.resolve(tiles) {
				resolved = true;
				break;
			}
			collided_once = true;
		}
		self.postsolve(collided_once, resolved);
	}

	/// Gives us a list of points, sorted by proximity on the line.
	///
	/// The sortedness of the returned iterator means you can base your decision on the
	/// first element(s), as they represent the first collision.
	fn tiles(&self) -> MultiIter<(i32, i32)> {
		let points = self.points();
		let queued = self.queued();
		let mut multi = interleave!((i32, i32););
		for point in points {
			let current = Vector::from_tuple(point);
			let next = current + queued;
			let line = Line(current, next);
			multi.push(Box::new(line.supercover()));
		}
		multi
	}
}
