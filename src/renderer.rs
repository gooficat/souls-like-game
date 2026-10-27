use std::{collections::HashMap};

use raylib::{RaylibHandle, RaylibThread, drawing::RaylibDraw};



struct SpriteSheet {
	texture : raylib::texture::Texture2D,
	frames : [i16; 2],
}

struct Sprite {
	sheet : usize,
}

struct SpriteBatch {
	queue : HashMap<u64, Sprite>,
	sheet : SpriteSheet,
}

pub struct Renderer {
	batches : Vec<SpriteBatch>,
}

impl Renderer {
	pub fn new() -> Self {
		Self {
			batches: Vec::new(),
		}
	}

	pub fn add_sheet(&mut self, sheet : SpriteSheet) {

	}

	pub fn render(&mut self, rl : &mut RaylibHandle, thread : &mut RaylibThread) {
		let mut d = rl.begin_drawing(&thread);
		d.clear_background(raylib::color::Color::REBECCAPURPLE);
	}
}
