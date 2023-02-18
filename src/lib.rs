pub mod misc;
mod block;
mod blockheader;
mod transaction_data;
mod input;
mod output;

pub use block::*;
pub use blockheader::*;
pub use transaction_data::*;
pub use input::*;
pub use output::*;

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use super::block::Block;

    #[test]
    fn it_works() {
        Block::load(PathBuf::from_str("D:\\BTC\\Bitcoin\\blocks\\blk00000.dat").unwrap()).unwrap();
    }
}
