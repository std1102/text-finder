use common::common as SysTime;
extern crate num_cpus;
use clap::Parser;
use file::file::File as CustomFile;
use file_reader::{
    file_reader::{is_exist, AsyncFileEmitter, AsyncFileReciever},
    result::{self, READ_RESULT},
};
use std::{
    env::{self},
    fs,
    path::PathBuf,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

pub mod common;
pub mod file;
pub mod file_reader;
mod reactive;

#[derive(Parser, Debug)]
/// Find text in directory or file with the best performace
struct Args {
    /// Path of file or directory you want to find
    #[arg(short = 'p')]
    path: String,
    /// String you want to find
    #[arg(short = 'f')]
    find_text: String,
    /// Thread size you want to use, default is 4
    #[arg(short = 't', default_value_t = 4)]
    thread_size: usize,
}

fn main() {
    let args = Args::parse();
    let srcdir = PathBuf::from(args.path);
    let absolute_path = fs::canonicalize(&srcdir)
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap()
        .to_string();
    match is_exist(&absolute_path) {
        READ_RESULT::TRUE(_) => {
            let (tx, rx) = mpsc::channel();
            excute(absolute_path, args.find_text, args.thread_size, tx, rx)
        }
        READ_RESULT::FALSE => {
            println!("Path not found!");
            return;
        }
        READ_RESULT::ERROR => {
            println!("Error reading path");
            return;
        }
    }
}

fn excute(
    path: String,
    find_string: String,
    thread_size: usize,
    sender: Sender<CustomFile>,
    reciever: Receiver<CustomFile>,
) {
    let start_time = SysTime::get_current_milis();
    let t1 = thread::spawn(move || {
        AsyncFileEmitter::emit(sender.clone(), &path);
    });

    let c_find_string = find_string.clone();
    let t2 = thread::spawn(move || {
        AsyncFileReciever::distribute(reciever, thread_size, c_find_string);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    println!("It takes {}ms", SysTime::get_current_milis() - start_time);
}
