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

/// Describe a line by its start and end `Point` respectively
///
/// A line can be constructed and used with tuples
///
/// ```
/// use tile_net::{Line, Point};
/// let start = Point(0.5, 1.0);
/// let finish = Point(1.2, -1.0);
/// let line = Line(start, finish);
/// assert_eq!(line.0, start);
/// assert_eq!(line.1, finish);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line(pub Point, pub Point);


