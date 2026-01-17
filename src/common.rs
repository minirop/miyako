use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use std::io;
use std::io::Read;
use std::io::Seek;

pub fn decompress<R: Read + Seek>(reader: &mut R) -> io::Result<Vec<u8>> {
    let encryption = reader.read_u8()?;
    let buffer = if encryption == 0x11 {
        let uncompressed_size = reader.read_u24::<LittleEndian>()? as usize;

        let mut output = vec![];
        'outer: loop {
            let head = reader.read_u8()?;

            for i in (0..8).rev() {
                let flag = (head >> i) & 1;

                if flag == 0 {
                    output.push(reader.read_u8()?);

                    if output.len() == uncompressed_size {
                        break 'outer;
                    }
                } else {
                    let high = reader.read_u8()?;
                    let low = reader.read_u8()? as usize;

                    let mode = high >> 4;

                    let (len, offs) = match mode {
                        0 => {
                            let low2 = reader.read_u8()? as usize;
                            let len = ((high as usize) << 4) + (low >> 4) + 0x11;
                            let offs = ((low & 0xF) << 8) + low2 + 1;
                            (len, offs)
                        }
                        1 => {
                            let low2 = reader.read_u8()? as usize;
                            let low3 = reader.read_u8()? as usize;

                            let len =
                                ((high as usize & 0xF) << 12) + (low << 4) + (low2 >> 4) + 0x111;
                            let offs = ((low2 & 0xF) << 8) + low3 + 1;
                            (len, offs)
                        }
                        _ => {
                            let len = (mode + 1) as usize;
                            let offs = ((high as usize & 0xF) << 8) + low + 1;
                            (len, offs)
                        }
                    };

                    let starter = output.len() - offs;
                    for i in 0..len {
                        output.push(output[starter + i]);
                        if output.len() == uncompressed_size {
                            break 'outer;
                        }
                    }
                }
            }
        }

        assert!(reader.read_u8().is_err());

        output
    } else {
        reader.seek(io::SeekFrom::Start(0))?;
        let mut buffer = vec![];
        reader.read_to_end(&mut buffer)?;
        buffer
    };

    Ok(buffer)
}
