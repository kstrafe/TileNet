pub use super::Point;

/// Describe a rectangle by its top-left corner, width, and height
pub struct Rect {
	pub top_left: Point,
	pub width: f32,
	pub height: f32,
}

impl Rect {
	pub fn new(width: f32, height: f32) -> Rect {
		Rect {
			top_left: Point(0.0, 0.0),
			width: width,
			height: height,
		}
	}

	pub fn set_place(&mut self, top_left: Point) {
		self.top_left = top_left;
	}
}

#[cfg(test)]
mod tests {
	use super::{Point, Rect};

	#[test]
	fn test() {
		Rect::new(2.0, 1.0);
	}
}
