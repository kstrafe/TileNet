#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]
extern crate sfml;
extern crate tile_net;

use sfml::graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable,
                     Drawable, RenderStates, View};
use sfml::window::{Key, VideoMode, event, window_style};
use sfml::system::Vector2f;
use tile_net::*;

static SIZE: f32 = 30.0;
static WINDOW: (u32, u32) = (800, 600);

fn main() {

	let mut window = create_window();
	let net = create_tilenet();
	let mut tile = create_block();
	let mut coller = Rects::new();
	let gravity = 0.00981;

	println!["Use WASD to move around"];

	'main: loop {
		if handle_events(&mut window) {
			break 'main;
		}

		let side_speed = 0.4;
		let vert_speed = 0.45;
		if Key::A.is_pressed() {
			coller.enqueue(Vector(-side_speed, 0.0));
		}
		if Key::D.is_pressed() {
			coller.enqueue(Vector(side_speed, 0.0));
		}

		let movement = coller.mov;
		coller.solve(&net, &mut ColState::check_x(movement));

		if Key::W.is_pressed() && coller.jmp {
			coller.set_speed(Vector(0.0, -vert_speed));
			coller.jmp = false;
		}
		if Key::S.is_pressed() {
			coller.enqueue(Vector(0.0, vert_speed * 100000.0));
		}

		coller.enqueue(Vector(0.0, gravity));
		let movement = coller.mov;
		coller.solve(&net, &mut ColState::no_check_x(movement));

		window.clear(&Color::new_rgb(255, 255, 255));
		let mut view = View::new_init(&Vector2f::new(0.0, 0.0),
		                              &Vector2f::new(WINDOW.0 as f32, WINDOW.1 as f32))
		                              .unwrap();
		let pos = coller.get_pos();
		view.set_center(&Vector2f::new(pos.0 * SIZE, pos.1 * SIZE));
		window.set_view(&view);

		for i in net.view_center_f32((pos.0, pos.1), (120usize, 60usize)) {
			if let (&1, col, row) = i {
				let col = col as f32;
				let row = row as f32;
				tile.set_position(&Vector2f::new(col * SIZE, row * SIZE));
				window.draw(&tile);
			}
		}
		window.draw(&coller);
		window.display();
	}
}

fn create_window() -> RenderWindow {
	let mut window = RenderWindow::new(VideoMode::new_init(WINDOW.0, WINDOW.1, 42),
	                                   "Custom shape",
	                                   window_style::CLOSE,
	                                   &Default::default())
		.unwrap_or_else(|| {
			panic!("Could not create window");
		});
	window.set_framerate_limit(60);
	window
}

fn create_tilenet() -> tile_net::TileNet<usize> {
	let mut net: TileNet<usize> = tile_net::TileNet::new(10, 10);
	net.set_box(&0, (0, 0), (10, 10));
	net.set_box(&1, (1, 1), (9, 9));
	net.set_box(&0, (2, 2), (8, 8));
	net.set_box(&1, (4, 4), (6, 6));
	net
}

fn create_block<'a>() -> RectangleShape<'a> {
	let mut block = RectangleShape::new().unwrap();
	block.set_size(&Vector2f::new(SIZE, SIZE));
	block.set_fill_color(&Color::new_rgb(0, 0, 0));
	block
}

fn handle_events(window: &mut RenderWindow) -> bool {
	for event in window.events() {
		match event {
			event::Closed => return true,
			event::KeyPressed { code, .. } => {
				if let Key::Escape = code {
					return true;
				}
			}
			_ => {}
		}
	}
	false
}

#[derive(Debug)]
struct Rects {
	pts: Vec<(f32, f32)>,
	pos: Vector,
	pub mov: Vector,
	jmp: bool,
	checking_x: bool,
	downward: bool,
}

impl Rects {
	fn new() -> Rects {
		Rects {
			pts: vec![(0.0, 0.0), (0.99, 0.0), (0.0, 0.99), (0.99, 0.99)],
			pos: Vector(2.0, 2.0),
			mov: Vector(0.0, 0.0),
			jmp: false,
			checking_x: false,
			downward: false,
		}
	}

	fn enqueue(&mut self, vector: Vector) {
		self.mov = self.mov + vector;
	}

	fn set_speed(&mut self, vec: Vector) {
		self.mov = vec;
	}

	fn get_pos(&self) -> Vector {
		self.pos
	}

}

struct ColState {
	checking_x: Option<f32>,
	downward: bool,
	mov: Vector,
}

impl Default for ColState {
	fn default() -> ColState {
		ColState {
			checking_x: None,
			downward: false,
			mov: Vector(0.0, 0.0),
		}
	}
}

impl ColState {
	fn check_x(mov: Vector) -> ColState {
		ColState {
			checking_x: Some(0.0),
			downward: false,
			mov: mov,
		}
	}

	fn no_check_x(mov: Vector) -> ColState {
		ColState {
			checking_x: None,
			downward: false,
			mov: mov,
		}
	}
}

impl CollableState for ColState {
	fn queued(&self) -> Vector {
		self.mov
	}
}

impl Collable<usize, ColState> for Rects {
	fn presolve(&mut self, state: &mut ColState) {
		if state.checking_x.is_some() {
			state.checking_x = Some(self.mov.1);
			state.mov = Vector(self.mov.0, 0.0);
			self.mov = Vector(self.mov.0, 0.0);
		} else {
			state.downward = self.mov.1 > 1e-6;
		}
	}

	fn postsolve(&mut self, collided_once: bool, _resolved: bool, state: &mut ColState) {
		if let Some(dy) = state.checking_x {
			state.mov = Vector(self.mov.0, dy);
			self.mov = Vector(self.mov.0, dy);
		} else {
			if collided_once && state.downward {
				self.jmp = true;
			} else {
				self.jmp = false;
			}
		}
	}

	fn points(&self) -> Points {
		Points::new(self.pos, &self.pts)
	}

	fn resolve<I>(&mut self, mut set: TileSet<usize, I>, state: &mut ColState) -> bool
		where I: Iterator<Item = (i32, i32)>
	{
		let mut mov = state.mov;
		state.mov = Vector(0.0, 0.0);
		self.mov = Vector(0.0, 0.0);
		if set.all(|x| *x == 0usize) {
			self.pos = self.pos + mov;
			state.mov = Vector(0.0, mov.1);
			self.mov = Vector(0.0, mov.1);
			true
		} else if mov.norm2sq() > 1e-6 {
			if state.checking_x.is_some() {
				mov = Vector(mov.0 * 0.59, mov.1);
				state.mov = mov;
				self.mov = mov;
			} else {
				mov.scale(0.6);
				state.mov = mov;
				self.mov = mov;
			}
			false
		} else {
			true
		}
	}
}

impl Drawable for Rects {
	fn draw<R: RenderTarget>(&self, rt: &mut R, _: &mut RenderStates) {
		let mut block = create_block();
		block.set_position(&Vector2f::new(self.pos.0 * SIZE, self.pos.1 * SIZE));
		rt.draw(&block);
	}
}
