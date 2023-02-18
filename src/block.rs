use std::io::{Cursor};
use std::path::PathBuf;
use std::{fs, vec};
use byteorder::{ReadBytesExt, LittleEndian, BigEndian};
use json::object;
use super::blockheader::BlockHeader;
use super::misc::var_int;
use super::transaction_data::TransactionData;

pub struct Block {
    pub magic: u32,
    /// Size of the upcoming block in bytes
    pub size: u32,
    pub header: BlockHeader,
    pub txcount: u64,
    pub transactions: Vec<TransactionData>
}

impl Block {
    pub fn load(path: PathBuf) -> Result<Self, ()> {
        let raw: Vec<u8> = match fs::read(path) { Err(_) => {return Err(())} Ok(v) => {v} };
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(raw);
        
        let magic: u32 = match cursor.read_u32::<BigEndian>() { Err(_) => {return Err(())} Ok(v) => {v} };
        let size: u32 = match cursor.read_u32::<LittleEndian>() { Err(_) => {return Err(())} Ok(v) => {v} };
        let header: BlockHeader = match BlockHeader::load(&mut cursor) { Err(_) => { return Err(()) } Ok(v) => {v} };
        let txcount: u64 = match var_int(&mut cursor) { Err(_) => { return Err(()) } Ok(v) => {v} };
        let mut transactions = vec![];
        for _ in 0..txcount {
            match TransactionData::load(&mut cursor) { Err(_) => {return Err(())} Ok(v) => {transactions.push(v)}}
        }

        //println!("MAGIC: {:#x}", magic);
        //println!("SIZE: {}", size);
        //println!("HEADER: {:?}", header);
        //println!("TXCOUNT: {}", txcount);
        return Ok(Block { magic: magic, size: size, header: header, txcount: txcount , transactions: transactions})
    }
}

impl Block {
    pub fn export(&self, path:&str) -> Result<(), ()> {
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
        let data = (object! {
            magic: format!("{:02x}",self.magic).to_uppercase(),
            size: self.size,
            block_header: object! {
                version: self.header.version,
                previous_block_hash: self.header.previous_block_hash.into_iter().map(|i| format!("{:x}", i)).collect::<String>().to_uppercase(),
                merkle_root: self.header.merkle_root.into_iter().map(|i| format!("{:x}", i)).collect::<String>().to_uppercase(),
                time: self.header.time,
                bits: self.header.bits,
                nonce: self.header.nonce
            },
            txcount: self.txcount,
            transactions: transactions
        }).to_string();
        match fs::write(path, data) { Err(_) => {return Err(())} Ok(_) => {return Ok(())}};
    }
}