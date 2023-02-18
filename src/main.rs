mod block;
mod blockheader;
mod input;
mod misc;
mod output;
mod transaction_data;
mod varint;

fn main() {
    println!(
        "{:?}",
        misc::load_block_and_dump("D:\\BTC\\Bitcoin\\blocks\\blk00000.dat")
    )
}
