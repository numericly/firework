pub mod lighting_engine {

    use world::world::region::chunk::ChunkSection;

    pub fn calculate_section_lighting(sections: [Option<ChunkSection>; 27]) -> [[u8; 2048]; 2] {
        //first array is the block light level, second array is the sky light level
        let mut lighting = [0; 2048];
        let mut accesses = [0u16; 27];

        //loop through the blocks and make sure the chunk sectioning works
        for x in 0..48 {
            for y in 0..48 {
                for z in 0..48 {
                    //println!("accessed section {}, at section position ({}, {}, {}) at block position ({}, {}, {})", (x / 16 + y / 16 * 3 + z / 16 * 9), x / 16, y / 16, z / 16, x, y, z);
                    accesses[x / 16 + y / 16 * 3 + z / 16 * 9] += 1;
                }
            }
        }
        println!("accesses: {:?}", accesses);
        [lighting, [0; 2048]]
    }


    fn source_brightness(x: u8, y: u8, z: u8, section: ChunkSection) -> u8 {
        //TODO: check wiki for how to get block data from x y z
        // block_data[(x + (z * 16) + (y * 256)) as usize]
        0u8
    }
}