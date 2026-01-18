use crate::common::read_header;
use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use std::io;
use std::io::Read;
use std::io::Seek;

pub struct NscrTile {
    pub id: u16,
    pub palette: u8,
}

pub struct Nscr {
    pub width: u32,
    pub height: u32,
    pub map: Vec<NscrTile>,
}

impl Nscr {
    pub fn new<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let header = read_header(reader)?;
        assert_eq!(header.magic, 0x4E534352);

        let scrn = reader.read_u32::<LittleEndian>()?;
        assert_eq!(scrn, 0x5343524E);
        let _size = reader.read_u32::<LittleEndian>()?;

        let width = reader.read_u16::<LittleEndian>()? as u32 / 8;
        let height = reader.read_u16::<LittleEndian>()? as u32 / 8;
        let _color_mode = reader.read_u16::<LittleEndian>()?;
        let format = reader.read_u16::<LittleEndian>()?;
        assert_eq!(format, 0);
        let _data_size = reader.read_u32::<LittleEndian>()?;

        let nb_tiles = width * height;

        let mut map = vec![];
        for _ in 0..nb_tiles {
            let data = reader.read_u16::<LittleEndian>()?;
            let id = data & 0xFFF;
            let palette = (data >> 12) as u8;

            map.push(NscrTile { id, palette });
        }

        assert!(reader.read_u8().is_err());

        Ok(Self { width, height, map })
    }
}
