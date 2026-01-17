use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use std::io;
use std::io::Read;
use std::io::Seek;

pub struct NcerCell {
    pub oam_entries: u16,
    pub cell_attrs: u16,
    pub attrs: Vec<u8>,
}

pub struct Ncer {
    pub cells: Vec<NcerCell>,
}

impl Ncer {
    pub fn new<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let magic = reader.read_u32::<LittleEndian>()?;
        assert_eq!(magic, 0x4e434552);

        let _unknown1 = reader.read_u32::<LittleEndian>()?;
        let _size = reader.read_u32::<LittleEndian>()?;
        let _unknown2 = reader.read_u32::<LittleEndian>()?;

        let cebk = reader.read_u32::<LittleEndian>()?;
        assert_eq!(cebk, 0x4345424b);
        let cebk_size = reader.read_u32::<LittleEndian>()?;
        let nb_cells = reader.read_u16::<LittleEndian>()?;
        let bank_attrib = reader.read_u16::<LittleEndian>()?;
        assert_eq!(bank_attrib, 0);

        let cell_data = reader.read_u32::<LittleEndian>()?;
        assert_eq!(cell_data, 24);
        let mapping_mode = reader.read_u32::<LittleEndian>()?;
        assert_eq!(mapping_mode, 2);
        let vram_transfer_offset = reader.read_u32::<LittleEndian>()?;
        assert_eq!(vram_transfer_offset, 0);
        let unk = reader.read_u32::<LittleEndian>()?;
        assert_eq!(unk, 0);
        let user_extended_offset = reader.read_u32::<LittleEndian>()?;
        assert_eq!(user_extended_offset, 0);

        let mut cells = vec![];
        let mut cells_attr_offsets = vec![];
        for _ in 0..nb_cells {
            let oam_entries = reader.read_u16::<LittleEndian>()?;
            let cell_attrs = reader.read_u16::<LittleEndian>()?;
            let oam_attrs = reader.read_u32::<LittleEndian>()?;
            cells_attr_offsets.push(oam_attrs);

            cells.push(NcerCell {
                oam_entries,
                cell_attrs,
                attrs: vec![],
            });
        }
        cells_attr_offsets.push(cebk_size - 32 - nb_cells as u32 * 8);

        for (id, win) in cells_attr_offsets.windows(2).enumerate() {
            let diff = win[1] - win[0];
            let mut buffer = vec![0u8; diff as usize];
            reader.read(&mut buffer)?;
            cells[id].attrs = buffer;
        }

        if let Ok(labl) = reader.read_u32::<LittleEndian>() {
            assert_eq!(labl, 0x4c41424c);
            let size = reader.read_u32::<LittleEndian>()? as usize;
            let mut labels = vec![0u8; size - 8];
            reader.read(&mut labels)?;
            // todo
        } else {
            // check if all images have those
            todo!();
        }

        if let Ok(uext) = reader.read_u32::<LittleEndian>() {
            assert_eq!(uext, 0x55455854);
            let size = reader.read_u32::<LittleEndian>()?;
            assert_eq!(size, 0x0c);
            let unk = reader.read_u32::<LittleEndian>()?;
            assert_eq!(unk, 0);
        } else {
            // check if all images have those
            todo!();
        }

        assert!(reader.read_u8().is_err());

        Ok(Self { cells })
    }
}
