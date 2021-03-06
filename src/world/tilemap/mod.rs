use sfml::graphics::{Image, Color};

use crate::vec::TileVec;

pub const TILEMAP_SIZE: TileVec = TileVec::new(32, 32);

static TILES: [Tile; 3] = [Tile::GROUND, Tile::WALL, Tile::SPIKE];

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
	GROUND,
	WALL,
	SPIKE
}

impl Tile {
	pub fn get_color(&self) -> Color {
		match self {
			Tile::GROUND => Color::rgb(100, 100, 0),
			Tile::WALL => Color::rgb(0, 0, 0),
			Tile::SPIKE => Color::rgb(255, 255, 255),
		}
	}
}

pub struct TileMap {
	tiles: [[Tile; TILEMAP_SIZE.y as usize]; TILEMAP_SIZE.x as usize],
}

impl TileMap {
	pub fn new() -> TileMap {
        let tiles = [[Tile::GROUND; TILEMAP_SIZE.y as usize]; TILEMAP_SIZE.x as usize];
		TileMap { tiles }
	}

	pub fn from_filename(filename: &str) -> TileMap {
		let img = Image::from_file(filename).unwrap();
		assert!(img.size().x == TILEMAP_SIZE.x as u32);
		assert!(img.size().y == TILEMAP_SIZE.y as u32);

        let mut tilemap = TileMap::new();
		
		for p in TileVec::iter_all() {
			let c = img.pixel_at(p.x as u32, p.y as u32);
			let t = TILES.iter().find(|t| t.get_color() == c).unwrap();
			tilemap.set_tile(p, *t);
		}
		tilemap
	}

	pub fn set_tile(&mut self, pos: TileVec, t: Tile) {
		self.tiles[pos.x as usize][pos.y as usize] = t;
	}

	pub fn get_tile(&self, pos: TileVec) -> Tile {
		self.tiles[pos.x as usize][pos.y as usize]
	}
}
