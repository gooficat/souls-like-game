use crate::game::Game;


pub struct Timer {
	duration : f32,
	elapsed : f32,
	callback: Box<dyn FnMut(&mut Game)>,
}

impl Timer {
	pub fn new(duration : f32, callback : Box<dyn FnMut(&mut Game)>) -> Self {
		Self {
			duration,
			elapsed : 0.0,
			callback,
		}
	}

	pub fn tick(&mut self, game: &mut Game, delta : f32) {
		self.elapsed += delta;
		if self.elapsed >= self.duration {
			(self.callback)(game);
			self.elapsed = 0.0;
		}
	}
}
