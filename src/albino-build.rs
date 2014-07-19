#![crate_name="albino-run"]
#![crate_type="bin"]
#![feature(phase, macro_rules)]
#![unstable]

#[phase(plugin, link)] extern crate log;

extern crate getopts;
extern crate whitebase;

use getopts::{optopt, getopts};
use std::os;
use std::io::{BufferedReader, File, Open, Write};
use std::io::stdio::{stdin, stdout};
use whitebase::syntax::{Compile, Brainfuck, DT, Ook, Whitespace};

mod util;

macro_rules! select_syntax (
    ($input:expr, $output:expr, $syntax:expr, $filename:expr) =>(match util::detect_target($syntax, $filename) {
        Some(util::Brainfuck)  => build($input, $output, Brainfuck::new()),
        Some(util::DT)         => build($input, $output, DT::new()),
        Some(util::Ook)        => build($input, $output, Ook::new()),
        Some(util::Whitespace) => build($input, $output, Whitespace::new()),
        _ => {
            println!("syntax should be \"bf\", \"dt\", \"ook\" or \"ws\" (default: ws)");
            os::set_exit_status(1);
        },
    })
)

macro_rules! select_output (
    ($input:expr, $output:expr, $syntax:expr, $filename:expr) => (match $output {
        Some(ref name) => {
            match File::open_mode(&Path::new(name.as_slice()), Open, Write) {
                Ok(ref mut output) => select_syntax!($input, output, $syntax, $filename),
                Err(e) => {
                    println!("{}", e);
                    os::set_exit_status(1);
                }
            }
        },
        None => select_syntax!($input, &mut stdout(), $syntax, $filename),
    })
)

fn build<B: Buffer, W: Writer, C: Compile>(input: &mut B, output: &mut W, syntax: C) {
    match syntax.compile(input, output) {
        Err(e) => {
            println!("{}", e);
            os::set_exit_status(1);
        }
        _ => (),
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

    let syntax = matches.opt_str("s");
    let output = matches.opt_str("o");
    if !matches.free.is_empty() {
        let ref filename = matches.free[0];
        match File::open(&Path::new(filename.as_slice())) {
            Ok(file) => {
                let mut buffer = BufferedReader::new(file);
                select_output!(&mut buffer, output, syntax, filename);
            }
            Err(e) => {
                println!("{}", e);
                os::set_exit_status(1);
            }
        }
    } else {
        select_output!(&mut stdin(), output, syntax, &String::new());
    }
}
