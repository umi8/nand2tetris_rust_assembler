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

                    let comp_mnemonic = caps.get(2).map_or("", |m| m.as_str());
                    let comp_code = comp_map(&comp_mnemonic);

                    let dest_mnemonic = caps.get(1).map_or("", |m| m.as_str()).replace("=", "");
                    let dest_code = dest_map(&dest_mnemonic);

                    let jump_mnemonic = caps.get(3).map_or("", |m| m.as_str()).replace(";", "");
                    let jump_code = jump_map(&jump_mnemonic);
                    writeln!(&mut file, "{}{}{}{}", "111", comp_code, dest_code, jump_code)?
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

fn comp_map(mnemonic: &str) -> &str {
    return match mnemonic {
        "0" => "0101010",
        "1" => "0111111",
        "-1" => "0111010",
        "D" => "0001100",
        "A" => "0110000",
        "!D" => "0001101",
        "!A" => "0110001",
        "-D" => "0001111",
        "-A" => "0110011",
        "D+1" => "0011111",
        "A+1" => "0110111",
        "D-1" => "0001110",
        "A-1" => "0110010",
        "D+A" => "0000010",
        "D-A" => "0010011",
        "A-D" => "0000111",
        "D&A" => "0000000",
        "D|A" => "0010101",
        "M" => "1110000",
        "!M" => "1110001",
        "-M" => "1110011",
        "M+1" => "1110111",
        "M-1" => "1110010",
        "D+M" => "1000010",
        "D-M" => "1010011",
        "M-D" => "1000111",
        "D&M" => "1000000",
        "D|M" => "1010101",
        &_ => "0000000"
    }
}

fn dest_map(mnemonic: &str) -> &str {
    return match mnemonic {
        "M" => "001",
        "D" => "010",
        "MD" => "011",
        "A" => "100",
        "AM" => "101",
        "AD" => "110",
        "AMD" => "111",
        &_ => "000"
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
