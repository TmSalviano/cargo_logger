#![allow(unused)]
use std::env;
use std::env::args;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::fs::Permissions;
use std::io;
use std::io::stdin;
use std::io::Error;
use std::io::Write;
use std::os::unix::fs::FileExt;
use std::path;
use std::path::Path;
use std::process;
use std::process::Command;
use std::process::Stdio;
fn main() -> std::io::Result<()> {
    // Why the design choice of a wrapper? A wrapper is non intrusive. Thanks to cargo_logger
    // being a wrapper you can choose if you want to create the log/ directory for every project
    //you create. The wrapper desingn choice also allow for you to only pipe the stdout and stderr of
    //cargo build and run commands when you want to, instead of being mandatory.

    //!!! the options operating on ./ (working directory) will be refactored to take <PATH>
    //<PATH> is ./ if no PATH is specified
    //<FILE_STREAM is out, err or all
    //OPTIONS -c <PATH> -> logs/ dir with stdout.log and stderr.log |
    //        -o -> this follows the last lines of the stdout.log file with tail
    //        -e -> tails the stderr.log
    //        -t <FILE_STREAM> -> truncates the stderr and stoud log files
    //        -h -> displays help info

    let args_buffer: Vec<String> = env::args().into_iter().collect();
    match args_buffer[1].as_str() {
        //DONE
        "-c" | "-create" => {
            if let Err(_) = is_len_valid(&args_buffer, 3, 3) {
                return Ok(());
            }

            let path = Path::new(&args_buffer[2]);
            if !path.exists() {
                println!("Path not found");
                return Ok(());
            }
            create(path)?;

            println!("logs/ folder successfully created");
        }
        "-o" | "--out" => {
            if let Err(_) = is_len_valid(&args_buffer, 2, 2) {
                return Ok(());
            }
            tail("./logs/stdout.log")?;
        }
        "-e" | "--err" => {
            if let Err(_) = is_len_valid(&args_buffer, 2, 2) {
                return Ok(());
            }
            tail("./logs/stderr.log")?;
        }
        //Done
        "-t" | "--truncate" => {
            if let Err(_) = is_len_valid(&args_buffer, 3, 3) {
                return Ok(());
            }
            match args_buffer[2].as_str() {
                "all" => truncate("all")?,
                "out" => truncate("out")?,
                "err" => truncate("err")?,
                _ => println!(
                    "Invalid input\nTry:\tcargo_logger -h or man cargo_logger for more information"
                ),
            }
        }
        //Requires Refactor
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

fn truncate(argument: &str) -> std::io::Result<()> {
    if !["all", "err", "out"].contains(&argument) {
        println!("Invalid input\nTry:\tcargo_logger -h or man cargo_logger for more information");
        return Ok(());
    }
    let logs_dir = Path::new("./logs");
    if !logs_dir.exists() || !logs_dir.is_dir() {
        println!("logs/ not found in working directory");
        return Ok(());
    }
    let log_files: Vec<_> = fs::read_dir(logs_dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let name = entry.file_name();
            let name = name.as_os_str();
            if (name == OsStr::new("stdout.log") || name == OsStr::new("stderr.log"))
                && entry.file_type().ok()?.is_file()
            {
                Some(entry.path())
            } else {
                None
            }
        })
        .collect();

    fn is_special_target(argument: &str, file_name: Option<&OsStr>) -> bool {
        if let Some(name) = file_name {
            match argument {
                "err" => name == OsStr::new("stderr.log"),
                "out" => name == OsStr::new("stdout.log"),
                _ => false,
            }
        } else {
            false
        }
    }
    for file_path in log_files {
        let mut open_options = OpenOptions::new();
        let open_options = open_options.write(true).truncate(true);
        let file_name = file_path.file_name();

        if argument == "all" || is_special_target(argument, file_name) {
            open_options.open(file_path)?;
        }
    }
    println!("Log files truncated successfully!");
    Ok(())
}

fn tail(argument: &str) -> io::Result<()> {
    if !["./logs/stdout.log", "./logs/stderr.log"].contains(&argument) {
        println!("Invalid input\nTry:\tcargo_logger -h or man cargo_logger for more information");
        return Ok(());
    }
    let mut file = File::open(argument)?;
    if file.metadata()?.len() == 0 {
        file.write("".as_bytes());
    }

    let mut tail_cargo_out = Command::new("tail")
        .args(["-f", argument])
        .stdout(Stdio::inherit())
        .spawn()
        .expect("<tail -f .> fail to start");

    if argument == "./logs/stdout.log" {
        println!("cargo_logger is now following stdout.log updates");
    } else {
        println!("cargo_logger is now following stderr.log updates")
    }

    let tail_output = tail_cargo_out.wait();

    assert!(tail_output.is_ok_and(|x| x.success()));
    return Ok(());
}
