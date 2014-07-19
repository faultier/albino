#![crate_name="albino-run"]
#![crate_type="bin"]
#![feature(phase)]
#![unstable]

#[phase(plugin, link)] extern crate log;

extern crate getopts;
extern crate whitebase;

use getopts::{optopt, getopts, Matches};
use std::os;
use std::io::{IoError, MemReader, MemWriter};
use whitebase::machine;
use whitebase::syntax::{Compile, Assembly, Brainfuck, DT, Ook, Whitespace};

use util::{ErrorHandler, SourceReadCommand, Target};

mod util;

fn run<B: Buffer, C: Compile>(buffer: &mut B, syntax: C) {
    let mut writer = MemWriter::new();
    match syntax.compile(buffer, &mut writer) {
        Err(e) => {
            println!("{}", e);
            os::set_exit_status(1);
        }
        _ => {
            let mut reader = MemReader::new(writer.unwrap());
            let mut machine = machine::with_stdio();
            match machine.run(&mut reader) {
                Err(e) => {
                    println!("{}", e);
                    os::set_exit_status(1);
                }
                _ => (),
            }
        },
    }
}

struct RunCommand;

impl SourceReadCommand for RunCommand {
    fn handle_input<B: Buffer>(&self, _: &Matches, buffer: &mut B, target: Option<Target>) {
        match target {
            Some(util::Assembly)   => run(buffer, Assembly::new()),
            Some(util::Brainfuck)  => run(buffer, Brainfuck::new()),
            Some(util::DT)         => run(buffer, DT::new()),
            Some(util::Ook)        => run(buffer, Ook::new()),
            Some(util::Whitespace) => run(buffer, Whitespace::new()),
            None => {
                println!("syntax should be \"asm\", \"bf\", \"dt\", \"ook\" or \"ws\" (default: ws)");
                os::set_exit_status(1);
            },
        }
    }
}

impl ErrorHandler for RunCommand {
    fn handle_error(&self, e: IoError) {
        println!("{}", e);
        os::set_exit_status(1);
    }
}

fn main() {
    debug!("executing; cmd=albino-run; args={}", os::args());

    let opts = [
        optopt("s", "syntax", "set input file syntax", "SYNTAX"),
        ];
    let matches = match getopts(os::args().tail(), opts) {
        Ok(m) => { m }
        Err(e) => {
            println!("{}", e);
            os::set_exit_status(1);
            return;
        }
    };

    RunCommand.select_input(&matches);
}
