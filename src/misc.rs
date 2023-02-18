use std::io::Cursor;
use byteorder::{ReadBytesExt, LittleEndian};

pub fn var_int(cursor:  &mut Cursor<Vec<u8>>) -> Result<u64,()> {
    match cursor.read_u8() {Err(_) => {return Err(())} Ok(v) => {
        match v {
            253 => {
                match cursor.read_u16::<LittleEndian>() { Err(_) => {return Err(())} Ok(v) => {return Ok(v as u64)}}
            }
            254 => {
                match cursor.read_u32::<LittleEndian>() { Err(_) => {return Err(())} Ok(v) => {return Ok(v as u64)}}
            }
            255 => {
                match cursor.read_u64::<LittleEndian>() { Err(_) => {return Err(())} Ok(v) => {return Ok(v)}}
            }
            _ => {return Ok(v as u64)}
        }
    }};
}