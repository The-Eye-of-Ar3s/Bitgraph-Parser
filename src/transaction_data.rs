use std::{io::Cursor};
use byteorder::{ReadBytesExt, LittleEndian};
use json::{object, JsonValue};
use super::output::Output;
use super::input::Input;
use sha2::{Sha256, Digest};
use hex;
use super::varint::VarInt;

pub struct TransactionData {
    pub version: u32,
    pub input_count: VarInt,
    pub inputs: Vec<Input>,
    pub output_count: VarInt,
    pub outputs: Vec<Output>,
    pub locktime: u32
}

impl TransactionData {
    pub fn load(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, ()> {
        let version = match cursor.read_u32::<LittleEndian>() { Err(_) => {return Err(())} Ok(v) => {v} };
        let input_count: VarInt = match VarInt::from_cursor(cursor) { Err(_) => { return Err(()) } Ok(v) => {v} };
        let mut inputs: Vec<Input> = vec![];
        for _ in 0..input_count.value {
            match Input::load(cursor) { Err(_) => {return Err(())} Ok(v) => {inputs.push(v)} }
        }
        let output_count: VarInt = match VarInt::from_cursor(cursor) { Err(_) => { return Err(()) } Ok(v) => {v} };
        let mut outputs: Vec<Output> = vec![];
        for _ in 0..output_count.value {
            match Output::load(cursor) { Err(_) => {return Err(())} Ok(v) => {outputs.push(v)} }
        }
        let locktime = match cursor.read_u32::<LittleEndian>() { Err(_) => {return Err(())} Ok(v) => {v} };
        return Ok( TransactionData { version: version, input_count: input_count, inputs: inputs, output_count: output_count, outputs: outputs, locktime: locktime })
    }
}

impl TransactionData {
    pub fn to_json(&self) -> Result<JsonValue, ()> {
        let mut inputs = json::JsonValue::new_array();
        for i in &self.inputs {
            match inputs.push(i.to_json()) { Err(_) => {return Err(())} Ok(()) => {} };
        }
        let mut outputs = json::JsonValue::new_array();
        for i in &self.outputs {
            match outputs.push(i.to_json()) { Err(_) => {return Err(())} Ok(()) => {} };
        }
        return Ok(object! {
            txid: self.calculate_txid(),
            version: self.version,
            input_count: self.input_count.value,
            input: inputs,
            output_count: self.output_count.value,
            output: outputs,
            locktime: self.locktime
        })
    }
}

impl TransactionData {
    pub fn calculate_txid(&self) -> String {
        let mut hasher: Sha256 = Sha256::new();
        hasher.update(self.to_binary_data());
        let round1 = hasher.finalize();
        let mut hasher: Sha256 = Sha256::new();
        hasher.update(round1);
        return hex::encode(&hasher.finalize()[..]);
    }

    pub fn to_binary_data(&self) -> Vec<u8> {
        let mut data: Vec<u8> = vec![];
        data.append(&mut self.version.to_be_bytes().to_vec());
        data.append(&mut self.input_count.to_binary());
        for i in &self.inputs {
            data.append(&mut i.to_binary())
        }
        data.append(&mut self.output_count.to_binary());
        for i in &self.outputs {
            data.append(&mut i.to_binary())
        }
        data.append(&mut self.locktime.to_be_bytes().to_vec());
        data.reverse();
        return data;
    }
}