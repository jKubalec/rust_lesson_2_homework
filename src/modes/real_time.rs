use std::ops::Deref;
use std::thread;
use std::fs;
use std::io;
use std::error::Error;
use std::thread::JoinHandle;

use flume::Receiver;
use flume::Sender;

use crate::model::command::Command;
use crate::model::transform_mode::TransformMode;

use crate::transformations::csv::Csv;
use crate::transformations::csv::CsvError::*;

fn spawn_input_thread(tx: Sender<Command>) -> JoinHandle<()> {
    thread::spawn(move || {
        let stdin = io::stdin();
        let mut line = String::new();
        
        loop {
            println!("Write command followed by input - e.g. \"slugify I love Rust!\". In case of \"csv\" command enter CSV filename. Empty line for finish");

            line.clear();
            stdin.read_line(&mut line).expect("Failed to read line");

            let binding =  String::from(line.trim());
            if binding.is_empty() {
                println!("Empty line recieved. Input thread loop end.");
                tx.send(Command::STOP).expect("Failed to send STOP command.");
                break;
            }
            let input_parts: Vec<&str> = binding.splitn(2, ' ').collect();
            let command_name = input_parts.get(0).unwrap_or(&"");
            let input = input_parts.get(1).unwrap_or(&"");

            let command = match TransformMode::from_str(&command_name) {
                Some(transform) => Command{command: transform, input: String::from(input.deref())},
                None => {
                    eprintln!("Unknown command on input ({}). Skipping.", command_name);
                    continue;
                }
            };

            tx.send(command).expect("Failed to send data");
        }
    })
}

fn spawn_recieve_thread(rx: Receiver<Command>) -> JoinHandle<()> {
    thread::spawn(move || {
        loop {
            let incoming_cmd = rx.recv();
            let command = match incoming_cmd {
                Ok(cmd) => cmd,
                Err(e) => {
                    eprintln!("{}", e);
                    continue;
                }
            };

            match command {
                Command {command: TransformMode::Stop, input: _} => {
                        println!("Recieved STOP command. Transformation thread loop end.");
                        break;
                    },
                Command {command: TransformMode::CsvParse, input: filename} => {
                    let file_read_res = fs::read_to_string(filename);
                    match file_read_res {
                        Ok(content) => {
                            let parse_result = Csv::parse_text(content);
                            match parse_result {
                                Ok(csv) => println!("{}", csv),
                                Err(e) => match e {
                                    EmptyString => 
                                        eprintln!("Empty string on input. Doing nothing."),
                                    WrongLine(wrong_line) =>
                                        eprintln!("Line of wrong length on the input. Exiting:\n{}", wrong_line),
                                }
                            }
                        },
                        Err(e) => {
                            eprintln!("Could not read file: {}", e);
                            break;
                        }
                    };
                },
                Command {command: simple_transfom, input: text} => {
                    let result: Result<String, Box<dyn Error>> = Command{command: simple_transfom, input: text}.execute();
                    match result {
                        Ok(text) => println!("{}", text),
                        Err(e) => eprintln!("{}",e),
                    };
                },
            };
        }
    })
}

pub fn real_time_processing() -> () {
    let (tx, rx) = flume::unbounded();
            
    let input_thread = spawn_input_thread(tx);

    let transformation_thread = spawn_recieve_thread(rx);
    input_thread.join().expect("Failed to close input thread.");
    transformation_thread.join().expect("Failed to close transformation thread.");
}
