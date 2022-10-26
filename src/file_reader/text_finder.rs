use crate::file::file::File;
use futures::{AsyncBufReadExt, StreamExt};
use std::{io::BufReader, sync::mpsc::Receiver};

pub fn find_text(recieve: Receiver<File>) {
    loop {
        match recieve.recv() {
            Ok(file) => {
                match std::fs::read_to_string(file.properties.path) {
                    Ok(ok) => {
                        
                    }
                    Err(e) => todo!(),
                }
                // println!("{}", file.properties.path);
                // let os_file = std::fs::File::open(&file.properties.path);
                // match os_file {
                //     Ok(_file) => {
                //         let reader = BufReader::new(_file);
                //         for line in reader.buffer().lines() {
                //             match line {

                //             }
                //         }
                //     }
                //     Err(_) => {
                //         println!("ERROR WHEN OPENING FILE");
                //     }
                // }
            }
            Err(err) => {
                println!("ERROR FROM RECIEVER TEXT FINDER {:?}", err);
                break;
            }
        }
    }
}
