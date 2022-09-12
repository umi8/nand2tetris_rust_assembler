use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use regex::Regex;

fn main() -> std::io::Result<()> {
    let asm = File::open("Prog.asm")?;
    let reader = BufReader::new(asm);

    let mut file = File::create("Prog.hack")?;

    for line in reader.lines() {
        if let Ok(command) = line {
            if command.starts_with("//") {
                continue;
            }

            match CommandType::from(&command) {
                CommandType::A => {
                    // @をとる
                    let value = &command.replace("@", "");
                    // 10進数の数値に型変換する
                    let num = value.parse::<i32>().unwrap();
                    // 16bitの2進数で出力する
                    writeln!(&mut file, "{:016b}", num)?
                }
                CommandType::C => {
                    let re = Regex::new(r"^(.{1,3}=)*(.*?)(;.{3})*$").unwrap();
                    let caps = re.captures(&command).unwrap();

                    let comp = caps.get(1).map_or("", |m| m.as_str());
                    let dest = caps.get(2).map_or("", |m| m.as_str());
                    writeln!(&mut file, "{}", &comp.replace("=", ""))?;
                    writeln!(&mut file, "{}", dest)?;

                    let jump_mnemonic = caps.get(3).map_or("", |m| m.as_str()).replace(";", "");
                    let jump_code = jump_map(&jump_mnemonic);
                    writeln!(&mut file, "{}", jump_code)?
                }
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
    fn from(command: &String) -> CommandType {
        return if command.starts_with("@") {
            CommandType::A
        } else {
            CommandType::C
        }
    }
}

fn jump_map(mnemonic: &str) -> &str {
    return match mnemonic {
        "JGT" => "001",
        "JEQ" => "010",
        "JGE" => "011",
        "JLT" => "100",
        "JNE" => "101",
        "JLE" => "110",
        "JMP" => "111",
        &_ => "000"
    }
}
