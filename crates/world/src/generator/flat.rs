use crate::{chunk::Chunk, generator::WorldGenerator};

pub struct FlatGenerator {
    layers: Vec<Layer>,
}

struct Layer {
    y: i32,
    block_id: u16,
}

impl FlatGenerator {
    pub fn new() -> Self {
        FlatGenerator {
            layers: vec![
                Layer { y: 0, block_id: 33 },
                Layer { y: 1, block_id: 10 },
                Layer { y: 2, block_id: 10 },
                Layer { y: 3, block_id: 10 },
                Layer { y: 4, block_id: 9 },
            ],
        }
    }
}

impl WorldGenerator for FlatGenerator {
    fn generate_chunk(&self, chunk_x: i32, chunk_z: i32) -> Chunk {
        let mut chunk = Chunk::new(chunk_x, chunk_z);

        for layer in &self.layers {
            chunk.fill_layer(layer.y, layer.block_id);
        }

        chunk
    }
}
