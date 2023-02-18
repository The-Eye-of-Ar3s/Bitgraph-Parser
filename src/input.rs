use std::{io::{Cursor, Read}, vec};

use byteorder::{ReadBytesExt, LittleEndian};
use json::{JsonValue, object};
use super::misc::var_int;

pub struct Input {
    pub txid: [u8; 32],
    pub vout: u32,
    pub script_sig_size: u64,
    pub script_sig: Vec<u8>,
    pub sequence: u32
}

impl Input {
    pub fn load(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, ()> {
        let mut txid: [u8; 32] = [0; 32];
        match cursor.read(&mut txid) { Err(_) => {return Err(())} Ok(_) => {} };
        txid.reverse();

        let vout: u32 = match cursor.read_u32::<LittleEndian>() { Err(_) => {return Err(())} Ok(v) => {v} };
        let script_sig_size: u64 = match var_int(cursor) { Err(_) => { return Err(()) } Ok(v) => {v} };
        
        let mut script_sig: Vec<u8> = vec![0; script_sig_size as usize];
        match cursor.read(&mut script_sig) { Err(_) => {return Err(())} Ok(_) => {} };

        let sequence: u32 = match cursor.read_u32::<LittleEndian>() { Err(_) => {return Err(())} Ok(v) => {v} };

        return Ok( Input { txid: txid, vout: vout, script_sig_size: script_sig_size, script_sig: script_sig, sequence: sequence } );
    }
}

impl Input {
    pub fn to_json(&self) -> JsonValue {
        return object! {
            txid: self.txid.into_iter().map(|i| format!("{:02x}", i)).collect::<String>().to_uppercase(),
            vout: self.vout,
            script_sig_size: self.script_sig_size,
            script_sig: (&self.script_sig).into_iter().map(|i| format!("{:02x}", i)).collect::<String>().to_uppercase(),
            sequence: self.sequence
        }
    }
}

impl Input {
    pub fn to_binary(&self) -> Vec<u8> {
        let mut data: Vec<u8> = vec![];
        data.append(&mut self.txid.to_vec());
        data.append(&mut self.vout.to_be_bytes().to_vec());
        data.append(&mut self.script_sig_size.to_be_bytes().to_vec());
        data.append(&mut self.script_sig.clone());
        return data;
    }
}