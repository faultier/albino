#![experimental]

use getopts::Matches;
use std::io::{BufferedReader, File, IoError, Open, Write};
use std::io::{stdin, stdout};

pub enum Target {
    Assembly,
    Brainfuck,
    DT,
    Ook,
    Whitespace,
}

pub trait ErrorHandler {
    fn handle_error(&self, IoError);
}

pub trait SourceReadCommand: ErrorHandler {
    fn handle_input<B: Buffer>(&self, &Matches, &mut B, Option<Target>);

    fn select_input(&self, m: &Matches) {
        let syntax = m.opt_str("s");
        if !m.free.is_empty() {
            let ref filename = m.free[0];
            match File::open(&Path::new(filename.as_slice())) {
                Ok(file) => {
                    let mut buffer = BufferedReader::new(file);
                    self.handle_input(m, &mut buffer, detect_target(syntax, filename));
                }
                Err(e) => self.handle_error(e),
            }
        } else {
            self.handle_input(m, &mut stdin(), detect_target(syntax, &"".to_string()));
        }
    }
}

pub trait SourceReadWriteCommand: SourceReadCommand {
    fn handle_io<B: Buffer, W: Writer>(&self, &Matches, &mut B, &mut W, Option<Target>);

    fn select_output<B: Buffer>(&self, m: &Matches, buffer: &mut B, target: Option<Target>) {
        match m.opt_str("o") {
            Some(ref name) => {
                match File::open_mode(&Path::new(name.as_slice()), Open, Write) {
                    Ok(ref mut output) => self.handle_io(m, buffer, output, target),
                    Err(e) => self.handle_error(e),
                }
            },
            None => self.handle_io(m, buffer, &mut stdout(), target),
        }
    }
}

pub trait ByteCodeReadCommand: ErrorHandler {
    fn handle_input<B: Buffer>(&self, &Matches, &mut B);

    fn select_input(&self, m: &Matches) {
        if !m.free.is_empty() {
            let ref filename = m.free[0];
            match File::open(&Path::new(filename.as_slice())) {
                Ok(file) => {
                    let mut buffer = BufferedReader::new(file);
                    self.handle_input(m, &mut buffer);
                }
                Err(e) => self.handle_error(e),
            }
        } else {
            self.handle_input(m, &mut stdin());
        }
    }
}

pub fn detect_target(option: Option<String>, filename: &String) -> Option<Target> {
    match option {
        Some(ref val) => match val.as_slice() {
            "asm" => Some(Assembly),
            "bf"  => Some(Brainfuck),
            "dt"  => Some(DT),
            "ook" => Some(Ook),
            "ws"  => Some(Whitespace),
            _     => None,
        },
        None => {
            let slice = filename.as_slice();
            let comps: Vec<&str> = slice.split('.').collect();
            if comps.len() < 2 {
                Some(Whitespace)
            } else {
                match *comps.last().unwrap() {
                    "asm" => Some(Assembly),
                    "bf"  => Some(Brainfuck),
                    "dt"  => Some(DT),
                    "ook" => Some(Ook),
                    _     => Some(Whitespace),
                }
            }
        },
    }
}
