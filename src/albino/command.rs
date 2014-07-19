#![experimental]

use getopts::{Matches, OptGroup, Yes, Maybe, No, getopts, optopt, optflag};
use std::io::{BufferedReader, File, IoError, Open, Write};
use std::io::{stdin, stdout};
use std::os;

use util::{Target, detect_target};

pub trait Executable {
    fn handle_error(&self, IoError);
    fn exec(&self, &Matches);
}

pub struct Command<'a, T> {
    command: &'static str,
    usage: &'static str,
    options: &'a mut Vec<OptGroup>,
    inner: T,
}

impl<'a, E: Executable> Command<'a, E> {
    pub fn new(command: &'static str, usage: &'static str, options: &'a mut Vec<OptGroup>, exec: E) -> Command<'a, E> {
        options.push(optflag("h", "help", ""));
        Command {
            command: command,
            usage: usage,
            options: options,
            inner: exec,
        }
    }

    pub fn exec(&self) {
        let matches = match getopts(os::args().tail(), self.options.as_slice()) {
            Ok(m) => { m }
            Err(e) => {
                println!("{}", e);
                os::set_exit_status(1);
                return;
            }
        };
        if matches.opt_present("h") {
            self.print_usage();
        }
        else {
            self.inner.exec(&matches);
        }
    }

    fn print_usage(&self) {
        println!("usage: albino {} {}", self.command, self.usage);
        println!("\nOptions:");
        for opt in self.options.iter() {
            print!("\t-{}", opt.short_name);
            if opt.long_name.len() > 0 { print!(", --{}", opt.long_name) }
            match opt.hasarg {
                Yes   => { print!(" {}", opt.hint) }
                Maybe => { print!(" [{}]", opt.hint) }
                No    => (),
            }
            if opt.desc.len() > 0 {
                println!("\n\t\t{}", opt.desc)
            } else {
                println!("")
            }
        }
    }
}

pub trait RunExecutable {
    fn handle_error(&self, IoError);
    fn exec<B: Buffer>(&self, &Matches, &mut B, Option<Target>);
}

pub struct RunCommand<T> {
    inner: T
}

impl<E: RunExecutable> RunCommand<E> {
    pub fn new<'a>(command: &'static str, usage: &'static str, options: &'a mut Vec<OptGroup>, exec: E) -> Command<'a, RunCommand<E>> {
        options.push(optopt("s", "syntax", "set input file syntax", "syntax"));
        Command::new(command, usage, options, RunCommand { inner: exec })
    }
}

impl<E: RunExecutable> Executable for RunCommand<E> {
    fn handle_error(&self, e: IoError) {
        self.inner.handle_error(e);
    }

    fn exec(&self, m: &Matches) {
        let syntax = m.opt_str("s");
        if !m.free.is_empty() {
            let ref filename = m.free[0];
            match File::open(&Path::new(filename.as_slice())) {
                Ok(file) => {
                    let mut buffer = BufferedReader::new(file);
                    self.inner.exec(m, &mut buffer, detect_target(syntax, filename));
                }
                Err(e) => self.inner.handle_error(e),
            }
        } else {
            self.inner.exec(m, &mut stdin(), detect_target(syntax, &"".to_string()));
        }
    }
}

pub trait BuildExecutable {
    fn handle_error(&self, IoError);
    fn exec<B: Buffer, W: Writer>(&self, &Matches, &mut B, &mut W, Option<Target>);
}

pub struct BuildCommand<T> {
    inner: T
}

impl<E: BuildExecutable> BuildCommand<E> {
    pub fn new<'a>(command: &'static str, usage: &'static str, options: &'a mut Vec<OptGroup>, exec: E) -> Command<'a, RunCommand<BuildCommand<E>>> {
        options.push(optopt("o", "", "set output file name", "name"));
        RunCommand::new(command, usage, options, BuildCommand { inner: exec })
    }
}

impl<E: BuildExecutable> RunExecutable for BuildCommand<E> {
    fn handle_error(&self, e: IoError) {
        self.inner.handle_error(e);
    }

    fn exec<B: Buffer>(&self, m: &Matches, buffer: &mut B, target: Option<Target>) {
        match m.opt_str("o") {
            Some(ref name) => {
                match File::open_mode(&Path::new(name.as_slice()), Open, Write) {
                    Ok(ref mut output) => self.inner.exec(m, buffer, output, target),
                    Err(e) => self.inner.handle_error(e),
                }
            },
            None => self.inner.exec(m, buffer, &mut stdout(), target),
        }
    }
}

pub trait LoadExecutable {
    fn handle_error(&self, IoError);
    fn exec<R: Reader>(&self, &Matches, &mut R);
}

pub struct LoadCommand<T> {
    inner: T
}

impl<E: LoadExecutable> LoadCommand<E> {
    pub fn new<'a>(command: &'static str, usage: &'static str, options: &'a mut Vec<OptGroup>, exec: E) -> Command<'a, LoadCommand<E>> {
        Command::new(command, usage, options, LoadCommand { inner: exec })
    }
}

impl<E: LoadExecutable> Executable for LoadCommand<E> {
    fn handle_error(&self, e: IoError) {
        self.inner.handle_error(e);
    }

    fn exec(&self, m: &Matches) {
        if !m.free.is_empty() {
            let ref filename = m.free[0];
            match File::open(&Path::new(filename.as_slice())) {
                Ok(ref mut file) => self.inner.exec(m, file),
                Err(e) => self.inner.handle_error(e),
            }
        } else {
            self.inner.exec(m, &mut stdin());
        }
    }
}

pub trait GenerateExecutable {
    fn handle_error(&self, IoError);
    fn exec<R: Reader, W: Writer>(&self, &Matches, &mut R, &mut W, Option<Target>);
}

pub struct GenerateCommand<T> {
    inner: T
}

impl<E: GenerateExecutable> GenerateCommand<E> {
    pub fn new<'a>(command: &'static str, usage: &'static str, options: &'a mut Vec<OptGroup>, exec: E) -> Command<'a, LoadCommand<GenerateCommand<E>>> {
        options.push(optopt("s", "syntax", "set input file syntax", "syntax"));
        options.push(optopt("o", "", "set output file name", "name"));
        LoadCommand::new(command, usage, options, GenerateCommand { inner: exec })
    }
}

impl<E: GenerateExecutable> LoadExecutable for GenerateCommand<E> {
    fn handle_error(&self, e: IoError) {
        self.inner.handle_error(e);
    }

    fn exec<R: Reader>(&self, m: &Matches, reader: &mut R) {
        let syntax = m.opt_str("s");
        match m.opt_str("o") {
            Some(ref name) => {
                match File::open_mode(&Path::new(name.as_slice()), Open, Write) {
                    Ok(ref mut output) => self.inner.exec(m, reader, output, detect_target(syntax, name)),
                    Err(e) => self.inner.handle_error(e),
                }
            },
            None => self.inner.exec(m, reader, &mut stdout(), detect_target(syntax, &"".to_string())),
        }
    }
}
