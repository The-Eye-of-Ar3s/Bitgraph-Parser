use std::fs;
use super::block::Block;
use sha2::{Sha256, Digest};

pub fn load_blk_and_dump(path: &str) -> Result<(), ()> {
    let blocks= match Block::load_all(path) { Err(_) => {return Err(())} Ok(v) => {v} };
    let mut json_blocks = json::JsonValue::new_array();
    let mut c = 0;
    let len = blocks.len();
    for i in blocks {
        println!("{} - {}", c, len);
        c+=1;
        match i { Err(_) => {return Err(())} Ok(v) => {
            match v.export() { Err(_) => {return Err(())} Ok(j) => {
                match json_blocks.push(j) { Err(_) => {return Err(())} Ok(_) => {}}
            }}
        }}
    }
    return match fs::write("./export_all.json", json_blocks.to_string()) {Err(_) => {Err(())} Ok(_) => {Ok(())}};
}

pub fn load_block_and_dump(path: &str) -> Result<(), ()> {
    let j = match Block::load_one(path) { Err(_) => {return Err(())} Ok(v) => {
        match v.export() { Err(_)=>{return Err(())} Ok(j) => {j}}
    }};
    return match fs::write("./export.json", j.to_string()) {Err(_) => {Err(())} Ok(_) => {Ok(())}};
}

pub fn hash(data: Vec<u8>) -> String {
    let mut hasher = Sha256::new();   
    hasher.update(data);
    let result1 = hasher.finalize();
    let mut hasher = Sha256::new();   
    hasher.update(result1);
    return hex::encode(&hasher.finalize()[..]);
}