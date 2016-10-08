#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]
//! `TileNet` holds integer aligned tiles for broad phase continuous collision detection.
//! The purpose of `TileNet` is to have a solid, tile-based, continuous, simple collision
//! library for aspiring game programmers.
//!
//! # How it works #
//! The library is built on the DDA Supercover algorithm, which is an extension of
//! Bresenham's algorithm. For each point moving, it creates a line. Each line's
//! overlapping tiles are reported. Your dynamic object decides how it should move.
//! It may adjust speed, and retry the collision. It may also accept and move.
//!
//! # Limitations #
//! The library will experience problems with huge coordinates. This is because adding
//! a small increment to a floating point above 2^24 may not register at all. Precision
//! becomes worse as you approach 2^24. The technical reason is that a 32-bit float
//! has 24 bits in its mantissa.
//!
//! # Examples - Setting Up #
//! We start out by including tile net into our program and creating an empty net
//!
//! ```
//! extern crate tile_net;
//! use tile_net::*;
//! fn main() {
//!   let net: TileNet<usize> = TileNet::new((10, 10));
//!   println!["{:?}", net];
//! }
//!
//! ```
//! This creates a `TileNet` that contains `usize` as its elements.
//! All tiles are initialized to None.
//! You can now edit various tiles:
//!
//! ```
//! extern crate tile_net;
//! use tile_net::*;
//! fn main() {
//!   let mut net: TileNet<usize> = TileNet::new((10, 10));
//!   net.set(&1, (9, 0));
//!   println!["{:?}", net];
//! }
//!
//! ```
//!
//! There are several helper functions so you can easily draw something interesting
//!
//! ```
//! extern crate tile_net;
//! use tile_net::*;
//! fn main() {
//!   let mut net: TileNet<usize> = TileNet::new((10, 10));
//!   net.set_row(&1, 0);
//!   net.set_row(&1, 9);
//!   net.set_col(&1, 0);
//!   net.set_col(&1, 9);
//!   net.set_box(&1, (3, 3), (5, 7));
//!   println!["{:?}", net];
//! }
//!
//! ```
//!
//! You can use any element in `TileNet` as long as it has the following traits:
//!
//! ```
//! extern crate tile_net;
//! use tile_net::*;
//! #[derive(Clone, Debug, Default)]
//! struct Example(i32);
//! fn main() {
//!   let mut net: TileNet<Example> = TileNet::new((10, 10));  // Requires Default trait
//!   net.set_row(&Example(1), 0);  // Requires Clone trait
//!   net.set_row(&Example(2), 9);
//!   net.set_col(&Example(3), 0);
//!   net.set_col(&Example(4), 9);
//!   net.set_box(&Example(5), (3, 3), (5, 7));
//!   println!["{:?}", net];  // Requires Debug trait
//! }
//! ```
//!
//! # Collision Detection #
//! `TileNet` is not used for drawing tiles to a grid, its main focus is continuous, tile-vertex
//! collision detection.
//! Continuous collision detection (CCD) prevents objects tunneling through other objects in a
//! frame. This happens
//! when we only check the beginning and end points of an object's movement. This library
//! interpolates on each
//! tile. So every intermediate tile is checked. Let's see an example.
//!
//! ```
//! extern crate tile_net;
//! use tile_net::*;
//!
//! fn main() {
//!   let mut net: TileNet<usize> = TileNet::new((10, 10));
//!   net.set_row(&1, 0);
//!   net.set_row(&2, 9);
//!   net.set_col(&3, 0);
//!   net.set_col(&4, 9);
//!   net.set_box(&5, (3, 3), (5, 7));
//!   println!["{:?}", net];
//!
//!   // We create a new object with speed (100, 100) and check where our collision points will be!
//!   let mut collider = MyObject::new();
//!   let supercover = collider.tiles();  // This is the supercover of the current movement
//!   // in the grid, it just returns integer points of every tile that collider touches
//!   let tiles = net.collide_set(supercover);
//!   if collider.resolve(tiles) {
//!     println!["Able to move"];
//!   } else {
//!     println!["Unable to move"];
//!   }
//! }
//!
//! #[derive(Debug)]
//! struct MyObject {
//!   pts: Vec<(f32, f32)>,
//!   pos: Vector,
//!   mov: Vector,
//! }
//!
//! impl MyObject {
//!   fn new() -> MyObject {
//!     MyObject {
//!       pts: vec![(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (1.0, 1.0)],
//!       pos: Vector(1.1, 1.1),
//!       mov: Vector(100.0, 100.0),
//!     }
//!   }
//!
//! }
//!
//! impl Collable<usize> for MyObject {
//!   // This function returns the vertices of the object
//!   // The points are used by the collision engine to create a set of
//!   // lines from the beginning to the end of the frame.
//!   fn points<'a>(&'a self) -> Points<'a> {
//!     Points::new(self.pos, &self.pts)
//!   }
//!
//!   // The physics engine uses this function in conjunction with points to compute
//!   // the lines - and thus - tiles it will iterate over during a collision test.
//!   fn queued(&self) -> Vector {
//!     self.mov
//!   }
//!
//!   // Here is where your magic happens!
//!   // You will be given a TileSet, which contains all tiles which your object
//!   // collides between the current frame jump.
//!   // The tiles given are in nearest-order, so the first tiles you get from the
//!   // iterator are always the ones you will collide with first.
//!   fn resolve<'a, I>(&mut self, mut set: TileSet<'a, usize, I>) -> bool
//!     where I: Iterator<Item = (i32, i32)>
//!   {
//!     if set.all(|x| *x == 0) {  // If there is no collision (we only collide with non-zero tiles)
//!       self.pos = self.pos + self.mov;
//!       self.mov = Vector(0.0, 0.0);
//!       true
//!     } else if self.mov.norm2sq() > 1e-6 {  // There was collision, but our speed isn't tiny
//!       self.mov.scale(0.9);
//!       false
//!     } else {  // This may happen if we generate a world where we're stuck in a tile,
//!               // normally this will never happen, this library can preserve consistently
//!               // perfectly.
//!       true
//!     }
//!   }
//! }
//! ```
//!
//! What you can do with `resolve` is to run it in a loop. After scaling down the movement vector
//! sufficiently in `resolve`, you may end up with a `TileSet` that does not cause collision.
//! This is how we can almost perfectly find the position.
//! You may employ other methods inside resolve. Whatever suits your needs.
//! Here is the example again but this time we resolve the collision using a loop
//!
//! ```
//! extern crate tile_net;
//! use tile_net::*;
//!
//! fn main() {
//!   let mut net: TileNet<usize> = TileNet::new((10, 10));
//!   net.set_row(&1, 0);
//!   net.set_row(&2, 9);
//!   net.set_col(&3, 0);
//!   net.set_col(&4, 9);
//!   net.set_box(&5, (3, 3), (5, 7));
//!   println!["{:?}", net];
//!
//!   // Movement vector is (100, 100), which is way outside the box
//!   let mut collider = MyObject::new();
//!   loop {
//!     let supercover = collider.tiles();
//!     let tiles = net.collide_set(supercover);
//!     if collider.resolve(tiles) {
//!       println!["Able to move"];
//!       break;
//!     } else {
//!       println!["Unable to move"];
//!     }
//!   }
//!   // We are interested in the final position!
//!   println!["{:?}", collider];
//! }
//!
//! #[derive(Debug)]
//! struct MyObject {
//!   pts: Vec<(f32, f32)>,
//!   pos: Vector,
//!   mov: Vector,
//! }
//!
//! impl MyObject {
//!   fn new() -> MyObject {
//!     MyObject {
//!       pts: vec![(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (1.0, 1.0)],
//!       pos: Vector(1.1, 1.1),
//!       mov: Vector(100.0, 100.0),
//!     }
//!   }
//!
//! }
//!
//! impl Collable<usize> for MyObject {
//!   // This function returns the vertices of the object
//!   // The points are used by the collision engine to create a set of
//!   // lines from the beginning to the end of the frame.
//!   fn points<'a>(&'a self) -> Points<'a> {
//!     Points::new(self.pos, &self.pts)
//!   }
//!
//!   // The physics engine uses this function in conjunction with points to compute
//!   // the lines - and thus - tiles it will iterate over during a collision test.
//!   fn queued(&self) -> Vector {
//!     self.mov
//!   }
//!
//!   // Here is where your magic happens!
//!   // You will be given a TileSet, which contains all tiles which your object
//!   // collides between the current frame jump.
//!   // The tiles given are in nearest-order, so the first tiles you get from the
//!   // iterator are always the ones you will collide with first.
//!   fn resolve<'a, I>(&mut self, mut set: TileSet<'a, usize, I>) -> bool
//!     where I: Iterator<Item = (i32, i32)>
//!   {
//!     if set.all(|x| *x == 0) {  // If there is no collision (we only collide with non-zero tiles)
//!       self.pos = self.pos + self.mov;
//!       self.mov = Vector(0.0, 0.0);
//!       true  // Means we resolved correctly
//!     } else if self.mov.norm2sq() > 1e-6 {  // There was collision, but our speed isn't tiny
//!       self.mov.scale(0.9);
//!       false  // Means we did not resolve collision
//!     } else {
//!       true
//!     }
//!   }
//! }
//! ```
//!
//! You can try to use more nuanced methods instead of scaling down and checking again. One method
//! may be to check the//! first collision point and scale down to the distance thereof.
//! Everything is iterator based.
//!
//! # TileView #
//! For drawing you may want to avoid sending huge grids to the GPU, so we use a view from the grid.
//!
//! ```
//! extern crate tile_net;
//! use tile_net::*;
//! fn main() {
//!   let mut net: TileNet<usize> = TileNet::new((10, 10));
//!   net.set_row(&1, 0);
//!   net.set_row(&2, 9);
//!   net.set_col(&3, 0);
//!   net.set_col(&4, 9);
//!   net.set_box(&5, (3, 3), (5, 7));
//!   println!["{:?}", net];
//!   // This creates a box with x from 0 to 4 and y from 3 to 6
//!   // Note that the last elements are not included (so for x: 0, 1, 2, 3, but not 4)
//!   for element in net.view_box((0, 4, 3, 6)) {
//!     let (value, col, row) = element;
//!     // Draw here!
//!     println!["{}-{} = {}", row, col, value];
//!   }
//!   // This just prints every single element in the net
//!   for element in net.view_all() {
//!     let (value, col, row) = element;
//!     // Draw here!
//!     println!["{}-{} = {}", row, col, value];
//!   }
//!   // Looks from (0, 1) to (6, 5). This takes care of negative indices that may be created.
//!   // The first argument represents the center. The second argument is the span around that center.
//!   for element in net.view_center((3, 3), (4, 2)) {
//!     let (value, col, row) = element;
//!     // Draw here!
//!     println!["{}-{} = {}", row, col, value];
//!   }
//! }
//! ```



