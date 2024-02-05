#[derive(Debug, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    IDENT,
    INT,

    // Operatos
    ASSIGN,
    EQ,
    NOT,
    NOTEQ,
    PLUS,
    MINUS,
    GT,
    LT,

    // Delimiters
    COMMA,
    SEMICOLON,
    ASTERISK,
    SLASH,

    // Braces
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
    IF,
    ELSE,
    TRUE,
    FALSE,
    RETURN,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,      // corresponds to ch position
    read_position: usize, // next char in input
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut l = Lexer {
            input,
            position: 0,
            ch: 0 as char,
            read_position: 0,
        };
        l.read_char();
        return l;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0 as char;
        } else {
            if let Some(c) = self.input.chars().nth(self.read_position) {
                self.ch = c;
            } else {
                panic!("Index {} out of range", self.read_position);
            }
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespaces();
        let mut tok = Token {
            token_type: TokenType::ILLEGAL,
            literal: self.ch.to_string(),
        };

        match self.ch {
            ',' => tok.token_type = TokenType::COMMA,

            '>' => tok.token_type = TokenType::GT,

            '<' => tok.token_type = TokenType::LT,

            '+' => tok.token_type = TokenType::PLUS,

            '-' => tok.token_type = TokenType::MINUS,

            ';' => tok.token_type = TokenType::SEMICOLON,

            '*' => tok.token_type = TokenType::ASTERISK,

            '(' => tok.token_type = TokenType::LPAREN,

            ')' => tok.token_type = TokenType::RPAREN,

            '{' => tok.token_type = TokenType::LBRACE,

            '}' => tok.token_type = TokenType::RBRACE,

            '/' => tok.token_type = TokenType::SLASH,

            '=' => {
                if let Some(next_char) = self.input.chars().nth(self.read_position) {
                    if next_char == '=' {
                        tok.token_type = TokenType::EQ;
                        tok.literal = "==".to_string();
                        self.read_char();
                    } else {
                        tok.token_type = TokenType::ASSIGN;
                        tok.literal = '='.to_string();
                    }
                }
            }
            '!' => {
                if let Some(next_char) = self.input.chars().nth(self.read_position) {
                    if next_char == '=' {
                        tok.token_type = TokenType::NOTEQ;
                        tok.literal = "!=".to_string();
                        self.read_char();
                    } else {
                        tok.token_type = TokenType::NOT;
                        tok.literal = self.ch.to_string();
                    }
                }
            }
            '\0' => {
                tok.token_type = TokenType::EOF;
                tok.literal = "".to_string();
            }

            _ => {
                if self.is_letter(self.ch) {
                    tok.literal = self.read_identifier();
                    tok.token_type = self.lookup_identifier(&tok.literal);
                } else if self.ch.is_ascii_digit() {
                    tok.literal = self.ch.to_string();
                    tok.token_type = TokenType::INT;
                }
            }
        };
        self.read_char();

        return tok;
    }

    fn is_letter(&self, c: char) -> bool {
        return c.is_ascii_alphabetic() || c == '_';
    }

    fn lookup_identifier(&self, identifier: &str) -> TokenType {
        match identifier {
            "fn" => TokenType::FUNCTION,
            "let" => TokenType::LET,
            "if" => TokenType::IF,
            "else" => TokenType::ELSE,
            "return" => TokenType::RETURN,
            "true" => TokenType::TRUE,
            "false" => TokenType::FALSE,
            _ => TokenType::IDENT,
        }
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position;
        while self.is_letter(self.ch) {
            self.read_char();
        }
        self.input[pos..self.position].to_string()
    }

    fn skip_whitespaces(&mut self) {
        while self.ch.is_ascii_whitespace() || self.ch == '\t' || self.ch == '\n' || self.ch == '\r'
        {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod lexer_tests {
    use crate::lexer::{Lexer, Token, TokenType};
    #[test]
    fn test_ident() {
        let mut l = Lexer::new("Hello");
        let next_token = l.next_token();
        assert_eq!(
            next_token,
            Token {
                token_type: TokenType::IDENT,
                literal: "Hello".to_string()
            }
        );
    }

    #[test]
    fn test_keywords() {
        let mut l = Lexer::new("!=");
        let next_token = l.next_token();
        assert_eq!(
            next_token,
            Token {
                token_type: TokenType::NOTEQ,
                literal: "!=".to_string(),
            }
        )
    }
}
