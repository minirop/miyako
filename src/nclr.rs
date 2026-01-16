use byteorder::ReadBytesExt;
use std::io::Read;
use std::io::{self, Cursor};

use byteorder::LittleEndian;

pub struct Nclr {
    pub palettes: Vec<Vec<u32>>,
}

impl Nclr {
    pub fn new<R: Read>(reader: &mut R) -> io::Result<Self> {
        let _unknown1 = reader.read_u32::<LittleEndian>()?;
        let _size = reader.read_u32::<LittleEndian>()?;
        let _unknown2 = reader.read_u32::<LittleEndian>()?;

        let pltt = reader.read_u32::<LittleEndian>()?;
        assert_eq!(pltt, 0x504c5454);
        let pltt_size = reader.read_u32::<LittleEndian>()?;
        let bits = reader.read_u32::<LittleEndian>()?;
        let bits = 1 << (bits - 1);
        let external_palette = reader.read_u32::<LittleEndian>()? != 0;
        assert!(external_palette);
        let data_size = reader.read_u32::<LittleEndian>()?;
        let data_offset = reader.read_u32::<LittleEndian>()?;
        assert_eq!(data_offset, 16);
        assert_eq!(pltt_size, data_size + 0x18);

        let mut pixel_data = vec![0u8; data_size as usize];
        reader.read(&mut pixel_data)?;
        let pcmp = reader.read_u32::<LittleEndian>()?;
        assert_eq!(pcmp, 0x50434D50);
        let pcmp_size = reader.read_u32::<LittleEndian>()?;
        let nb_palettes = reader.read_u16::<LittleEndian>()?;
        let _beef = reader.read_u16::<LittleEndian>()?;
        assert_eq!(pcmp_size, nb_palettes as u32 * 2 + 16);

        let mut palettes_idx = vec![];
        for _ in 0..nb_palettes {
            palettes_idx.push(reader.read_u16::<LittleEndian>()?);
        }

        let colours_per_palette = 1 << bits;

        let mut palettes = vec![];
        let mut i = 0;
        for palette in pixel_data.chunks_exact(colours_per_palette * 2) {
            i += 1;
            let mut cursor = Cursor::new(palette);
            let mut colours = vec![];
            for _ in 0..colours_per_palette {
                let colour = cursor.read_u16::<LittleEndian>()?;
                let rgb = convert_5551_to_8888(colour);
                colours.push(rgb);
            }

            palettes.push(colours);
        }
        assert_eq!(i, nb_palettes);

        Ok(Self { palettes })
    }
}

fn convert_5551_to_8888(data: u16) -> u32 {
    let r = ((data & 0x1f) << 3) as u32;
    let g = (((data >> 5) & 0x1f) << 3) as u32;
    let b = (((data >> 10) & 0x1f) << 3) as u32;
    let a = ((data >> 15) * 255) as u32;
    r << 24 | g << 16 | b << 8 | a
}