#[macro_use(interleave)]
extern crate interleave;

mod collable;
mod defs;
mod tiles;

pub use defs::{SuperCover, Line, Vector};
pub use collable::{Collable, Points};
pub use tiles::{TileNet, TileView, TileSet};

#[cfg(test)]
mod tests {
	use super::TileNet;

	#[test]
	fn get() {
		let map: TileNet<usize> = TileNet::new((10, 10));
		assert_eq!(Some(&0), map.get((9, 9)));
		assert_eq!(None, map.get((10, 9)));
		assert_eq!(None, map.get((9, 10)));
	}

	#[test]
	fn get_mut() {
		let mut map: TileNet<usize> = TileNet::new((10, 10));
		*map.get_mut((9, 9)).unwrap() = 10;
		assert_eq!(Some(&10), map.get((9, 9)));
		*map.get_mut((9, 9)).unwrap() = 1;
		assert_eq!(Some(&1), map.get((9, 9)));
	}

	#[test]
	fn get_size() {
		let map: TileNet<usize> = TileNet::new((10, 10));
		assert_eq!((10, 10), map.get_size());
		let map: TileNet<usize> = TileNet::new((194, 483));
		assert_eq!((194, 483), map.get_size());
	}

	#[test]
	fn resize() {
		let mut map: TileNet<usize> = TileNet::new((10, 10));
		*map.get_mut((0, 0)).unwrap() = 0;
		map.resize((5, 5));
		map.resize((10, 10));
	}

