#![crate_name="albino-run"]
#![crate_type="bin"]
#![feature(phase)]
#![unstable]

#[phase(plugin, link)] extern crate log;

extern crate getopts;
extern crate whitebase;

use getopts::{optopt, getopts};
use std::os;
use std::io::{BufferedReader, File, MemReader, MemWriter};
use whitebase::machine;
use whitebase::syntax::{Compile, Assembly, Brainfuck, DT, Ook, Whitespace};

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

fn main() {
    debug!("executing; cmd=albino-run; args={}", os::args());

    let opts = [
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

    let syntax = matches.opt_str("s");
    if !matches.free.is_empty() {
        let ref filename = matches.free[0];
        match File::open(&Path::new(filename.as_slice())) {
            Ok(file) => {
                let mut buffer = BufferedReader::new(file);
                match util::detect_target(syntax, filename) {
                    Some(util::Assembly)   => run(&mut buffer, Assembly::new()),
                    Some(util::Brainfuck)  => run(&mut buffer, Brainfuck::new()),
                    Some(util::DT)         => run(&mut buffer, DT::new()),
                    Some(util::Ook)        => run(&mut buffer, Ook::new()),
                    Some(util::Whitespace) => run(&mut buffer, Whitespace::new()),
                    None => {
                        println!("syntax should be \"asm\", \"bf\", \"dt\", \"ook\" or \"ws\" (default: ws)");
                        os::set_exit_status(1);
                    },
                }
            }
            Err(e) => {
                println!("{}", e);
                os::set_exit_status(1);
            }
        }
    } else {
        unimplemented!()
    }
}
