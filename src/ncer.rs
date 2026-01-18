use crate::common::read_header;
use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use std::io;
use std::io::Read;
use std::io::Seek;

pub struct NcerCellAttr {
    pub y: u16,
    pub rotate_scale: bool,
    pub double_size: bool,
    pub disabled: bool,
    pub mode: u8,
    pub mosaic: bool,
    pub is_8: bool,
    pub shape: u8,
    pub x: u16,
    pub matrix: u8,
    pub flip_x: bool,
    pub flip_y: bool,
    pub size: u8,
    pub character_name: u16,
    pub priority: u8,
    pub palette: u8,
}

pub struct NcerCell {
    pub attributes: u16,
    pub oam: Vec<NcerCellAttr>,
}

pub struct Ncer {
    pub cells: Vec<NcerCell>,
    pub labels: Vec<String>,
}

impl Ncer {
    pub fn new<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let header = read_header(reader)?;
        assert_eq!(header.magic, 0x4e434552);

        let cebk = reader.read_u32::<LittleEndian>()?;
        assert_eq!(cebk, 0x4345424b);
        let _size = reader.read_u32::<LittleEndian>()?;
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
        let mut oams_entries = vec![];
        for _ in 0..nb_cells {
            let oam_entries = reader.read_u16::<LittleEndian>()?;
            oams_entries.push(oam_entries);

            let attributes = reader.read_u16::<LittleEndian>()?;
            let _oam_offset = reader.read_u32::<LittleEndian>()?;

            cells.push(NcerCell {
                attributes,
                oam: vec![],
            });
        }

        for (cell, entries) in cells.iter_mut().zip(oams_entries) {
            for _ in 0..entries {
                let attr0 = reader.read_u16::<LittleEndian>()?;
                let attr1 = reader.read_u16::<LittleEndian>()?;
                let attr2 = reader.read_u16::<LittleEndian>()?;

                let y = attr0 & 0xFF;
                let rotate_scale = (attr0 & 0x0100) != 0;
                let double_or_disabled = (attr0 & 0x0200) != 0;
                let double_size = rotate_scale && double_or_disabled;
                let disabled = !rotate_scale && double_or_disabled;
                let mode = ((attr0 >> 10) & 0b11) as u8;
                let mosaic = (attr0 & 0x1000) != 0;
                let is_8 = (attr0 & 0x2000) != 0;
                let shape = (attr0 >> 14) as u8;
                let x = attr1 & 0x1FF;
                let matrix = if rotate_scale {
                    ((attr0 >> 9) & 0b11111) as u8
                } else {
                    0
                };
                let flip_x = !rotate_scale && (attr0 & 0x1000) != 0;
                let flip_y = !rotate_scale && (attr0 & 0x2000) != 0;
                let size = (attr1 >> 14) as u8;
                let character_name = attr2 & 0x3FF;
                let priority = ((attr2 >> 10) & 0b11) as u8;
                let palette = ((attr2 >> 12) & 0x0F) as u8;

                cell.oam.push(NcerCellAttr {
                    y,
                    rotate_scale,
                    double_size,
                    disabled,
                    mode,
                    mosaic,
                    is_8,
                    shape,
                    x,
                    matrix,
                    flip_x,
                    flip_y,
                    size,
                    character_name,
                    priority,
                    palette,
                });
            }
        }

        let labels = if let Ok(labl) = reader.read_u32::<LittleEndian>() {
            assert_eq!(labl, 0x4c41424c);
            let size = reader.read_u32::<LittleEndian>()? as usize;
            let _unk1 = reader.read_u32::<LittleEndian>()?;
            let _unk2 = reader.read_u32::<LittleEndian>()?;
            let mut labels = vec![0u8; size - 16];
            reader.read(&mut labels)?;

            let labels = labels
                .split(|x| *x == 0)
                .map(|v| str::from_utf8(v).unwrap().to_string())
                .filter(|s| s.len() > 0)
                .collect::<Vec<_>>();
            labels
        } else {
            // check if all images have those
            todo!();
        };

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

        Ok(Self { cells, labels })
    }
}