	#[test]
	fn from_iter_and_view_box() {
		let map: TileNet<usize> = TileNet::from_iter(10, (1..101));
		let mut view = map.view_box((3, 8, 1, 4));
		(14usize..19)
			.chain((24..29))
			.chain((34..39))
			.map(|x| assert_eq!(view.next().unwrap().0, &x))
			.count();
	}

	#[test]
	fn from_iter_with_remainder() {
		let map: TileNet<usize> = TileNet::from_iter(10, (1..25));
		let mut view = map.view_box((0, 10, 0, 3));
		for x in (1..31).map(|x| if x >= 25 { 0 } else { x }) {
			assert_eq!(view.next().unwrap().0, &x);
		}

		let map: TileNet<usize> = TileNet::from_iter(10, (1..31));
		let mut view = map.view_box((0, 10, 0, 3));
		for x in 1..31 {
			assert_eq!(view.next().unwrap().0, &x);
		}
	}

	#[test]
	fn collide_set() {
		let map: TileNet<usize> = TileNet::from_iter(10,
		                                             (1..101).map(|x| if x > 50 { x } else { 0 }));
		let mut set = map.collide_set((3..7).map(|x| (4, x)));
		for _ in 1..3 {
			assert_eq!(set.next().unwrap(), &0);
		}
		for x in 0..2 {
			assert_eq!(set.next().unwrap(), &(55 + 10 * x));
		}
	}

}
