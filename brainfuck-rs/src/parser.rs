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

pub struct Parser {
    tokens: Vec<Token>,
    curr_token: Token,
    curr_pos: usize
}

impl Parser {

    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            curr_token: Token::Nop,
            curr_pos: 0
        }
    }

    pub fn next_token(&mut self) {

    }

    pub fn parse_tokens(&mut self) -> Program {
        let mut commands = vec![];
        let total = self.tokens.len();

        while self.curr_token != Token::EOF {
            let command = self.token_to_command();
            commands.push(command);
            self.curr_pos = self.curr_pos + 1;
        }

        Program { commands }
    }

    fn token_to_command(&mut self) -> Command {
        match self.tokens[self.curr_pos] {
            Token::Increment => Command::Increment,
            Token::LoopStart => Command::Increment,
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
        let mut parser = Parser::new(tokens);

        let program = parser.parse_tokens();

        assert_eq!(program.commands[0], Command::Increment);
    }

    #[test]
    fn parse_non_nested_loops() {
        let tokens = vec![Token::LoopStart, Token::Increment, Token::LoopEnd];
        let mut parser = Parser::new(tokens);

        let program = parser.parse_tokens();

        let inner = vec![Command::Increment];

        assert_eq!(program.commands[0], Command::LoopStart(inner));
    }

}