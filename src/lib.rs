pub mod misc;
mod block;
mod blockheader;
mod transaction_data;
mod input;
mod output;
mod varint;

pub use block::*;
pub use blockheader::*;
pub use transaction_data::*;
pub use input::*;
pub use output::*;

#[cfg(test)]
mod tests {
    use super::block::Block;

    #[test]
    fn it_works() {
        Block::load_one("D:\\BTC\\Bitcoin\\blocks\\blk00000.dat").unwrap();
    }
}
