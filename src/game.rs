
	use std::{collections::BTreeMap, mem::take};

	use raylib::{RaylibHandle, RaylibThread};
	use crate::{renderer::{Renderer, Sprite, SpriteSheet}, time::{self, Timer}, world::World};

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
				timers: BTreeMap::new(),
			}
		}

		pub fn add_timer(&mut self, uuid : u64, timer : time::Timer) {
			self.timers.insert(uuid, timer);
		}

		pub fn init(&mut self) {
			self.renderer.add_sheet(SpriteSheet::new(&mut self.rl, &self.thread, "./res/tilemap_packed.png", [12,11]));
			self.renderer.get_batch(0).add_sprite(1, Sprite::new([10.0, 10.0]));


			self.add_timer(1, Timer::new(0.5,
				Box::new(|game: &mut Game| {
				game.renderer.get_batch(0).get_sprite(1).frame += 1;
			})));
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
				self.tick(self.tick_speed as f32 / dt);
				self.render();
			}
		}
	}
