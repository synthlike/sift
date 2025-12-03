use crate::{
    ast::{DataLocation, Function, Mutability, Parameter, Type, Visibility},
    lexer::Token,
};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if self.current() == &expected {
            self.advance();
            Ok(())
        } else {
            Err(format!(
                "unexpected token: {:?}, expected: {:?}",
                self.current(),
                expected
            ))
        }
    }

    pub fn parse_function(&mut self) -> Result<Function, String> {
        self.expect(Token::Function)?;

        let name = match self.current() {
            Token::Identifier(n) => {
                let name = n.clone();
                self.advance();
                name
            }
            _ => return Err(format!("unexpected identifier: {:?}", self.current())),
        };

        self.expect(Token::LeftParen)?;
        let parameters = self.parse_parameter_list()?;
        self.expect(Token::RightParen)?;

        let visibility = self.parse_visibility()?;
        let mutability = self.parse_state_mutability();
        let returns = if self.current() == &Token::Returns {
            self.advance();
            self.expect(Token::LeftParen)?;
            let ret = self.parse_parameter_list()?;
            self.expect(Token::RightParen)?;
            Some(ret)
        } else {
            None
        };

        Ok(Function {
            name,
            parameters,
            visibility,
            mutability,
            returns,
        })
    }

    fn parse_parameter_list(&mut self) -> Result<Vec<Parameter>, String> {
        let mut params = Vec::new();

        if self.current() == &Token::RightParen {
            return Ok(params);
        }

        loop {
            let param = self.parse_parameter()?;
            params.push(param);

            if self.current() == &Token::Comma {
                self.advance();
            } else {
                break;
            }
        }

        Ok(params)
    }

    fn parse_parameter(&mut self) -> Result<Parameter, String> {
        let type_name = self.parse_type()?;

        let data_location = match self.current() {
            Token::Memory => {
                self.advance();
                Some(DataLocation::Memory)
            }
            Token::Calldata => {
                self.advance();
                Some(DataLocation::Calldata)
            }
            Token::Storage => {
                self.advance();
                Some(DataLocation::Storage)
            }
            _ => None,
        };

        let name = match self.current() {
            Token::Identifier(n) => {
                let name = n.clone();
                self.advance();
                Some(name)
            }
            _ => None,
        };

        Ok(Parameter {
            type_name,
            name,
            data_location,
        })
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        let base_type = match self.current() {
            Token::Type(t) => {
                let type_name = t.clone();
                self.advance();
                Type::Elementary(type_name)
            }
            Token::Identifier(t) => {
                let type_name = t.clone();
                self.advance();
                Type::Elementary(type_name)
            }
            _ => return Err("expected type".to_string()),
        };

        // array?
        if self.current() == &Token::LeftBracket {
            self.advance();

            if let Token::Number(size_str) = self.current() {
                let size: usize = size_str
                    .parse()
                    .map_err(|_| "invalid array size".to_string())?;
                self.advance();
                self.expect(Token::RightBracket)?;
                Ok(Type::FixedArray(Box::new(base_type), size))
            } else if self.current() == &Token::RightBracket {
                // dynamic array
                self.advance();
                Ok(Type::Array(Box::new(base_type)))
            } else {
                println!("current: {:?}", self.current());
                Err("invalid array syntax".to_string())
            }
        } else {
            Ok(base_type)
        }
    }

    fn parse_visibility(&mut self) -> Result<Visibility, String> {
        let vis = match self.current() {
            Token::External => {
                self.advance();
                Visibility::External
            }
            Token::Public => {
                self.advance();
                Visibility::Public
            }
            Token::Internal => {
                self.advance();
                Visibility::Internal
            }
            Token::Private => {
                self.advance();
                Visibility::Private
            }
            _ => Visibility::Public,
        };

        Ok(vis)
    }

    fn parse_state_mutability(&mut self) -> Option<Mutability> {
        match self.current() {
            Token::Pure => {
                self.advance();
                Some(Mutability::Pure)
            }
            Token::View => {
                self.advance();
                Some(Mutability::View)
            }
            Token::Payable => {
                self.advance();
                Some(Mutability::Payable)
            }
            _ => None,
        }
    }

    pub fn parse_all_functions(&mut self) -> Vec<Function> {
        let mut functions = Vec::new();

        while self.current() != &Token::Eof {
            if self.current() == &Token::Function {
                match self.parse_function() {
                    Ok(func) => functions.push(func),
                    Err(_) => {
                        // skip to next func on err
                        self.advance();
                    }
                }
            } else {
                self.advance();
            }
        }

        functions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn simple() {
        let input = "function foo() external {}";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let func = parser.parse_function().unwrap();

        assert_eq!(func.name, "foo");
        assert_eq!(func.parameters.len(), 0);
        assert_eq!(func.visibility, Visibility::External);
        assert_eq!(func.signature(), "foo()");
    }

    #[test]
    fn parameters() {
        let input = "function transfer(address to, uint256 amount) external";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let func = parser.parse_function().unwrap();

        assert_eq!(func.name, "transfer");
        assert_eq!(func.parameters.len(), 2);
        assert_eq!(func.parameters[0].type_name.canonical(), "address");
        assert_eq!(func.parameters[1].type_name.canonical(), "uint256");
        assert_eq!(func.signature(), "transfer(address,uint256)");
    }

    #[test]
    fn canonical() {
        let input = "function foo(uint x) external";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let func = parser.parse_function().unwrap();

        // uint -> uint256
        assert_eq!(func.signature(), "foo(uint256)");
    }

    #[test]
    fn arrays() {
        let input = "function bar(uint256[] memory arr, uint256[3] memory fixed) external";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let func = parser.parse_function().unwrap();

        assert_eq!(func.signature(), "bar(uint256[],uint256[3])");
    }
}
