mod world;
mod renderer;

use raylib::{RaylibHandle, RaylibThread, drawing::RaylibDraw};
use crate::{renderer::Renderer, world::World};

#[derive(PartialEq, Eq)]
enum GameState {
	Menu,
	Paused,
	Playing,
	Quitting,
}

struct Game {
	world : World,
	rl : RaylibHandle,
	thread : RaylibThread,
	state : GameState,
	renderer: Renderer,
	tick_speed : u8,
}

impl Game {
	pub fn new(size : [i32 ; 2], tick_speed : u8) -> Self {
		let (rl, thread) = raylib::init()
			.size(size[0], size[1])
			.title("Game")
			.resizable()
			.vsync()
			.build();

		Self {
			world: World::new(),
			rl,
			thread,
			state: GameState::Playing,
			renderer: Renderer::new(),
			tick_speed,
		}
	}

	pub fn tick(&mut self, dt : f32) {

	}

	pub fn listen(&mut self) {
		if self.rl.window_should_close() {
			self.state = GameState::Quitting;
		}
	}

	pub fn render(&mut self) {
		self.renderer.render(&mut self.rl, &mut self.thread);
	}

	pub fn run(&mut self) {
		while self.state != GameState::Quitting {
			let dt = self.rl.get_frame_time();
			self.listen();
			self.tick(self.tick_speed as f32 / dt);
			self.render();
		}
	}
}

fn main() {
	let mut game = Game::new([1280, 720], 24);
	game.run();
}
