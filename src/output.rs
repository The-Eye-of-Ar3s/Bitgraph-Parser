use std::io::{Cursor, Read};

use byteorder::{ReadBytesExt, LittleEndian};
use json::{JsonValue, object};
use super::varint::VarInt;

pub struct Output {
    value: u64,
    script_pub_key_size: VarInt,
    script_pub_key: Vec<u8>
}

impl Output {
    pub fn load(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, ()> {
        let value: u64 = match cursor.read_u64::<LittleEndian>() { Err(_) => {return Err(())} Ok(v) => {v} };
        let script_pub_key_size: VarInt = match VarInt::from_cursor(cursor) { Err(_) => { return Err(()) } Ok(v) => {v} };
        let mut script_pub_key: Vec<u8> = vec![0; script_pub_key_size.value as usize];
        match cursor.read(&mut script_pub_key) { Err(_) => {return Err(())} Ok(_) => {} };
        return Ok( Output { value: value, script_pub_key_size: script_pub_key_size, script_pub_key: script_pub_key } );
    }
}

impl Output {
    pub fn to_json(&self) -> JsonValue {
        return object! {
            value: self.value.to_le(),
            script_pub_key_size: self.script_pub_key_size.value,
            script_pub_key: (&self.script_pub_key).into_iter().map(|i| format!("{:02x}", i)).collect::<String>().to_uppercase(),
        }
    }
}

impl Output {
    pub fn to_binary(&self) -> Vec<u8> {
        let mut data: Vec<u8> = vec![];
        data.append(&mut self.value.to_be_bytes().to_vec());
        data.append(&mut self.script_pub_key_size.to_binary());
        data.append(&mut self.script_pub_key.clone());
        return data;
    }
}