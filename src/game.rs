
use raylib::{RaylibHandle, RaylibThread};
use crate::{renderer::{Renderer, Sprite, SpriteSheet}, world::World};

#[derive(PartialEq, Eq)]
enum GameState {
	Menu,
	Paused,
	Playing,
	Quitting,
}

pub struct Game {
	world : World,
	rl : RaylibHandle,
	thread : RaylibThread,
	state : GameState,
	renderer: Renderer,
	tick_speed : u8,
	keys : [bool ; 512],
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
			keys: [false; 512],
		}
	}

	pub fn init(&mut self) {
		self.renderer.add_sheet(SpriteSheet::new(&mut self.rl, &self.thread, "./res/tilemap.png", [1, 1]));
		self.renderer.get_batch(0).add_sprite(1, Sprite::new([128.0, 128.0]));
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
		self.init();
		while self.state != GameState::Quitting {
			let dt = self.rl.get_frame_time();
			self.listen();
			self.tick(self.tick_speed as f32 / dt);
			self.render();
		}
	}
}
