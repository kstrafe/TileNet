pub use super::Point;

/// Describe a rectangle by its top-left corner, width, and height
pub struct Rect {
	pub top_left: Point,
	pub width: f32,
	pub height: f32,
}

impl Rect {
	pub fn new(top_left: Point, width: f32, height: f32) -> Rect {
		Rect {
			top_left: top_left,
			width: width,
			height: height,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{Point, Rect};

	#[test]
	fn test() {
		let r = Rect::new(Point(1.0, 2.0), 1.0, 1.0);
	}
}
