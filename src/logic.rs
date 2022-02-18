mod logic {
    use rand::{rngs::ThreadRng, Rng};
    use std::fmt;

    const NO_VAR: u8 = 0;

    #[derive(PartialEq, PartialOrd, Eq, Debug)]
    pub enum Token {
        And,
        Or,
        Implies,
        BiImplies,
        Not,
        Var,
    }

    impl Type {
        fn from_u8(number: u8) -> Type {
            match number {
                0 => Type::And,
                1 => Type::Or,
                2 => Type::Implies,
                3 => Type::BiImplies,
                4 => Type::Not,
                5 => Type::Var,
                _ => panic!("unsigned integer cannot be converted into 'Type' enum"),
            }
        }
    }

    #[derive(Debug)]
    pub struct Token {
        m_type: Type,
        m_var: u8,
    }

    pub struct TokenError;

    impl fmt::Display for TokenError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "token is not a variable (display)")
        }
    }

    impl fmt::Debug for TokenError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "{}:{}:{}: token is not a variable (debug)",
                file!(),
                line!(),
                column!()
            )
        }
    }

    impl Token {
        pub fn new_var(var: u8) -> Token {
            Token {
                m_type: Type::Var,
                m_var: var,
            }
        }

        pub fn new_oper(token_type: Type) -> Token {
            Token {
                m_type: token_type,
                m_var: NO_VAR,
            }
        }

        pub fn is_oper(&self) -> bool {
            if self.m_type != Type::Var {
                true
            } else {
                false
            }
        }

        pub fn get_variable(&self) -> Result<u8, TokenError> {
            if self.m_type == Type::Var {
                Ok(self.m_var)
            } else {
                Err(TokenError)
            }
        }

        fn get_symbol(&self) -> String {
            match self.m_type {
                Type::And => "^".to_string(),
                Type::Or => "v".to_string(),
                Type::Implies => "=>".to_string(),
                Type::BiImplies => "<=>".to_string(),
                Type::Not => "!".to_string(),
                Type::Var => {
                    let variable = match self.get_variable() {
                        Ok(variable) => variable,
                        Err(error) => panic!("{}", error),
                    };
                    match variable {
                        1 => "P".to_string(),
                        2 => "Q".to_string(),
                        3 => "R".to_string(),
                        4 => "S".to_string(),
                        5 => "T".to_string(),
                        variable => format!("A{}", variable),
                    }
                }
                _ => panic!("invalid type"),
            }
        }
    }

    pub struct Expression {
        num_of_var: u8,
        token_vec: Vec<Token>,
        expr_string: String,
    }

    impl fmt::Display for Expression {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.expr_string)
        }
    }

    impl Expression {
        pub fn random_wrapper(&mut self, depth: u8) {
            self.random(&mut rand::thread_rng(), depth);
        }

        pub fn random(&mut self, rng: &mut ThreadRng, depth: u8) {
            if depth != 1 {
                match Type::from_u8(rng.gen_range(0..(Type::Total as u8))) {
                    Type::Var => {
                        self.push_tok(Token::new_var(rng.gen_range(1..=self.num_of_var)));
                    }

                    Type::Not => {
                        self.push_tok(Token::new_oper(Type::Not));
                        self.random(rng, depth - 1);
                    }

                    binary_type => {
                        self.push_tok(Token::new_oper(binary_type));
                        self.random(rng, depth - 1);
                        self.random(rng, depth - 1);
                    }
                }
            } else {
                self.push_tok(Token::new_var(rng.gen_range(1..=self.num_of_var)));
            }
        }

        fn push_tok(&mut self, token: Token) {
            self.token_vec.push(token);
        }

        pub fn new(num_of_var: u8) -> Expression {
            Expression {
                num_of_var,
                token_vec: Vec::<Token>::new(),
                expr_string: String::new(),
            }
        }
    }
}
