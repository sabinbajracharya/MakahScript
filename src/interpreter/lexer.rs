use super::token::{Token, TokenType};

pub struct Lexer {
    pub input: String,
    position: usize,
    read_position: usize,
    pub ch: u8,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut lexer = Lexer {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: 0,
        };

        lexer.read_char();

        return lexer;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let ch = self.ch.to_string();

        let token = match self.ch as char {
            '=' => Token::new(TokenType::ASSIGN, ch),
            ';' => Token::new(TokenType::SEMICOLON, ch),
            '(' => Token::new(TokenType::LPAREN, ch),
            ')' => Token::new(TokenType::RPAREN, ch),
            ',' => Token::new(TokenType::COMMA, ch),
            '+' => Token::new(TokenType::PLUS, ch),
            '{' => Token::new(TokenType::LBRACE, ch),
            '}' => Token::new(TokenType::RBRACE, ch),
            _ => Token::new(TokenType::EOF, "".to_string()),
        };

        self.read_char();

        return token;
    }
}
