use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() -> std::io::Result<()> {
    let asm = File::open("Prog.asm")?;
    let reader = BufReader::new(asm);

    let mut file = File::create("Prog.hack")?;
    for line in reader.lines() {
        if let Ok(command) = line {
            match CommandType::from(command) {
                CommandType::A => file.write(b"A\n")?,
                CommandType::C => file.write(b"C\n")?,
            };
        };
    }
    Ok(())
}

enum CommandType {
    A,
    C,
}

impl CommandType {
    fn from(command: String) -> CommandType {
        return if command.starts_with("@") {
            CommandType::A
        } else {
            CommandType::C
        }
    }
}
