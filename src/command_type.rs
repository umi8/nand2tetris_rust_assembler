#[derive(Debug, PartialEq)]
pub enum CommandType {
    A,
    C,
    L
}

impl CommandType {
    pub fn from(command: &String) -> CommandType {
        return if command.starts_with("@") {
            CommandType::A
        } else if command.starts_with("(") && command.ends_with(")") {
            CommandType::L
        } else {
            CommandType::C
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::CommandType;

    #[test]
    fn return_type_a() {
        assert_eq!(CommandType::A, CommandType::from(&String::from("@2")))
    }

    #[test]
    fn return_type_c() {
        assert_eq!(CommandType::C, CommandType::from(&String::from("M=D")))
    }

    #[test]
    fn return_type_l() {
        assert_eq!(CommandType::L, CommandType::from(&String::from("(LABEL)")))
    }
}
