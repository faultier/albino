#![crate_name="albino"]
#![crate_type="rlib"]
#![feature(phase)]
#![unstable]

#[phase(plugin, link)] extern crate log;
extern crate getopts;

pub static VERSION_MAJOR: uint = 0;
pub static VERSION_MINOR: uint = 1;
pub static VERSION_TINY: uint = 0;
pub static PRE_RELEASE: bool = true;

pub fn version() -> String {
    format!("{}.{}.{}{}",
            VERSION_MAJOR, VERSION_MINOR, VERSION_TINY,
            if PRE_RELEASE { "-pre" } else { "" })
}

pub mod command;
pub mod util;
