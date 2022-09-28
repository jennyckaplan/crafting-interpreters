use crate::token::{Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.chars().count()
    }

    fn advance(&mut self) -> char {
        let char = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        char
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: String) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(
            token_type,
            text.to_string(),
            Some(literal),
            self.line,
        ))
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(token_type, text.to_string(), None, self.line))
    }

    fn is_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current) != Some(expected) {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source.chars().nth(self.current).unwrap()
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            println!("Unterminated string"); // TODO: custom error
            return;
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes
        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token_with_literal(TokenType::String, value.to_string());
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            // Consume the "."
            self.advance();
        }

        while self.peek().is_digit(10) {
            self.advance();
        }

        // TODO: should be passing in Double.parseDouble(source.substring(start, current))
        self.add_token_with_literal(
            TokenType::Number,
            self.source[self.start..self.current].to_string(),
        );
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        c.is_digit(10) || self.is_alpha(c)
    }

    fn get_token_type_from_keyword(&self, keyword: &str) -> Option<TokenType> {
        match keyword {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "fun" => Some(TokenType::Fun),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }

    fn identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = self
            .get_token_type_from_keyword(text)
            .unwrap_or(TokenType::Identifier);

        self.add_token(token_type.clone());
    }

    fn scan_token(&mut self) -> Result<(), ()> {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let token = if self.is_match('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token);
            }
            '=' => {
                let token = if self.is_match('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token);
            }
            '<' => {
                let token = if self.is_match('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token);
            }
            '>' => {
                let token = if self.is_match('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token);
            }
            '/' => {
                if self.is_match('/') {
                    // A comment goes until the end of the line
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => self.string(),
            'o' => {
                if self.is_match('r') {
                    self.add_token(TokenType::Or);
                }
            }
            _ => {
                if c.is_digit(10) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    println!("Unexpected character."); // TODO: custom error here
                }
            }
        }

        Ok(())
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, ()> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), None, self.line));

        Ok(self.tokens.clone())
    }
}
