#![crate_name="albino"]
#![crate_type="bin"]
#![feature(phase)]

#[phase(plugin, link)] extern crate log;

extern crate whitebase;
extern crate albino;

use std::os;
use std::io::process::{Command,InheritFd,ExitStatus,ExitSignal};

fn main() {
    debug!("executing; cmd=albino; args={}", os::args());

    let (cmd, args) = process(os::args());

    match cmd.as_slice() {
        "--help" | "-h" | "help" | "-?" => {
            println!("Commands:");
            println!("  build          # compile the source code file");
            println!("  exec           # execute the bytecode file");
            println!("  run            # build and execute");
            println!("");
        }
        "--version" | "-v" | "version" => {
            println!("albino {}, whitebase {}", albino::version(), "0.1.1");
            //println!("albino {}, whitebase {}", albino::version(), whitebase::version());
            //println!("albino v{}, whitebase v{}", albino::VERSION, whitebase::VERSION);
        }
        _ => {
            let command = format!("albino-{}{}", cmd, os::consts::EXE_SUFFIX);
            let mut command = match os::self_exe_path() {
                Some(path) => {
                    let p = path.join(command.as_slice());
                    if p.exists() {
                        Command::new(p)
                    } else {
                        Command::new(command)
                    }
                }
                None => Command::new(command),
            };
            let command = command
                .args(args.as_slice())
                .stdin(InheritFd(0))
                .stdout(InheritFd(1))
                .stderr(InheritFd(2))
                .status();

            match command {
                Ok(ExitStatus(0)) => (),
                Ok(ExitStatus(i)) | Ok(ExitSignal(i)) => handle_error("", i),
                Err(_) => handle_error("no such command.", 127),
            }
        }
    }
}

fn process(args: Vec<String>) -> (String, Vec<String>) {
    let mut args = Vec::from_slice(args.tail());
    let head = args.shift().unwrap_or("--help".to_string());

    (head, args)
}

fn handle_error<'a>(message: &'a str, exit: int) {
    println!("{}", message);
    os::set_exit_status(exit)
}
