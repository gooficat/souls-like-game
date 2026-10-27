use std::{any::Any, collections::BTreeMap, fs};

use raylib::{RaylibHandle, RaylibThread, drawing::RaylibDrawHandle, texture::Texture2D};
use serde::Deserialize;

use crate::renderer::{Renderer, Sprite, SpriteBatch, SpriteSheet};

struct Weapon {
	weight : i16,
	damage : i16,
}

struct Consumable {

}

struct Wearable {

}

enum Item {
	Weapon(Weapon),
	Consumable(Consumable),
	Wearable(Wearable),
}

struct Inventory {
	items : Vec<Item>,
	main_hand : usize,
	off_hand : usize,
	body : usize,
}

struct Entity {
	position : [f32 ; 2],
	health : i32,
	stamina : i32,
	inventory : Inventory,
}

struct Map {
	tiles : Vec<Vec<u8>>,
}

pub struct World {
	map : Map,
	entities : BTreeMap<u64, Entity>,// Main player is always entity 0
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorldRes {
	pub name : String,
	pub map_sheet : String,
	pub sheet_size : [u16; 2],
	#[serde(default)]
	pub entity_types : Vec<String>,
	#[serde(default)]
	pub tiles : Vec<Vec<u8>>,
}

impl WorldRes {
	fn validate(&self) -> Result<(), String> {
		Ok(())
	}
}

impl World {
	pub fn load(path : &str, rl : &mut RaylibHandle, thread : &mut RaylibThread, renderer : &mut Renderer) -> Self {
		let text = fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to read level file `{}`!", path));
		let res : WorldRes = toml::from_str(&text).unwrap_or_else(|_| panic!("Failed to parse toml for level file `{}`!", path));
		res.validate().unwrap_or_else(|_| panic!("Toml for level file `{}` is invalid!", path));

		let idx = renderer.add_sheet(SpriteSheet::new(rl, thread, res.map_sheet.as_str(), res.sheet_size));
		let batch = renderer.get_batch(idx);

		let world = World {
			map: Map {tiles: res.tiles},
			entities: BTreeMap::new(),
		};

		let mut x : usize = 0;
		let mut y : usize = 0;
		let mut i : u64 = 0;

		let tile_w = batch.sheet.texture.width as f32 / res.sheet_size[0] as f32;
		let tile_h = batch.sheet.texture.height as f32 / res.sheet_size[1] as f32;

		for row in world.map.tiles.iter() {
			for col in row.iter() {
				i += 1;
				let mut sprite = Sprite::new([1.0, 1.0]);

				sprite.frame = *col as usize;

				sprite.pos = [
					x as f32 * tile_w,
					y as f32 * tile_h,
				];

				batch.add_sprite(i, sprite);
				x += 1;
			}
			x = 0;
			y += 1;
		}


		world
	}

	pub fn update(&mut self, delta : f32) {

	}

	pub fn render(&mut self, d : &mut RaylibDrawHandle, renderer : &mut Renderer) {

	}
}
