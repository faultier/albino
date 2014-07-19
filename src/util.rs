#![experimental]

pub enum Target {
    Assembly,
    Brainfuck,
    DT,
    Ook,
    Whitespace,
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
