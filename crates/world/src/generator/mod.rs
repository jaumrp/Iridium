use crate::chunk::Chunk;

pub mod flat;
pub mod pregen;

pub trait WorldGenerator: Send + Sync {
    fn generate_chunk(&self, chunk_x: i32, chunk_z: i32) -> Chunk;
}
