use file_reader::file_reader::{AsyncFileEmitter, AsyncFileReciever};
use futures::{executor, future, join, FutureExt};
use std::{sync::mpsc, thread};

pub mod common;
pub mod file;
pub mod file_reader;
mod reactive;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel();
    // let reciever = AsyncFileReciever::distribute(rx, 8);
    // let runtime = tokio::runtime::Runtime::new().unwrap();
    // runtime.block_on(
    //     runtime.spawn(async move {reciever.await;})
    // );
    // futures::future::join(emiter, reciever).await;

    tokio::spawn(async move {
        AsyncFileEmitter::emit(tx, "C:\\Users\\luyen\\Desktop\\Projects").await;
    });

    tokio::spawn(async move {
        AsyncFileReciever::distribute(rx, 8).await;
    });
}
