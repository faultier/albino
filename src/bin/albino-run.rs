#![crate_name="albino-run"]
#![crate_type="bin"]
#![feature(phase)]
#![unstable]

#[phase(plugin, link)] extern crate log;

extern crate getopts;
extern crate whitebase;
extern crate albino;

use getopts::Matches;
use std::os;
use std::io::{IoError, MemReader, MemWriter};
use whitebase::machine;
use whitebase::syntax::{Compiler, Assembly, Brainfuck, DT, Ook, Whitespace};

use albino::command::{RunCommand, RunExecutable};
use albino::util;
use albino::util::Target;

fn run<B: Buffer, C: Compiler>(buffer: &mut B, syntax: C) {
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
                    os::set_exit_status(2);
                }
                _ => (),
            }
        },
    }
}

struct CommandBody;

impl RunExecutable for CommandBody {
    fn handle_error(&self, e: IoError) {
        println!("{}", e);
        os::set_exit_status(1);
    }

    fn exec<B: Buffer>(&self, _: &Matches, buffer: &mut B, target: Option<Target>) {
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

fn main() {
    debug!("executing; cmd=albino-run; args={}", os::args());

    let mut opts = vec!();
    let cmd = RunCommand::new("run",
                              "[-s syntax] [file]",
                              &mut opts, CommandBody);
    cmd.exec();
}
