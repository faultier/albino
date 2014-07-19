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
use std::io::{IoError, MemReader};
use whitebase::syntax::{Decompiler, Assembly, DT, Whitespace};

use albino::command::{GenerateCommand, GenerateExecutable};
use albino::util;
use albino::util::Target;

fn gen<R: Reader, W: Writer, D: Decompiler>(input: &mut R, output: &mut W, syntax: D) {
    match input.read_to_end() {
        Ok(buf) => {
            let mut reader = MemReader::new(buf);
            match syntax.decompile(&mut reader, output) {
                Err(e) => {
                    println!("{}", e);
                    os::set_exit_status(1);
                },
                _ => (),
            }
        },
        Err(e) => {
            println!("{}", e);
            os::set_exit_status(1);
        },
    }
}

struct CommandBody;

impl GenerateExecutable for CommandBody {
    fn handle_error(&self, e: IoError) {
        println!("{}", e);
        os::set_exit_status(1);
    }

    fn exec<R: Reader, W: Writer>(&self, _: &Matches, reader: &mut R, writer: &mut W, target: Option<Target>) {
        match target {
            Some(util::Assembly)   => gen(reader, writer, Assembly::new()),
            Some(util::DT)         => gen(reader, writer, DT::new()),
            Some(util::Whitespace) => gen(reader, writer, Whitespace::new()),
            _ => {
                println!("syntax should be \"asm\", \"dt\" or \"ws\" (default: ws)");
                os::set_exit_status(1);
            },
        }
    }
}

fn main() {
    debug!("executing; cmd=albino-gen; args={}", os::args());

    let mut opts = vec!();
    let cmd = GenerateCommand::new("gen",
                                "[-s syntax] [-o output] [file]",
                                &mut opts, CommandBody);
    cmd.exec();
}
