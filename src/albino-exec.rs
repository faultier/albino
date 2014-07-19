#![crate_name="albino-run"]
#![crate_type="bin"]
#![feature(phase)]
#![unstable]

#[phase(plugin, link)] extern crate log;

extern crate getopts;
extern crate whitebase;

use getopts::{optflag, getopts, Matches};
use std::os;
use std::io::{IoError, MemReader};
use whitebase::machine;
use util::{ErrorHandler, ByteCodeReadCommand};

mod util;

struct ExecCommand;

impl ByteCodeReadCommand for ExecCommand {
    fn handle_input<B: Buffer>(&self, _: &Matches, buffer: &mut B) {
        match buffer.read_to_end() {
            Ok(buf) => {
                let mut reader = MemReader::new(buf);
                let mut machine = machine::with_stdio();
                match machine.run(&mut reader) {
                    Err(e) => {
                        println!("{}", e);
                        os::set_exit_status(1);
                    }
                    _ => (),
                }
            },
            Err(e) => self.handle_error(e),
        }
    }
}

impl ErrorHandler for ExecCommand {
    fn handle_error(&self, e: IoError) {
        println!("{}", e);
        os::set_exit_status(1);
    }
}

fn main() {
    debug!("executing; cmd=albino-run; args={}", os::args());

    let opts = [
        optflag("h", "help", ""),
    ];
    let matches = match getopts(os::args().tail(), opts) {
        Ok(m) => { m }
        Err(e) => {
            println!("{}", e);
            os::set_exit_status(1);
            return;
        }
    };
    // TODO: help

    ExecCommand.select_input(&matches);
}
