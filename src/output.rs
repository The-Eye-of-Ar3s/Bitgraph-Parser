use std::io::{Cursor, Read};

use byteorder::{ReadBytesExt, LittleEndian};
use json::{JsonValue, object};
use super::misc::var_int;

pub struct Output {
    value: u64,
    script_pub_key_size: u64,
    script_pub_key: Vec<u8>
}

impl Output {
    pub fn load(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, ()> {
        let value: u64 = match cursor.read_u64::<LittleEndian>() { Err(_) => {return Err(())} Ok(v) => {v} };
        let script_pub_key_size: u64 = match var_int(cursor) { Err(_) => { return Err(()) } Ok(v) => {v} };
        let mut script_pub_key: Vec<u8> = vec![0; script_pub_key_size as usize];
        match cursor.read(&mut script_pub_key) { Err(_) => {return Err(())} Ok(_) => {} };
        return Ok( Output { value: value, script_pub_key_size: script_pub_key_size, script_pub_key: script_pub_key } );
    }
}

impl Output {
    pub fn to_json(&self) -> JsonValue {
        return object! {
            value: self.value,
            script_pub_key_size: self.script_pub_key_size,
            script_pub_key: (&self.script_pub_key).into_iter().map(|i| format!("{:02x}", i)).collect::<String>().to_uppercase(),
        }
    }
}

impl Output {
    pub fn to_binary(&self) -> Vec<u8> {
        let mut data: Vec<u8> = vec![];
        data.append(&mut self.value.to_be_bytes().to_vec());
        data.append(&mut self.script_pub_key_size.to_be_bytes().to_vec());
        data.append(&mut self.script_pub_key.clone());
        return data;
    }
}