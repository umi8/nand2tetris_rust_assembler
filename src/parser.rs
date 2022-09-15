pub mod parser {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use regex::Regex;

    use crate::parser::CommandType;

    pub struct Parser {
        reader: BufReader<File>,
        command: String,
    }

    impl Parser {
        pub fn new() -> Result<Self, &'static str> {
            let asm = File::open("Prog.asm");

            match asm {
                Ok(file) => {
                    let reader = BufReader::new(file);
                    Ok(Parser {
                        reader,
                        command: "".parse().unwrap(),
                    })
                }
                Err(_) => Err("file error"),
            }
        }

        pub fn has_more_commands(&mut self) -> bool {
            loop {
                let mut buf = String::new();
                return match &self.reader.read_line(&mut buf) {
                    Ok(0) => false,
                    Ok(_) => {
                        let v = buf.trim();
                        if v.starts_with("//") {
                            continue;
                        }
                        if v.is_empty() {
                            continue;
                        }
                        self.command = v.to_string();
                        true
                    }
                    Err(_) => false
                }
            }
        }

        pub fn command_type(&self) -> CommandType {
            CommandType::from(&self.command)
        }

        pub fn symbol(&self) -> String {
            // @をとる
            let value = &self.command.replace("@", "");
            // 10進数の数値に型変換する
            let num = value.parse::<i32>().unwrap();
            num.to_string()
        }

        pub fn dest(&self) -> String {
            let re = Regex::new(r"^(.{1,3}=)*(.*?)(;.{3})*$").unwrap();
            let caps = re.captures(&self.command).unwrap();
            let dest_mnemonic = caps.get(1).map_or("", |m| m.as_str()).replace("=", "");
            dest_mnemonic
        }

        pub fn comp(&self) -> String {
            let re = Regex::new(r"^(.{1,3}=)*(.*?)(;.{3})*$").unwrap();
            let caps = re.captures(&self.command).unwrap();
            let comp_mnemonic = caps.get(2).map_or("", |m| m.as_str());
            comp_mnemonic.to_string()
        }

        pub fn jump(&self) -> String {
            let re = Regex::new(r"^(.{1,3}=)*(.*?)(;.{3})*$").unwrap();
            let caps = re.captures(&self.command).unwrap();
            let jump_mnemonic = caps.get(3).map_or("", |m| m.as_str()).replace(";", "");
            jump_mnemonic
        }
    }
}

pub enum CommandType {
    A,
    C,
}

impl CommandType {
    fn from(command: &String) -> CommandType {
        return if command.starts_with("@") {
            CommandType::A
        } else {
            CommandType::C
        }
    }
}
