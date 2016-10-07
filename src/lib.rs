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
//! # Examples #
//! We start out by including tile net into our program and creating an empty net
//!
//! ```
//! extern crate tile_net;
//! use tile_net::*;
//! fn main() {
//!   let net: TileNet<usize> = TileNet::new((10, 10));
//! }
//!
//! ```
//! This creates a `TileNet` that contains `usize` as its elements.
//! All tiles are initialized to None.
//! You can now edit various tiles:
//! ```
//! extern crate tile_net;
//! use tile_net::*;
//! fn main() {
//!   let mut net: TileNet<usize> = TileNet::new((10, 10));
//!   *net.get_mut((0, 0)) = Some(0);
//! }
//!
//! ```
//!
//!

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
