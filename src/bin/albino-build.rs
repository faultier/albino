#![crate_name="albino-build"]
#![crate_type="bin"]
#![feature(phase)]
#![unstable]

#[phase(plugin, link)] extern crate log;

extern crate getopts;
extern crate whitebase;
extern crate albino;

use getopts::Matches;
use std::os;
use std::io::IoError;
use whitebase::syntax::{Compiler, Assembly, Brainfuck, DT, Ook, Whitespace};

use albino::command::{BuildCommand, BuildExecutable};
use albino::util;
use albino::util::Target;

fn build<B: Buffer, W: Writer, C: Compiler>(input: &mut B, output: &mut W, syntax: C) {
    match syntax.compile(input, output) {
        Err(e) => {
            println!("{}", e);
            os::set_exit_status(1);
        }
        _ => (),
    }
}

struct CommandBody;

impl BuildExecutable for CommandBody {
    fn handle_error(&self, e: IoError) {
        println!("{}", e);
        os::set_exit_status(1);
    }

    fn exec<B: Buffer, W: Writer>(&self, _: &Matches, buffer: &mut B, writer: &mut W, target: Option<Target>) {
        match target {
            Some(util::Assembly)   => build(buffer, writer, Assembly::new()),
            Some(util::Brainfuck)  => build(buffer, writer, Brainfuck::new()),
            Some(util::DT)         => build(buffer, writer, DT::new()),
            Some(util::Ook)        => build(buffer, writer, Ook::new()),
            Some(util::Whitespace) => build(buffer, writer, Whitespace::new()),
            _ => {
                println!("syntax should be \"asm\", \"bf\", \"dt\", \"ook\" or \"ws\" (default: ws)");
                os::set_exit_status(1);
            },
        }
    }
}

fn main() {
    debug!("executing; cmd=albino-build; args={}", os::args());

    let mut opts = vec!();
    let cmd = BuildCommand::new("build",
                                "[-s syntax] [-o output] [file]",
                                &mut opts, CommandBody);
    cmd.exec();
}
