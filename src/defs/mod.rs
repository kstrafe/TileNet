pub mod line;

pub use self::line::Line;

/// Describe a point in 2-space
///
/// Use two floats to denote the x and y coordinates
/// respectively in the tuple.
/// ```
/// use tile_net::Point;
/// let point = Point(0.5, 1.0);
/// assert_eq!(point.0, 0.5);
/// assert_eq!(point.1, 1.0);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point(pub f32, pub f32);

/// Index for a tile in a 2D map
///
/// An index represents a location in a 2D integer
/// bounded map. Only positive integers are allowed because
/// the ideal map in this library is positive.
///
/// ```
/// use tile_net::Index;
/// let index = Index(10, 50);
/// assert_eq!(index.0, 10);
/// assert_eq!(index.1, 50);
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Index(pub usize, pub usize);

/// Descriptor denoting the quadrant
/// of a point or line.
///
/// The quadrant of a line is taken
/// with the starting point as origo.
/// The ending point's quadrant can
/// then be deduced.
///
/// The quadrant of a point is taken
/// with respect to origo (0, 0).
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
/// Note that origo is considered part of the first quadrant.
/// The reasoning behind this is that the algorithms relying on
/// it have no specific discriminatory use against an origo point.
/// Going with the first quadrant is the most natural result then.
///
/// We can confirm these boundaries by using a `Line`:
///
/// ```
/// use tile_net::{Line, Point, Quadrant};
/// let line = Line::from_origo(Point(1.0, 0.0));
/// assert_eq!(line.quadrant(), Quadrant::One);
/// let line = Line::from_origo(Point(0.0, 1.0));
/// assert_eq!(line.quadrant(), Quadrant::Two);
/// let line = Line::from_origo(Point(-1.0, 0.0));
/// assert_eq!(line.quadrant(), Quadrant::Three);
/// let line = Line::from_origo(Point(0.0, -1.0));
/// assert_eq!(line.quadrant(), Quadrant::Four);
/// let line = Line::from_origo(Point(0.0, 0.0));
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
