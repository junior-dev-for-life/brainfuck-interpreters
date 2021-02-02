#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Increment,
    Decrement,
    MoveLeft,
    MoveRight,
    Read,
    Write,
    LoopStart,
    LoopEnd,
    Nop,
    EOF
}

pub fn get_tokens(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    let mut char_iter = input.chars().peekable();

    while let Some(c) = char_iter.next() {
        match c {
            '>' => tokens.push(Token::MoveRight),
            '<' => tokens.push(Token::MoveLeft),
            '+' => tokens.push(Token::Increment),
            '-' => tokens.push(Token::Decrement),
            ',' => tokens.push(Token::Read),
            '.' => tokens.push(Token::Write),
            '[' => tokens.push(Token::LoopStart),
            ']' => tokens.push(Token::LoopEnd),
            _ => ()
        }
    }

    tokens.push(Token::EOF);

    return tokens;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_move_right() {
        let tokens = get_tokens(">");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], Token::MoveRight);
    }

    #[test]
    fn detect_move_left() {
        let tokens = get_tokens("<");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], Token::MoveLeft);
    }

    #[test]
    fn detect_increment() {
        let tokens = get_tokens("+");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], Token::Increment);
    }

    #[test]
    fn detect_decrement() {
        let tokens = get_tokens("-");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], Token::Decrement);
    }

    #[test]
    fn detect_read() {
        let tokens = get_tokens(",");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], Token::Read);
    }

    #[test]
    fn detect_write() {
        let tokens = get_tokens(".");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], Token::Write);
    }

    #[test]
    fn detect_loop_start() {
        let tokens = get_tokens("[");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], Token::LoopStart);
    }

    #[test]
    fn detect_loop_end() {
        let tokens = get_tokens("]");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], Token::LoopEnd);
    }

    #[test]
    fn ignore_whitespaces() {
        let tokens = get_tokens("\n +\t-");
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::Increment);
        assert_eq!(tokens[1], Token::Decrement);
    }
}