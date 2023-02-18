use std::io::Cursor;
use byteorder::{ReadBytesExt,LittleEndian};

pub enum VarIntVariants {
    None,
    Fd,
    Fe,
    Ff
}

pub struct VarInt {
    pub value: u64,
    pub variant: VarIntVariants
}

impl VarInt {
    pub fn to_binary(&self) -> Vec<u8> {
        let mut v: Vec<u8> = match self.variant {
            VarIntVariants::None => {
                vec![]
            }
            VarIntVariants::Fd => {
                vec![253]
            }
            VarIntVariants::Fe => {
                vec![254]
            }
            VarIntVariants::Ff => {
                vec![255]
            }
        };
        let mut data: Vec<u8> = self.value.to_be_bytes().to_vec();
        v.append(&mut data);
        return v;
    }

    pub fn from_cursor(cursor: &mut Cursor<Vec<u8>>) -> Result<VarInt,()> {
        match cursor.read_u8() {Err(_) => {return Err(())} Ok(v) => {
            match v {
                253 => {
                    match cursor.read_u16::<LittleEndian>() { Err(_) => {return Err(())} Ok(v) => { return Ok(VarInt {value: v as u64, variant: VarIntVariants::Fd}) }}
                }
                254 => {
                    match cursor.read_u32::<LittleEndian>() { Err(_) => {return Err(())} Ok(v) => {return Ok(VarInt {value: v as u64, variant: VarIntVariants::Fe})}}
                }
                255 => {
                    match cursor.read_u64::<LittleEndian>() { Err(_) => {return Err(())} Ok(v) => {return Ok(VarInt {value: v, variant: VarIntVariants::Ff})}}
                }
                _ => {return Ok(VarInt {value: v as u64, variant: VarIntVariants::None})}
            }
        }};
    }
}