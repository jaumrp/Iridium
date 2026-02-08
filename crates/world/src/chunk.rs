#[derive(Debug, Clone)]
pub struct Chunk {
    pub x: i32,
    pub z: i32,
    pub sections: Vec<ChunkSection>,
}

#[derive(Debug, Clone)]
pub struct ChunkSection {
    pub blocks_count: u16,
    pub blocks: Box<[u16; 4096]>,
    pub biomes: Option<Box<[u8; 64]>>,
}

impl ChunkSection {
    pub fn new() -> Self {
        ChunkSection {
            blocks_count: 0,
            blocks: Box::new([0; 4096]),
            biomes: None,
        }
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block_id: u16) {
        let index = (y * 16 + z) * 16 + x;
        let old_id = self.blocks[index];

        if old_id == 0 && block_id != 0 {
            self.blocks_count += 1;
        } else if old_id != 0 && block_id == 0 {
            self.blocks_count -= 1;
        }
        self.blocks[index] = block_id;
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> u16 {
        self.blocks[(y * 16 + z) * 16 + x]
    }
}

impl Chunk {
    pub fn new(x: i32, z: i32) -> Self {
        Chunk {
            x,
            z,
            sections: vec![ChunkSection::new(); 24],
        }
    }

    pub fn fill_layer(&mut self, y: i32, block_id: u16) {
        if block_id == 0 || y < -64 || y > 320 {
            return;
        }

        let idx = ((y + 64) / 16) as usize;
        let local_y = ((y + 64) % 16) as usize;

        let section = &mut self.sections[idx];
        section.blocks_count += 256;

        for x in 0..16 {
            for z in 0..16 {
                let index = (local_y * 16 + z) * 16 + x;
                section.blocks[index] = block_id;
            }
        }
    }
}
