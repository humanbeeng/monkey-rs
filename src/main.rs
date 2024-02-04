#[derive(Debug)]
enum TokenType {
    ILLEGAL,
    EOF,
    IDENT { literal: String },
    INT,
    // Operatos
    ASSIGN,
    PLUS,
    MINUS,
    // Delimiters
    COMMA,
    SEMICOLON,

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
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    literal: String,
}

struct Lexer<'a> {
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

    fn next_token(&mut self) -> Token {
        let mut tok = Token {
            token_type: TokenType::ILLEGAL,
            literal: String::new(),
        };

        self.skip_whitespaces();

        match self.ch {
            ',' => {
                tok.token_type = TokenType::COMMA;
            }
            '+' => {
                tok.token_type = TokenType::PLUS;
            }
            '-' => {
                tok.token_type = TokenType::MINUS;
            }
            ';' => {
                tok.token_type = TokenType::SEMICOLON;
            }
            '(' => {
                tok.token_type = TokenType::LPAREN;
            }
            ')' => {
                tok.token_type = TokenType::RPAREN;
            }
            '{' => {
                tok.token_type = TokenType::LBRACE;
            }
            '}' => {
                tok.token_type = TokenType::RBRACE;
            }
            '=' => {
                tok.token_type = TokenType::ASSIGN;
            }
            '0' => {
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
            _ => TokenType::IDENT {
                literal: identifier.to_string(),
            },
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

fn main() {
    let mut l = Lexer::new("    6");
    let tok = l.next_token();
    println!("{:?}", tok);
}
