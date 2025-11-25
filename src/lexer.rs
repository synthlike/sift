#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Contract,
    Function,
    Public,
    External,
    Internal,
    Private,
    View,
    Pure,
    Payable,
    Returns,
    Memory,
    Calldata,
    Storage,
    Return,

    LeftParen,    // (
    RightParen,   // }
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Comma,        // ,
    Semicolon,    // ;

    Identifier(String),
    Number(String),

    Type(String), // uint256, address, etc.

    Eof,
    Unknown(char),
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current = chars.get(0).copied();

        Lexer {
            input: chars,
            position: 0,
            current,
        }
    }

    fn advance(&mut self) {
        self.position += 1;
        self.current = self.input.get(self.position).copied();
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.position + 1).copied()
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_line_comment(&mut self) {
        // skip '//'
        self.advance();
        self.advance();

        while let Some(ch) = self.current {
            if ch == '\n' {
                self.advance();
                break;
            }
            self.advance();
        }
    }

    fn skip_block_comment(&mut self) {
        // skip '/*'
        self.advance();
        self.advance();

        while let Some(ch) = self.current {
            if ch == '*' && self.peek() == Some('/') {
                self.advance();
                self.advance();
                break;
            }
            self.advance();
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();

        while let Some(ch) = self.current {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        identifier
    }

    fn read_number(&mut self) -> String {
        let mut number = String::new();

        while let Some(ch) = self.current {
            if ch.is_numeric() {
                number.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        number
    }

    pub fn next_token(&mut self) -> Token {
        loop {
            self.skip_whitespace();

            if self.current == Some('/') {
                match self.peek() {
                    Some('/') => self.skip_line_comment(),
                    Some('*') => self.skip_block_comment(),
                    _ => break,
                }
            } else {
                break;
            }
        }

        let ch = match self.current {
            Some(c) => c,
            None => return Token::Eof,
        };

        let token = match ch {
            '(' => {
                self.advance();
                Token::LeftParen
            }
            ')' => {
                self.advance();
                Token::RightParen
            }
            '{' => {
                self.advance();
                Token::LeftBrace
            }
            '}' => {
                self.advance();
                Token::RightBrace
            }
            '[' => {
                self.advance();
                Token::LeftBracket
            }
            ']' => {
                self.advance();
                Token::RightBracket
            }
            ',' => {
                self.advance();
                Token::Comma
            }
            ';' => {
                self.advance();
                Token::Semicolon
            }
            _ if ch.is_alphanumeric() || ch == '_' => {
                let ident = self.read_identifier();

                match ident.as_str() {
                    "contract" => Token::Contract,
                    "function" => Token::Function,
                    "external" => Token::External,
                    "public" => Token::Public,
                    "internal" => Token::Internal,
                    "private" => Token::Private,
                    "view" => Token::View,
                    "pure" => Token::Pure,
                    "payable" => Token::Payable,
                    "returns" => Token::Returns,
                    "memory" => Token::Memory,
                    "calldata" => Token::Calldata,
                    "storage" => Token::Storage,
                    "return" => Token::Return,

                    // types
                    s if s.starts_with("uint")
                        || s.starts_with("int")
                        || s.starts_with("bytes")
                        || s == "address"
                        || s == "bool"
                        || s == "string" =>
                    {
                        Token::Type(ident)
                    }

                    _ => Token::Identifier(ident),
                }
            }

            _ if ch.is_numeric() => Token::Number(self.read_number()),

            _ => {
                self.advance();
                Token::Unknown(ch)
            }
        };

        token
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token();

            if token == Token::Eof {
                tokens.push(token);
                break;
            }

            tokens.push(token);
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_function() {
        let input = "function foo() external {}";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            Vec::from([
                Token::Function,
                Token::Identifier("foo".to_string()),
                Token::LeftParen,
                Token::RightParen,
                Token::External,
                Token::LeftBrace,
                Token::RightBrace,
                Token::Eof,
            ])
        );
    }
    #[test]
    fn function_with_params() {
        let input = r#"
            function transfer(address to, uint256 amount) external returns bool {
                return true;
            }
        "#;
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            Vec::from([
                Token::Function,
                Token::Identifier("transfer".to_string()),
                Token::LeftParen,
                Token::Type("address".to_string()),
                Token::Identifier("to".to_string()),
                Token::Comma,
                Token::Type("uint256".to_string()),
                Token::Identifier("amount".to_string()),
                Token::RightParen,
                Token::External,
                Token::Returns,
                Token::Type("bool".to_string()),
                Token::LeftBrace,
                Token::Return,
                Token::Identifier("true".to_string()), // actually not an identifier but we don't care
                Token::Semicolon,
                Token::RightBrace,
                Token::Eof,
            ])
        );
    }

    #[test]
    fn comments() {
        let input = r#"
            // This is a comment
            function foo() external {}
            /* This is a
               multi-line comment */
        "#;
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            Vec::from([
                Token::Function,
                Token::Identifier("foo".to_string()),
                Token::LeftParen,
                Token::RightParen,
                Token::External,
                Token::LeftBrace,
                Token::RightBrace,
                Token::Eof,
            ])
        );
    }
}
