
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

}

impl Map {
	pub fn new() -> Self {
		Self {

		}
	}
}

pub struct World {
	map : Map,
	entities : Vec<Entity>,// Main player is always entity 0
}

impl World {
	pub fn new() -> Self {
		Self {
			map : Map::new(),
			entities : Vec::new(),
		}
	}
}
