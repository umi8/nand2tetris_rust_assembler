pub enum CommandType {
    A,
    C,
}

impl CommandType {
    pub fn from(command: &String) -> CommandType {
        return if command.starts_with("@") {
            CommandType::A
        } else {
            CommandType::C
        }
    }
}
