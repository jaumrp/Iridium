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
        if y < -64 || y > 320 {
            return;
        }

        let idx = ((y + 64) / 16) as usize;
        let local_y = ((y + 64) % 16) as usize;

        let section = &mut self.sections[idx];

        let start_index = local_y * 256;
        let end_idnex = start_index + 256;
        let slice = &mut section.blocks[start_index..end_idnex];

        let existing_blocks = slice.iter().filter(|&&id| id != 0).count() as u16;
        section.blocks_count -= existing_blocks;
        slice.fill(block_id);
        if block_id != 0 {
            section.blocks_count += 256;
        }
    }

    pub fn fill_section(&mut self, section_index: usize, block_id: u16) {
        if let Some(section) = self.sections.get_mut(section_index) {
            section.blocks.fill(block_id);
            if block_id == 0 {
                section.blocks_count = 0;
            } else {
                section.blocks_count = 4096;
            }
        }
    }
}
