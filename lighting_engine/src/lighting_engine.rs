pub mod lighting_engine {

    use world::world::region::chunk::ChunkSection;

    struct QueuedLightingUpdate {
        x: u8,
        y: u16,
        z: u8,
        light_level: u8,
        is_sky_light: bool,
    }

    pub fn lighting_update_in_section(
        section_index: u16,
        sections: &mut [[Option<ChunkSection>; 26]; 25],
    ) {
        //recalculates lighting in the 3x3x3 cube around the section updated

        let mut queue = vec![];

        //loop through the blocks and add block light calculations to the queue
        for x in 0..80 {
            for y in 16 * section_index - 80..16 * section_index {
                for z in 0..80 {
                    //if there's a light source at these coordinates, add it to the queue
                    //the zero check takes slight CPU but saves a lot of memory and honestly probably more CPU than it uses by removing the need to push a value to the queue
                    let lvl = get_source_brightness(
                        x % 16,
                        (y % 16) as u8,
                        z % 16,
                        &sections[(x / 16 + z / 16 * 5) as usize][(y / 16) as usize],
                    );
                    if lvl != 0 {
                        queue.push(QueuedLightingUpdate {
                            x,
                            y,
                            z,
                            light_level: lvl,
                            is_sky_light: false,
                        });
                    }
                }
            }
        }

        //go through sky light calculations
        for x in 0..80 {
            for z in 0..80 {
                for y in (0..320).rev() {
                    //loop through 319 to 0
                    if is_transparent(
                        x % 16,
                        (y % 16) as u8,
                        z % 16,
                        &sections[(x / 16 + z / 16 * 5) as usize][(y / 16) as usize],
                    ) {
                        queue.push(QueuedLightingUpdate {
                            x,
                            y: y + 1, //FIXME: could be an issue for the top block at y=319
                            z,
                            light_level: 15,
                            is_sky_light: true,
                        });
                        break;
                    }
                }
            }
        }

        //possible optimization could be to sort the queue by light level, so that the highest light levels are calculated first
        //loop through the queue and calculate lighting
        for update in queue {
            //calculate lighting for the block
            propagate_lighting(
                update.x,
                update.y,
                update.z,
                update.light_level,
                sections,
                update.is_sky_light,
            );
        }
    }

    fn propagate_lighting(
        x: u8,
        y: u16,
        z: u8,
        lvl: u8,
        sections: &mut [[Option<ChunkSection>; 26]; 25],
        is_sky_light: bool,
    ) {
        if get_source_brightness(x % 16, (y%16) as u8, z%16, &sections[(x / 16  + z / 16 * 5) as usize][(y / 16) as usize]) == 0            //if the block is not a light source
        || lvl == 0                                                                                                                                         //or if the light level is 0   
        || get_brightness(x%16, (y%16) as u8, z%16, &sections[(x / 16  + z / 16 * 5) as usize][(y / 16) as usize], is_sky_light) >= lvl     //or if the block is already brighter than the source
        || x>47                                                                                                                                             //or if the block is out of bounds
        || y>47
        || z>47
        || is_opaque(x%16, (y%16) as u8, z%16, &sections[(x / 16 + z / 16 * 5) as usize][(y / 16) as usize])
        //or if the block is opaque
        {
            return;
        } //then do nothing

        //set light level of current block
        set_brightness(
            x % 16,
            (y & 16) as u8,
            z % 16,
            sections[(x / 16 + z / 16 * 5) as usize][(y / 16) as usize]
                .as_mut()
                .unwrap(),
            lvl,
            is_sky_light,
        );

        //propagate to neighboring blocks
        propagate_lighting(x + 1, y, z, lvl - 1, sections, is_sky_light);
        propagate_lighting(x - 1, y, z, lvl - 1, sections, is_sky_light);
        propagate_lighting(x, y + 1, z, lvl - 1, sections, is_sky_light);
        propagate_lighting(x, y - 1, z, lvl - 1, sections, is_sky_light);
        propagate_lighting(x, y, z + 1, lvl - 1, sections, is_sky_light);
        propagate_lighting(x, y, z - 1, lvl - 1, sections, is_sky_light);
    }

    fn is_transparent(x: u8, y: u8, z: u8, section: &Option<ChunkSection>) -> bool {
        //x y z are coordinates within the section
        if section.is_none() || section.as_ref().unwrap().block_states.data.is_none() {
            println!("<ERROR> in lighting_engine.is_transparent, chunk or chunk data was none: section = {:?}", section);
            return false;
        }
        //TODO: read opacity from block states

        false
    }

    fn is_opaque(x: u8, y: u8, z: u8, section: &Option<ChunkSection>) -> bool {
        //x y z are coordinates within the section
        if section.is_none() || section.as_ref().unwrap().block_states.data.is_none() {
            println!("<ERROR> in lighting_engine.is_opaque, chunk or chunk data was none: section = {:?}", section);
            return true;
        }
        //TODO: read opacity from block states

        //lava is included here strangely enough
        false
    }

    fn get_brightness(
        x: u8,
        y: u8,
        z: u8,
        section: &Option<ChunkSection>,
        is_sky_light: bool,
    ) -> u8 {
        if section.is_none() {
            return 0;
        }
        let lighting_data: &[i8; 2048]; //TODO: maybe make this use if let if that's relevant
        if is_sky_light {
            if section.as_ref().unwrap().sky_light.is_none() {
                return 0;
            }
            lighting_data = section.as_ref().unwrap().sky_light.as_ref().unwrap();
        } else {
            if section.as_ref().unwrap().block_light.is_none() {
                return 0;
            }
            lighting_data = section.as_ref().unwrap().block_light.as_ref().unwrap();
        }
        if x % 2 == 0 {
            //uses the first four bits, so shift the bits to the right by 4
            lighting_data[((x + y * 16 + z * 16 * 16) / 2) as usize] as u8 >> 4
        } else {
            //uses the second four bits, so mask the bits using 0b00001111
            lighting_data[((x + y * 16 + z * 16 * 16) / 2) as usize] as u8 & 0b00001111
        }
    }

    fn set_brightness(
        x: u8,
        y: u8,
        z: u8,
        section: &mut ChunkSection,
        lvl: u8,
        is_sky_light: bool,
    ) {
        let lighting_data: &mut [i8; 2048]; //TODO: maybe make this use if let if that's relevant
        if is_sky_light {
            lighting_data = section.sky_light.as_mut().unwrap();
        } else {
            lighting_data = section.block_light.as_mut().unwrap();
        }
        if x % 2 == 0 {
            //uses the first four bits
            lighting_data[((x + y * 16 + z * 16 * 16) / 2) as usize] =
                (lighting_data[((x + y * 16 + z * 16 * 16) / 2) as usize] as u8 & 0b11110000 | lvl)
                    as i8;
        //FIXME: does this copy the whole chunk section?
        } else {
            //uses the second four bits
            lighting_data[((x + y * 16 + z * 16 * 16) / 2) as usize] =
                (lighting_data[((x + y * 16 + z * 16 * 16) / 2) as usize] as u8 & 0b00001111
                    | (lvl << 4)) as i8;
        } //FIXME: does this copy the whole chunk section?
    }

    fn get_source_brightness(x: u8, y: u8, z: u8, section: &Option<ChunkSection>) -> u8 {
        //TODO: read brightness from block states
        if section.is_none() {
            //for unloaded chunk sections, return 0
            return 0;
        }
        0u8
    }
}
