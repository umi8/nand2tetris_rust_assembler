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

    let mut constant_address = 0;
    while parser.has_more_commands() {
        match parser.command_type() {
            CommandType::A => { constant_address += 1 }
            CommandType::C => { constant_address += 1 }
            CommandType::L => {
                let label = parser.symbol().unwrap();
                symbol_table.add_entry(label, constant_address)
            }
        }
    }
    parser.reset_cursor();

    let mut variable_address = 16;
    while parser.has_more_commands() {
        match parser.command_type() {
            CommandType::A => {
                let symbol = parser.symbol().unwrap();
                let num = match symbol.parse::<i32>() {
                    Ok(num) => num,
                    Err(_) => {
                        if symbol_table.contains(&symbol) {
                            symbol_table.get_address(&symbol).unwrap()
                        } else {
                            symbol_table.add_entry(symbol, variable_address);
                            let current_address = variable_address;
                            variable_address += 1;
                            current_address
                        }
                    }
                };
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
