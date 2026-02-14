use std::sync::Arc;

use dashmap::DashMap;

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
    pub dimensions: DashMap<DimensionId, Arc<Dimension>>,
}

pub struct Dimension {
    pub min_y: i32,
    pub height: i32,
    pub chunks: DashMap<(i32, i32), Chunk>,
    pub generator: Box<dyn WorldGenerator>,
}

impl World {
    pub fn new() -> Self {
        World {
            dimensions: DashMap::new(),
        }
    }

    pub fn add_dimension(&mut self, id: DimensionId, dimension: Dimension) {
        self.dimensions.insert(id, Arc::new(dimension));
    }

    pub fn get_dimension(&self, id: DimensionId) -> Option<Arc<Dimension>> {
        self.dimensions.get(&id).map(|r| r.value().clone())
    }
}

impl Dimension {
    pub fn new(id: DimensionId, generator: Box<dyn WorldGenerator>) -> Self {
        let (min_y, height) = match id {
            DimensionId::Overworld => (-64, 320),
            DimensionId::Nether => (0, 128),
            DimensionId::TheEnd => (0, 256),
        };
        Dimension {
            min_y,
            height,
            chunks: DashMap::new(),
            generator,
        }
    }
}
