use std::collections::BTreeMap;

use raylib::{RaylibHandle, RaylibThread, drawing::RaylibDraw, prelude::*};



pub struct SpriteSheet {
	texture : raylib::core::texture::Texture2D,
	frames : [u16; 2],
}

pub struct Sprite {
	pub frame : usize,
	pub pos : [f32; 2],
	pub scale : [f32; 2],
}

pub struct SpriteBatch {
	queue : BTreeMap<u64, Sprite>,
	sheet : SpriteSheet,
}

pub struct Renderer {
	pub batches : Vec<SpriteBatch>,
}

impl Sprite {
	pub fn new(scale : [f32; 2]) -> Self {
		Self {
			frame: 0,
			pos: [0.0, 0.0],
			scale,
		}
	}
}

impl SpriteSheet {
	pub fn new(rl : &mut RaylibHandle, thread : & RaylibThread, path : &str, frames : [u16; 2]) -> Self {
		let texture = rl.load_texture(thread, path).unwrap_or_else(|err| panic!("Failed to load texture {}:\n\t{}", path, err));
		Self { texture, frames }
	}
}

impl SpriteBatch {
	pub fn new(sheet : SpriteSheet) -> Self {
		Self {
			queue: BTreeMap::new(),
			sheet,
		}
	}

	pub fn get_sprite(&mut self, uuid : u64) -> &mut Sprite {
		self.queue.get_mut(&uuid).unwrap_or_else(|| panic!("Sprite does not exist!"))
	}

	pub fn add_sprite(&mut self, uuid : u64, sprite : Sprite) {
		self.queue.insert(uuid, sprite);
	}

	pub fn render(&self, d : &mut raylib::drawing::RaylibDrawHandle) {
		let sw = (self.sheet.texture.width / self.sheet.frames[0] as i32) as f32;
		let cols = self.sheet.frames[0] as usize;
		let sh = (self.sheet.texture.height / self.sheet.frames[1] as i32) as f32;
		for (_, sprite) in self.queue.iter() {
			let col = (sprite.frame % cols) as f32;
			let row = (sprite.frame / cols) as f32;
			d.draw_texture_pro(&self.sheet.texture, Rectangle::new(
				col * sw, row * sh, sw, sh
			),
			Rectangle::new(
				sprite.pos[0], sprite.pos[1],
				sw * sprite.scale[0], sh * sprite.scale[1]
			), Vector2::new(0.0, 0.0), 0.0, Color::WHITE);
		}
	}
}


impl Renderer {
	pub fn new() -> Self {
		Self {
			batches: Vec::new(),
		}
	}

	pub fn get_batch(&mut self, index : usize) -> &mut SpriteBatch {
		&mut self.batches[index]
	}

	pub fn add_sheet(&mut self, sheet : SpriteSheet) -> usize {
		self.batches.push(SpriteBatch::new(sheet));
		self.batches.len() - 1
	}

	pub fn render(&mut self, rl : &mut RaylibHandle, thread : &RaylibThread) {
		let mut d = rl.begin_drawing(&thread);
		d.clear_background(raylib::color::Color::REBECCAPURPLE);

		for batch in self.batches.iter() {
			batch.render(&mut d);
		}
	}
}
