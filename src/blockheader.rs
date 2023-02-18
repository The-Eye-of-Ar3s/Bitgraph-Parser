use std::io::{Cursor, Read};
use byteorder::{ReadBytesExt, LittleEndian};
use sha2::{Sha256, Digest};

#[derive(Debug)]
pub struct BlockHeader {
    /// The version of the block
    pub version: u32,
    /// The Block Hash (double SHA256) of the block that this block is being built on top of. This is what “chains” the blocks together
    pub previous_block_hash: [u8; 32],
    /// All of the transactions in this block, hashed together. Basically provides a single-line summary of all the transactions in this block.
    pub merkle_root: [u8; 32],
    /// When a miner is trying to mine this block, the Unix time at which this block header is being hashed is noted within the block header itself.
    pub time: u32,
    /// A shortened version of the Target.
    pub bits: u32,
    /// The field that miners change in order to try and get a hash of the block header (a Block Hash) that is below the Target.
    pub nonce: u32
}

impl BlockHeader {
    pub fn load(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, ()> {
        let version: u32 = match cursor.read_u32::<LittleEndian>() { Err(_) => {return Err(())} Ok(v) => {v} };

        let mut previous_block_hash: [u8; 32] = [0; 32];
        match cursor.read(&mut previous_block_hash) { Err(_) => {return Err(())} Ok(_) => {} };
        
        let mut merkle_root: [u8; 32] = [0; 32];
        match cursor.read(&mut merkle_root) { Err(_) => {return Err(())} Ok(_) => {} };

        let time: u32 = match cursor.read_u32::<LittleEndian>() { Err(_) => {return Err(())} Ok(v) => {v} };
        let bits: u32 = match cursor.read_u32::<LittleEndian>() { Err(_) => {return Err(())} Ok(v) => {v} };
        let nonce: u32 = match cursor.read_u32::<LittleEndian>() { Err(_) => {return Err(())} Ok(v) => {v} };

        return Ok(BlockHeader { version: version, previous_block_hash: previous_block_hash, merkle_root: merkle_root, time: time, bits: bits, nonce: nonce });
    }
}

impl BlockHeader {
    pub fn calculate_blockhash(&self) -> String {
        let mut hasher: Sha256 = Sha256::new();
        hasher.update(self.to_binary());
        let round1 = hasher.finalize();
        let mut hasher: Sha256 = Sha256::new();
        hasher.update(round1);
        return hex::encode(&hasher.finalize()[..]);
    }
    pub fn to_binary(&self) -> Vec<u8> {
        let mut data: Vec<u8> = vec![];
        data.append(&mut self.version.to_be_bytes().to_vec());
        data.append(&mut self.previous_block_hash.to_vec());
        data.append(&mut self.merkle_root.to_vec());
        data.append(&mut self.time.to_be_bytes().to_vec());
        data.append(&mut self.bits.to_be_bytes().to_vec());
        data.append(&mut self.nonce.to_be_bytes().to_vec());
        return data;
    }
}