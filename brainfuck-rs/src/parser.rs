use crate::tokenizer::Token;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub commands: Vec<Command>
}

#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    Increment,
    Decrement,
    MoveRight,
    MoveLeft,
    LoopStart(Vec<Command>),
    LoopEnd,
    Read,
    Write
}

pub struct Parser {}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser {
    
    pub fn new() -> Parser {
        Parser {}
    }

    pub fn parse_tokens(&mut self, tokens: Vec<Token>) -> Program {
        let mut commands = vec![];
        let mut iter = tokens.iter().peekable();
        
        while let Some(c) = iter.next() {
            let command = self.token_to_command(c);
            commands.push(command);
        }

        Program { commands }
    }

    fn token_to_command(&mut self, token: &Token) -> Command {
        match token {
            Token::Increment => Command::Increment,
            _ => panic!("Unknown token")
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_increment() {
        let tokens = vec![Token::Increment];
        let mut parser = Parser::new();

        let program = parser.parse_tokens(tokens);

        assert_eq!(program.commands[0], Command::Increment);
    }

}