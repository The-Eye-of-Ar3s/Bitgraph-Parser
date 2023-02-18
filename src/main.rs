mod block;
mod blockheader;
mod input;
mod misc;
mod output;
mod transaction_data;
use block::Block;
use std::{path::PathBuf, str::FromStr};

fn main() {
    match Block::load(PathBuf::from_str("D:\\BTC\\Bitcoin\\blocks\\blk00000.dat").unwrap()) {
        Ok(v) => {
            v.export("./export.json").unwrap();
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
