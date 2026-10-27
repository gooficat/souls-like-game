
	use std::{collections::BTreeMap, mem::take};

	use raylib::{RaylibHandle, RaylibThread};
	use crate::{renderer::{self, Renderer, Sprite, SpriteSheet}, time::{self, Timer}, world::World};

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
		timers : BTreeMap<u64, time::Timer>
	}

	impl Game {
		pub fn new(size : [i32 ; 2], tick_speed : u8, path : &str) -> Self {
			let (mut rl, mut thread) = raylib::init()
				.size(size[0], size[1])
				.title("Game")
				.resizable()
				.vsync()
				.build();

			rl.set_target_fps(60);
			let mut renderer =  Renderer::new();

			Self {
				world: World::load(path, &mut rl, &mut thread, &mut renderer),
				rl,
				thread,
				state: GameState::Playing,
				tick_speed,
				renderer,
				timers: BTreeMap::new(),
			}
		}

		pub fn add_timer(&mut self, uuid : u64, timer : time::Timer) {
			self.timers.insert(uuid, timer);
		}

		pub fn init(&mut self) {

		}

		pub fn tick(&mut self, dt : f32) {
			let mut timers = take(&mut self.timers);

			for timer in timers.values_mut() {
				timer.tick(self, dt);
			}

			timers.append(&mut self.timers);
			self.timers = timers;
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
				self.tick(self.tick_speed as f32 * dt);
				self.render();
			}
		}
	}
