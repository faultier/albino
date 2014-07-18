#![crate_name="albino"]
#![crate_type="bin"]
#![feature(phase)]

#[phase(plugin, link)] extern crate log;

extern crate whitebase;

use std::io::{BufReader, MemReader, MemWriter};
use whitebase::machine;
use whitebase::syntax::{Compiler, Whitespace};

fn main() {
    let src = "   \t\t \t  \t\n   \t  \t   \n\t\n  \t\n  \n\n\n";
    let mut buffer = BufReader::new(src.as_bytes());
    let mut writer = MemWriter::new();
    let ws = Whitespace::new();
    match ws.compile(&mut buffer, &mut writer) {
        Err(e) => fail!("{}", e),
        _ => {
            let mut reader = MemReader::new(writer.unwrap());
            let mut machine = machine::with_stdio();
            match machine.run(&mut reader) {
                Err(e) => fail!("{}", e),
                _ => (),
            }
        },
    }
}
