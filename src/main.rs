use file_reader::file_reader::{AsyncFileEmitter, AsyncFileReciever};
use std::sync::mpsc;

pub mod common;
pub mod file;
pub mod file_reader;
mod reactive;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel();
    let emiter = AsyncFileEmitter::emit(tx, "C:\\Users\\luyen\\Desktop\\Projects");
    emiter.await;
    AsyncFileReciever::contribute(rx, 8);
}
