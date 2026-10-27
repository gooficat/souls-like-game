mod game;
mod error;
mod renderer;
mod world;
mod time;

fn main() {
	let mut game = game::Game::new([1280, 720], 24, "./res/map1.toml");
	game.run();
}
