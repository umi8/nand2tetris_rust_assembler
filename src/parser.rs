use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};

use regex::Regex;

use crate::CommandType;

pub struct Parser {
    reader: BufReader<File>,
    command: String,
}

impl Parser {
    pub fn new(file_path: &str) -> Result<Self, &'static str> {
        let asm = File::open(file_path);

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
                    let re = Regex::new(r"//.*").unwrap();
                    let comments_removed = re.replace_all(&buf, "");
                    let command = &comments_removed.trim();
                    if command.is_empty() {
                        continue;
                    }
                    self.command = command.parse().unwrap();
                    true
                }
                Err(_) => false
            }
        }
    }

    pub fn command_type(&self) -> CommandType {
        CommandType::from(&self.command)
    }

    pub fn symbol(&self) -> Result<String, &'static str> {
        return match self.command_type() {
            CommandType::A => {
                Ok(self.command.replace("@", ""))
            }
            CommandType::C => {
                Err("This function doesn't support type C.")
            }
            CommandType::L => {
                Ok(self.command.replace("(", "").replace(")", ""))
            }
        };
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

    pub fn reset_cursor(&mut self) -> u64 {
        self.reader.seek(SeekFrom::Start(0)).unwrap()
    }
}
