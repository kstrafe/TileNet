pub mod line;
pub mod vector;
pub mod rectangle;

pub use self::line::{Line, LineTiles};
pub use self::vector::Vector;
pub use self::rectangle::Rect;

/// Descriptor denoting the quadrant
/// of a `Vector` or `Line`.
///
/// The quadrant of a line is taken
/// with the starting point as origin.
/// The ending point's quadrant can
/// then be deduced.
///
/// The quadrant of a point is taken
/// with respect to origin (0, 0).
///
/// Quadrant points lying on a zero
/// will include their start-angle
/// quadrant. This means the following:
/// ( or ) is an excluding boundary.
/// [ or ] is an inclusive boundary.
///
/// The following ranges describe each
/// quadrant.
///
/// ```ignore
/// One = ((0, inf), [0, inf)) + [0, 0]
/// Two = ((-inf, 0], (0, inf))
/// Three = ((-inf, 0), (-inf, 0])
/// Four = ([0, inf), (0, -inf))
/// ```
///
/// Note that origin is considered part of the first quadrant.
/// The reasoning behind this is that the algorithms relying on
/// it have no specific discriminatory use against an origin point.
/// Going with the first quadrant is the most natural result then.
///
/// We can confirm these boundaries by using a `Line`:
///
/// ```
/// use tile_net::{Line, Vector, Quadrant};
/// let line = Line::from_origin(Vector(1.0, 0.0));
/// assert_eq!(line.quadrant(), Quadrant::One);
/// let line = Line::from_origin(Vector(0.0, 1.0));
/// assert_eq!(line.quadrant(), Quadrant::Two);
/// let line = Line::from_origin(Vector(-1.0, 0.0));
/// assert_eq!(line.quadrant(), Quadrant::Three);
/// let line = Line::from_origin(Vector(0.0, -1.0));
/// assert_eq!(line.quadrant(), Quadrant::Four);
/// let line = Line::from_origin(Vector(0.0, 0.0));
/// assert_eq!(line.quadrant(), Quadrant::One);
/// ```
/// ![Plot](../../../res/quadrant.png)
#[derive(Debug, Eq, PartialEq)]
pub enum Quadrant {
	One,
	Two,
	Three,
	Four,
}
