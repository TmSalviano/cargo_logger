#![allow(unused)]
use std::fs::File;
use std::fs::Permissions;
use std::path;
use std::env;
use std::env::args;
use std::fs;
use std::io;
use std::io::stdin;
use std::io::Error;
use std::path::Path;
use std::process;
fn main() -> std::io::Result<()> {
    //OPTIONS -c <PATH> -> logs/ dir with stdout.log and stderr.log |
    //        -o -> this follows the last lines of the stdout.log file with tail
    //        -e -> tails the stderr.log
    //        -t -> truncates the stderr and stoud log files
    //        -h -> displays help info

    let args_buffer: Vec<String> = env::args().into_iter().collect();
    match args_buffer[1].as_str() {
        //DONE
        "-c" | "-create" => {
            if let Err(_) = is_len_valid(&args_buffer, 3, 3) {
                return Ok(())
            }
            let path = Path::new(&args_buffer[2]);
            if !path.exists() {
                println!("Path not found");
                return Ok(())
            }
            create(path)?;
            println!("./logs/stdout.log and ./logs/stderr.log were successfully created.");
        },
        "-o" | "--out" => (),
        "-e" | "--err" => (),
        "-t" | "--truncate" => (),
        //DONE
        "-h" | "--help" => {
            if let Err(_) = is_len_valid(&args_buffer, 2, 2) {
                return Ok(());
            }
            help();
        }
        _ => (),
    }

    Ok(())
}

fn help() {
    println!(
        "Usage:
        \n\tcargo_logger -c <PATH>
        \n\tcargo_logger [OPTION]
        \nArguments:
            \n\t <PATH>: path of the cargo project's working directory
        \nOptions:
            \n\t-c, --create: creates the logs/ directory in cargo project
            \n\t-o, --out: tails the stout.log file
            \n\t-e, --err: tails the stderr.log file
            \n\t-t, --truncate: truncates log files
            \n\t-h,--help: prints help"
    );
}

fn is_len_valid(args_buffer: &Vec<String>, lowerbound: u8, upperbound: u8) -> io::Result<()> {
    if args_buffer.len() < lowerbound.into() || args_buffer.len() > upperbound.into() {
        println!("Invalid input\nTry:\tcargo_logger -h or man cargo_logger for more information");
        return Err(Error::new(io::ErrorKind::InvalidInput, "Dummy"));
    }
    return Ok(());
}

fn create(path: &Path) -> io::Result<()> {
            let working_dir = path::absolute(path).unwrap();
            let dir_str = working_dir.to_str().unwrap();
            fs::create_dir(format!("{}/logs", dir_str))?;
            File::create(format!("{}/logs/stdout.log", dir_str))?;
            File::create(format!("{}/logs/stderr.log", dir_str))?;
            return Ok(());
}