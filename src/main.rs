use std::fs::File;
use std::io::Write;

use crate::command_type::CommandType;
use crate::parser::Parser;
use crate::symbol_table::SymbolTable;

mod code;
mod parser;
mod command_type;
mod symbol_table;

fn main() -> std::io::Result<()> {
    let mut file = File::create("Prog.hack")?;
    let mut symbol_table = SymbolTable::new();

    let mut parser = match Parser::new("Prog.asm") {
        Ok(parser) => parser,
        Err(why) => panic!("couldn't parse: {}", why)
    };

    let mut address = 0;
    while parser.has_more_commands() {
        match parser.command_type() {
            CommandType::A => { address += 1 }
            CommandType::C => { address += 1 }
            CommandType::L => {
                let label = parser.symbol().unwrap();
                symbol_table.add_entry(label, address + 1)
            }
        }
    }
    parser.reset_cursor();

    while parser.has_more_commands() {
        match parser.command_type() {
            CommandType::A => {
                let num = parser.symbol().unwrap().parse::<i32>().unwrap();
                writeln!(&mut file, "{:016b}", num)?
            }
            CommandType::C => {
                let dest_code = code::dest(&parser.dest()).to_string();
                let comp_code = code::comp(&parser.comp()).to_string();
                let jump_code = code::jump(&parser.jump()).to_string();
                writeln!(&mut file, "{}{}{}{}", "111", comp_code, dest_code, jump_code)?
            }
            CommandType::L => {}
        }
    }
    Ok(())
}
