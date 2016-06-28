pub use super::{LineTiles, Line, Vector, TileSet};
use std::fmt;

pub use interleave::{IterList, MultiIter};

struct Points<'a> {
	index : usize,
	offset: Vector,
	points: &'a Vec<(f32, f32)>,
}

impl<'a> Points<'a> {
	fn new(offset: Vector, points: &'a Vec<(f32, f32)>) -> Points {
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
		let ret = self.points.get(self.index).map(|x| *x);
		self.index += 1;
		ret
	}
}

/// Trait for dynamic objects so they can easily check collisions with the `TileMap`
pub trait Collable {
	/// Returns the set of points associated with this object. These points are used to
	/// draw lines to their respective next points. For a rectangle, the four courners
	/// may be points. For a circle, a whole bunch of points may be defined.
	fn points<'a>(&'a self) -> Points<'a>;

	/// Instructs the object to store (queue) a change in position. This may be useful when
	/// you have an event loop and you'd like to move a character. You call this function.
	/// An AI handler may also queue a move. It's up to you if you want to add moves
	/// together or store them in any other way.
	fn enqueue(&mut self, vector: Vector);

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
	fn resolve<'a, T, I>(&mut self, set: TileSet<'a, T, I>) -> bool
		where T: 'a,
		      I: Iterator<Item = (i32, i32)>;

	/// Gives us a list of points, sorted by proximity on the line.
	///
	/// The sortedness of the returned iterator means you can base your decision on the
	/// first element(s), as they represent the first collision.
	fn tiles(&self) -> MultiIter<(i32, i32)> {
		let points = self.points();
		let queued = self.queued();
		let mut multi = interleave!((i32, i32));
		for point in points {
			let current = Vector::from_tuple(point);
			let next = current + queued;
			let line = Line(current, next);
			multi.push(Box::new(line.supercover()));
		}
		multi
	}
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
					let maybe = line.next();
					if let Some(_) = maybe {
						return maybe;
					} else {

					}
				}
				self.index += 1;
			} else if clean {
				return None;
			} else {
				self.index = 0;
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use super::Points;

	#[test]
	fn test() {
		struct Character {
			pts: Vec<(f32, f32)>,
		}

		impl Character {
			fn new() -> Character {
				Character {
					pts: vec![(0.0, 0.0), (1.0, 1.0), (0.0, 1.0), (1.0, 0.0)],
				}
			}
		}

		impl Collable for Character {
			fn points<'a>(&'a self) -> Points<'a> {
				let off = Vector(0.0, 0.0);
				Points::new(off, &self.pts)
			}

			fn enqueue(&mut self, vector: Vector) { }

			fn queued(&self) -> Vector {
				Vector(2.0, 2.0)
			}

			fn resolve<'a, T, I>(&mut self, set: TileSet<'a, T, I>) -> bool
				where T: 'a,
				      I: Iterator<Item = (i32, i32)>
			{
				unimplemented!();
			}
		}


		// Line::from_origin(Vector(0.5, 0.0)).supercover().inspect(|x| println!("{:?}", x)).count();
		// println!("HELLO");
		// Line(Vector(0.5, 0.0), Vector(1.0, 0.0))
		// .supercover()
		// .inspect(|x| println!("{:?}", x))
		// .count();
		// Line(Vector(-5.0, 0.0), Vector(-4.0, 0.0))
		// .supercover()
		// .inspect(|x| println!("{:?}", x))
		// .count();
		// Line(Vector(-5.0, 0.0), Vector(4.0, 0.0))
		// .supercover()
		// .inspect(|x| println!("{:?}", x))
		// .count();
		// Line(Vector(5.0, -3.0), Vector(3.99, 4.0))
		// .supercover()
		// .inspect(|x| println!("{:?}", x))
		// .count();
		//
	}
}
