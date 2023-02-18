use std::io::{Cursor};
use std::{fs, vec};
use byteorder::{ReadBytesExt, LittleEndian, BigEndian};
use json::{object, JsonValue};
use super::blockheader::BlockHeader;
use super::varint::VarInt;
use super::transaction_data::TransactionData;

pub struct Block {
    pub magic: u32,
    /// Size of the upcoming block in bytes
    pub size: u32,
    pub header: BlockHeader,
    pub txcount: VarInt,
    pub transactions: Vec<TransactionData>
}

impl Block {
    pub fn load_one(path: &str) -> Result<Self, ()> {
        let raw: Vec<u8> = match fs::read(path) { Err(_) => {return Err(())} Ok(v) => {v} };
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(raw);

        return Self::load(&mut cursor);
    }

    fn load(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, ()> {
        let magic: u32 = match cursor.read_u32::<BigEndian>() { Err(_) => {return Err(())} Ok(v) => {v} };
        let size: u32 = match cursor.read_u32::<LittleEndian>() { Err(_) => {return Err(())} Ok(v) => {v} };
        let header: BlockHeader = match BlockHeader::load(cursor) { Err(_) => { return Err(()) } Ok(v) => {v} };
        let txcount: VarInt = match VarInt::from_cursor(cursor) { Err(_) => { return Err(()) } Ok(v) => {v} };
        let mut transactions = vec![];
        for _ in 0..txcount.value {
            match TransactionData::load(cursor) { Err(_) => {return Err(())} Ok(v) => {transactions.push(v)}}
        }
        return Ok(Block { magic: magic, size: size, header: header, txcount: txcount, transactions: transactions});
    }

    pub fn load_all(path: &str) -> Result<Vec<Result<Self, ()>>, ()> {
        let raw: Vec<u8> = match fs::read(path) { Err(_) => {return Err(())} Ok(v) => {v} };
        let len: usize = raw.len();
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(raw);

        let mut blocks: Vec<Result<Self, ()>> = vec![];
        while cursor.position() < len as u64 {
            blocks.push(Self::load(&mut cursor));
        }
        return Ok(blocks);
    }
}

impl Block {
    pub fn export(&self) -> Result<JsonValue, ()> {
        let mut transactions = json::JsonValue::new_array();
        for i in &self.transactions {
            match i.to_json() {
                Ok(v) => {
                    match transactions.push(v) { Err(_) => {return Err(())} Ok(()) => {} };
                }
                Err(_) => {
                    return Err(())
                }
            }
        }
        let data = object! {
            blockheader: self.header.calculate_blockhash(),
            magic: format!("{:02x}",self.magic).to_uppercase(),
            size: self.size,
            block_header: object! {
                version: self.header.version,
                previous_block_hash: self.header.previous_block_hash.into_iter().map(|i| format!("{:x}", i)).collect::<String>().to_uppercase(),
                merkle_root: self.header.merkle_root.into_iter().rev().map(|i| format!("{:x}", i)).collect::<String>().to_uppercase(),
                time: self.header.time,
                bits: self.header.bits,
                nonce: self.header.nonce
            },
            txcount: self.txcount.value,
            transactions: transactions
        };
        return Ok(data);
    }
}