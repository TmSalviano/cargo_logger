#![allow(unused)]
use std::io;
use std::env;
use std::env::args;
use std::fs;
use std::io::stdin;
use std::io::Error;
use std::process;
fn main() -> std::io::Result<()> {
    //OPTIONS -c <PATH> -> logs/ dir with stdout.log and stderr.log |
    //        -o -> this follows the last lines of the stdout.log file with tail
    //        -e -> tails the stderr.log
    //        -t -> truncates the stderr and stoud log files
    //        -h -> displays help info

    let args_buffer: Vec<String> = env::args().into_iter().collect();
    println!("{}", args_buffer[1]);
        
    match args_buffer[1].as_str() {
        "-c" | "-create" => (),
        "-o" | "--out" => (),
        "-e" | "--err" => (),
        "-t" | "--truncate" => (),
        //Done
        "-h" | "--help" => {
            if let Err(error) = is_len_valid(&args_buffer, 2, 2) {
                println!("{}", error);
                return Ok(());
            }
            help();
        },
        _ => (),
    }

    Ok(())
}

fn help() {
    println!("Usage:\tcargo_logger [OPTIONS] <PATH>\nArguments:\n\t <PATH>\nOptions:\n\t-h,--help prints help");
}

fn is_len_valid(args_buffer: &Vec<String>, lowerbound: u8, upperbound: u8) -> io::Result<()> {
    if args_buffer.len() < lowerbound.into() || args_buffer.len() > upperbound.into() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid input\nTry:\tcargo_logger -h or man cargo_logger for more information")) 
    }
    return Ok(())
}
