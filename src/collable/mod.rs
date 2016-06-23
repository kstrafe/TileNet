use super::{Line, Vector, TileSet};
use std::fmt;

trait Collable {
	fn points(&self);
	fn queue_move(&mut self);
	fn queued_move(&self) -> Vector;
	fn resolve_move<'a, T, I>(&mut self, set: TileSet<'a, T, I>) -> bool
		where T: 'a + Clone + fmt::Debug,
		      I: Iterator<Item = (i32, i32)>;

	fn lines(&self) -> Line {
		Line(Vector(0.0, 0.0), Vector(1.0, 2.0))
	}
}
