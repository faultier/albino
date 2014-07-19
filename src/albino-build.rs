#![crate_name="albino-run"]
#![crate_type="bin"]
#![feature(phase)]
#![unstable]

#[phase(plugin, link)] extern crate log;

extern crate getopts;
extern crate whitebase;

use getopts::{optopt, getopts, Matches};
use std::os;
use std::io::IoError;
use whitebase::syntax::{Compile, Brainfuck, DT, Ook, Whitespace};
use util::{ErrorHandler, SourceReadCommand, SourceReadWriteCommand, Target};

mod util;

fn build<B: Buffer, W: Writer, C: Compile>(input: &mut B, output: &mut W, syntax: C) {
    match syntax.compile(input, output) {
        Err(e) => {
            println!("{}", e);
            os::set_exit_status(1);
        }
        _ => (),
    }
}

struct BuildCommand;

impl ErrorHandler for BuildCommand {
    fn handle_error(&self, e: IoError) {
        println!("{}", e);
        os::set_exit_status(1);
    }
}

impl SourceReadCommand for BuildCommand {
    fn handle_input<B: Buffer>(&self, m: &Matches, buffer: &mut B, target: Option<Target>) {
        self.select_output(m, buffer, target)
    }
}

impl SourceReadWriteCommand for BuildCommand {
    fn handle_io<B: Buffer, W: Writer>(&self, _: &Matches, buffer: &mut B, writer: &mut W, target: Option<Target>) {
        match target {
            Some(util::Brainfuck)  => build(buffer, writer, Brainfuck::new()),
            Some(util::DT)         => build(buffer, writer, DT::new()),
            Some(util::Ook)        => build(buffer, writer, Ook::new()),
            Some(util::Whitespace) => build(buffer, writer, Whitespace::new()),
            _ => {
                println!("syntax should be \"bf\", \"dt\", \"ook\" or \"ws\" (default: ws)");
                os::set_exit_status(1);
            },
        }
    }
}

fn main() {
    debug!("executing; cmd=albino-run; args={}", os::args());

    let opts = [
        optopt("o", "", "set output file name", "NAME"),
        optopt("s", "syntax", "set input file syntax", "SYNTAX"),
        ];
    let matches = match getopts(os::args().tail(), opts) {
        Ok(m) => { m }
        Err(e) => {
            println!("{}", e)
                os::set_exit_status(1);
            return;
        }
    };

    BuildCommand.select_input(&matches);
}
