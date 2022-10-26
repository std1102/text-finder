use std::io::BufRead;
use std::io::BufReader;
use std::ops::Add;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::Thread;

use crate::file;
use crate::file::file::File;
use crate::file::file::BLOCK_SIZE;

pub async fn find_text(recieve: &Receiver<File>) {
    loop {
        match recieve.recv() {
            Ok(msg) => {
                println!("{}", msg.properties.path);
            }
            Err(err) => println!("{:?}", err),
        }
    }

    // loop {
    //     match recieve.recv() {
    //         Ok(file) => {
    //             println!("{}", file.properties.path);
    //             let os_file = std::fs::File::open(&file.properties.path);
    //             match os_file {
    //                 Ok(_file) => {
    //                     let reader = BufReader::new(_file);
    //                     for line in reader.buffer().lines() {
    //                         match line {
    //                             Ok(l) => {
    //                                 println!("{}", l);
    //                             }
    //                             Err(e) => println!("\n"),
    //                         }
    //                     }
    //                 }
    //                 Err(_) => {
    //                     println!("ERROR WHEN OPENING FILE");
    //                 }
    //             }
    //         }
    //         Err(err) => {
    //             println!("ERROR FROM RECIEVER TEXT FINDER {:?}", err);
    //             break;
    //         }
    //     }
    // }
}
