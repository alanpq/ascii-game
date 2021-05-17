use std::collections::HashMap;
use rand;

pub const CHUNK_SIZE: usize = 50;
pub const CHUNK_SIZE_I32: i32 = CHUNK_SIZE as i32;

#[derive(Copy, Clone)]
pub enum Tile {
    Air,
    Wall
}

pub struct Chunk {
    pub tiles: [[Tile; CHUNK_SIZE]; CHUNK_SIZE],
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            tiles: [[Tile::Air; CHUNK_SIZE]; CHUNK_SIZE],
        }
    }

    pub fn generate(&mut self) {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                if rand::random() {
                    self.tiles[x][y] = Tile::Wall;
                }
            }
        }
    }
}

pub struct Map {
    chunks: HashMap<(i32, i32), Chunk>,
}

impl Map {
    pub fn new() -> Map {
        Map {
            chunks: HashMap::new(),
        }
    }

    pub fn get_chunk(&self, x: i32, y: i32) -> Option<&Chunk> {
        self.chunks.get(&(x,y))
    }

    pub fn get_chunk_or_create(&mut self, x: i32, y: i32) -> &Chunk {
        self.chunks.entry((x,y)).or_insert_with(|| {
            let mut chunk = Chunk::new();
            chunk.generate();
            chunk
        })
    }

    pub fn get_chunk_from_point(&self, x: i32, y: i32) -> Option<&Chunk> {
        self.get_chunk(x / CHUNK_SIZE as i32, y / CHUNK_SIZE as i32)
    }
}