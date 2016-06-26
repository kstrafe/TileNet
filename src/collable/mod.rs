pub use super::{LineTiles, Line, Vector, TileSet};
use std::fmt;

/// Trait for dynamic objects so they can easily check collisions with the `TileMap`
pub trait Collable {
	/// Returns the set of points associated with this object. These points are used to
	/// draw lines to their respective next points. For a rectangle, the four courners
	/// may be points. For a circle, a whole bunch of points may be defined.
	fn points(&self) -> Vector;

	/// Instructs the object to store (queue) a change in position. This may be useful when
	/// you have an event loop and you'd like to move a character. You call this function.
	/// An AI handler may also queue a move. It's up to you if you want to add moves
	/// together or store them in any other way.
	fn queue(&mut self, vector: Vector);

	/// Get the previously queued move. Should reasonably return what was given in `queue_move`,
	/// but you can do whatever makes sense in your application.
	fn queued(&self) -> Vector;

	/// Resolve the movement: you get a set of tiles and you decide what to do with them.
	/// If you aren't satisfied, you can change the move vector and return false, this means
	/// that we'll try again. Another set of tiles may then be given.
	/// If you're satisfied, return true and adjust your `Collable`'s position accordingly.
	///
	/// IMPORTANT: You should add the move from queued_move to your point set. The ray tracer
	/// also adds to find the next points. This will prevent you from getting stuck in a wall.
	fn resolve<'a, T, I>(&mut self, set: TileSet<'a, T, I>) -> bool
		where T: 'a + Clone + fmt::Debug,
		      I: Iterator<Item = (i32, i32)>;

	/// Gives us a list of points, sorted by proximity on the line.
	///
	/// The sortedness of the returned iterator means you can base your decision on the
	/// first element(s), as they represent the first collision.
	fn tiles(&self) {}
}

/// Represents the tiles touched by various lines
struct LinesTiles {
	// NOTE: Lines are assumed to be of equal length, so it's probably true that there are equal
	// amounts of tiles in each LineTiles iterator
	lines: Vec<LineTiles>,
	index: usize,
}

impl Iterator for LinesTiles {
	type Item = (i32, i32);
	fn next(&mut self) -> Option<Self::Item> {
		let clean = true;
		loop {
			if self.index < self.lines.len() {
				if let Some(line) = self.lines.get_mut(self.index) {
					self.index += 1;
					return line.next();
				}
				self.index += 1;
			} else {
				if clean {
					return None;
				} else {
					self.index = 0;
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test() {
		Line::from_origin(Vector(0.5, 0.0)).supercover().inspect(|x| println!("{:?}", x)).count();
		println!("HELLO");
		Line(Vector(0.5, 0.0), Vector(1.0, 0.0)).supercover().inspect(|x| println!("{:?}", x)).count();
		Line(Vector(-5.0, 0.0), Vector(-4.0, 0.0)).supercover().inspect(|x| println!("{:?}", x)).count();
		Line(Vector(-5.0, 0.0), Vector(4.0, 0.0)).supercover().inspect(|x| println!("{:?}", x)).count();
		Line(Vector(5.0, -3.0), Vector(3.99, 4.0)).supercover().inspect(|x| println!("{:?}", x)).count();
	}
}
