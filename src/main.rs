use std::fs::File;
use std::io::Write;
use crate::command_type::CommandType;

mod code;
mod parser;
mod command_type;

fn main() -> std::io::Result<()> {
    let mut file = File::create("Prog.hack")?;

    let mut parser = match parser::parser::Parser::new("Prog.asm") {
        Ok(parser) => parser,
        Err(why) => panic!("couldn't parse: {}", why)
    };

    while parser.has_more_commands() {
        match parser.command_type() {
            CommandType::A => {
                let num = parser.symbol().parse::<i32>().unwrap();
                writeln!(&mut file, "{:016b}", num)?
            }
            CommandType::C => {
                let dest_code = code::dest(&parser.dest()).to_string();
                let comp_code = code::comp(&parser.comp()).to_string();
                let jump_code = code::jump(&parser.jump()).to_string();
                writeln!(&mut file, "{}{}{}{}", "111", comp_code, dest_code, jump_code)?
            }
        }
    }
    Ok(())
}
