use ahash::AHashMap;

use crate::{chunk::Chunk, generator::WorldGenerator};

pub mod chunk;
pub mod generator;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DimensionId {
    Overworld,
    Nether,
    TheEnd,
}

pub struct World {
    pub dimensions: AHashMap<DimensionId, Dimension>,
}

pub struct Dimension {
    pub min_y: i32,
    pub height: i32,
    pub chunks: AHashMap<(i32, i32), Chunk>,
    pub generator: Box<dyn WorldGenerator>,
}

impl Dimension {
    pub fn new(id: DimensionId, generator: Box<dyn WorldGenerator>) -> Self {
        let (min_y, height) = match id {
            DimensionId::Overworld => (-64, 384),
            DimensionId::Nether => (0, 128),
            DimensionId::TheEnd => (0, 256),
        };
        Dimension {
            min_y,
            height,
            chunks: AHashMap::new(),
            generator,
        }
    }

    pub fn get_or_generate_chunk(&mut self, x: i32, z: i32) -> &mut Chunk {
        self.chunks
            .entry((x, z))
            .or_insert_with(|| self.generator.generate_chunk(x, z))
    }
}
