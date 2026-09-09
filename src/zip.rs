use std::io::{self, Write};

pub(crate) fn write_store_zip(mut out: impl Write, files: &[(String, Vec<u8>)]) -> io::Result<()> {
    let mut central = Vec::new();
    let mut offset = 0_u32;
    for (name, data) in files {
        let name = name.as_bytes();
        let crc = crc32(data);
        let size =
            u32::try_from(data.len()).map_err(|_| io::Error::other("ZIP entry is too large"))?;
        out.write_all(&0x0403_4b50_u32.to_le_bytes())?;
        put_u16(&mut out, 20)?;
        put_u16(&mut out, 0)?;
        put_u16(&mut out, 0)?;
        put_u16(&mut out, 0)?;
        put_u16(&mut out, 0)?;
        put_u32(&mut out, crc)?;
        put_u32(&mut out, size)?;
        put_u32(&mut out, size)?;
        put_u16(&mut out, name.len() as u16)?;
        put_u16(&mut out, 0)?;
        out.write_all(name)?;
        out.write_all(data)?;
        central.extend_from_slice(&0x0201_4b50_u32.to_le_bytes());
        for value in [20_u16, 20, 0, 0, 0, 0] {
            central.extend_from_slice(&value.to_le_bytes());
        }
        for value in [crc, size, size] {
            central.extend_from_slice(&value.to_le_bytes());
        }
        for value in [name.len() as u16, 0, 0, 0, 0] {
            central.extend_from_slice(&value.to_le_bytes());
        }
        central.extend_from_slice(&0_u32.to_le_bytes());
        central.extend_from_slice(&offset.to_le_bytes());
        central.extend_from_slice(name);
        offset = offset
            .checked_add(30 + name.len() as u32 + size)
            .ok_or_else(|| io::Error::other("ZIP is too large"))?;
    }
    let central_offset = offset;
    out.write_all(&central)?;
    out.write_all(&0x0605_4b50_u32.to_le_bytes())?;
    let count = u16::try_from(files.len()).map_err(|_| io::Error::other("too many ZIP entries"))?;
    put_u16(&mut out, 0)?;
    put_u16(&mut out, 0)?;
    put_u16(&mut out, count)?;
    put_u16(&mut out, count)?;
    put_u32(&mut out, central.len() as u32)?;
    put_u32(&mut out, central_offset)?;
    put_u16(&mut out, 0)
}

fn put_u16(out: &mut impl Write, value: u16) -> io::Result<()> {
    out.write_all(&value.to_le_bytes())
}
fn put_u32(out: &mut impl Write, value: u32) -> io::Result<()> {
    out.write_all(&value.to_le_bytes())
}
fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = !0_u32;
    for byte in bytes {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            crc = (crc >> 1) ^ (0xedb8_8320 & 0_u32.wrapping_sub(crc & 1));
        }
    }
    !crc
}
