use std::collections::HashMap;
use nanoserde::{DeBin, SerBin};

use rand::{SeedableRng, RngCore};
use rand::rngs::{SmallRng};
use std::rc::{Rc};
use std::cell::{RefCell};

pub const CHUNK_SIZE: usize = 32;
pub const CHUNK_SIZE_I32: i32 = CHUNK_SIZE as i32;

#[derive(Copy, Clone)]
#[derive(PartialEq, DeBin, SerBin)]
pub enum Tile {
    Air,
    Wall
}

#[derive(PartialEq, DeBin)]
pub struct Chunk {
    pub tiles: [[Tile; CHUNK_SIZE]; CHUNK_SIZE],
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            tiles: [[Tile::Air; CHUNK_SIZE]; CHUNK_SIZE],
        }
    }

    pub fn generate(&mut self, rng: Rc<RefCell<SmallRng>>) {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                if rng.borrow_mut().next_u32() < 2147483647 {
                    self.tiles[x][y] = Tile::Wall;
                }
            }
        }
    }
}

pub struct Map {
    pub seed: u64,
    chunks: HashMap<(i32, i32), Chunk>,
    rng: Rc<RefCell<SmallRng>>,
}

impl Map {
    pub fn new(seed: u64) -> Map {
        Map {
            seed,
            chunks: HashMap::new(),
            rng: Rc::new(RefCell::new(SeedableRng::seed_from_u64(seed))),
        }
    }

    pub fn default() -> Map {
        let seed = rand::random();
        Self::new(seed)
    }

    pub fn get_chunk(&self, x: i32, y: i32) -> Option<&Chunk> {
        self.chunks.get(&(x,y))
    }

    pub fn get_chunk_or_create(&mut self, x: i32, y: i32) -> &Chunk {
        let rng = self.rng.clone();
        self.chunks.entry((x,y)).or_insert_with(|| {
            let mut chunk = Chunk::new();
            chunk.generate(rng);
            chunk
        })
    }

    pub fn get_chunk_from_point(&self, x: i32, y: i32) -> Option<&Chunk> {
        self.get_chunk(x / CHUNK_SIZE as i32, y / CHUNK_SIZE as i32)
    }
}
