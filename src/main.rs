use common::common as SysTime;
use file_reader::file_reader::{AsyncFileEmitter, AsyncFileReciever};
use futures::{executor, future, join, lock::Mutex, FutureExt};
use std::{
    sync::{mpsc, Arc},
    thread,
};
pub mod common;
pub mod file;
pub mod file_reader;
mod reactive;

fn main() {
    let start_time = SysTime::get_current_milis();
    let (tx, rx) = mpsc::channel();

    let t1 = thread::spawn(move || {
        AsyncFileEmitter::emit(tx, "C:\\Users\\luyen");
    });

    let t2 = thread::spawn(move || {
        AsyncFileReciever::distribute(rx, 8);
    });

    t1.join().unwrap();
    t2.join().unwrap();
    println!("IT TAKES {}ms", SysTime::get_current_milis() - start_time);
}
