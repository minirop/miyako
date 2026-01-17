use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use std::io;
use std::io::Read;
use std::io::Seek;

pub struct Ncgr {
    pub tiles: Vec<Vec<u8>>,
}

impl Ncgr {
    pub fn new<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let magic = reader.read_u32::<LittleEndian>()?;
        assert_eq!(magic, 0x4e434752);

        let _unknown1 = reader.read_u32::<LittleEndian>()?;
        let _size = reader.read_u32::<LittleEndian>()?;
        let _unknown2 = reader.read_u32::<LittleEndian>()?;

        let char_ = reader.read_u32::<LittleEndian>()?;
        assert_eq!(char_, 0x43484152);
        let _size = reader.read_u32::<LittleEndian>()?;

        let mut tiles_y = reader.read_u16::<LittleEndian>()?;
        let mut tiles_x = reader.read_u16::<LittleEndian>()?;

        let depth = reader.read_u32::<LittleEndian>()?;
        let depth = 1 << (depth - 1);
        assert_eq!(depth, 8);

        let _mapping = reader.read_u32::<LittleEndian>()?;

        let kind = reader.read_u32::<LittleEndian>()?;
        assert_eq!(kind, 0);

        let tile_data_size = reader.read_u32::<LittleEndian>()?;
        let gfx_offset = reader.read_u32::<LittleEndian>()?;
        assert_eq!(gfx_offset, 24);

        let mut tiles_count = tiles_x as u32 * tiles_y as u32;
        let present_tiles = tile_data_size >> 6;
        if tiles_count != present_tiles {
            tiles_count = present_tiles;
            assert_eq!(tiles_count % 32, 0);
            tiles_x = 32;
            tiles_y = tiles_count as u16 / tiles_x;
        }

        let mut tiles = vec![];
        for _ in 0..tiles_count {
            let mut tile = vec![0u8; 64];
            reader.read(&mut tile)?;

            tiles.push(tile);
        }

        if let Ok(cpos) = reader.read_u32::<LittleEndian>() {
            assert_eq!(cpos, 0x43504f53);
            let size = reader.read_u32::<LittleEndian>()?;
            assert_eq!(size, 0x10);
            let unk = reader.read_u32::<LittleEndian>()?;
            assert_eq!(unk, 0);
            let tw = reader.read_u16::<LittleEndian>()?;
            assert_eq!(tw, tiles_x);
            let th = reader.read_u16::<LittleEndian>()?;
            assert_eq!(th, tiles_y);
        }

        assert!(reader.read_u8().is_err());

        Ok(Self { tiles })
    }
}
