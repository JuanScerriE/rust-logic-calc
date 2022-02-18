mod logic {
    use rand::{rngs::ThreadRng, Rng};
    use std::fmt;

    #[derive(PartialEq, PartialOrd, Eq, Debug)]
    pub enum Token {
        And,
        Or,
        Impl,
        BiImpl,
        Not,
        Var(u32),
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
        pub fn is_opr(&self) -> bool {
            match *self {
                Token::Var(_) => false,
                _ => true,
            }
        }

        pub fn is_var(&self) -> bool {
            match *self {
                Token::Var(_) => true,
                _ => false,
            }
        }

        pub fn get_var_index(&self) -> Result<u32, TokenError> {
            match *self {
                Token::Var(index) => Ok(index),
                _ => Err(TokenError),
            }
        }

        fn get_symbol(&self) -> String {
            match *self {
                Token::And => String::from("^"),
                Token::Or => String::from("v"),
                Token::Impl => String::from("=>"),
                Token::BiImpl => String::from("<=>"),
                Token::Not => String::from("!"),

                Token::Var(index) => match index {
                    0 => String::from("P"),
                    1 => String::from("Q"),
                    2 => String::from("R"),
                    3 => String::from("S"),
                    4 => String::from("T"),
                    index => format!("A{}", index),
                },
            }
        }
    }

    pub struct Expression {
        num_of_var: u32,
        token_vec: Vec<Token>,
        expr_string: String,
    }

    impl fmt::Display for Expression {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.expr_string)
        }
    }

    impl Expression {
        pub fn gen_expr(&mut self, rng: &mut ThreadRng, depth: u8) {
            if depth == 0 {
                panic!("depth cannot be 0");
            }

            if depth != 1 {
                match rng.gen_range(0..6) {
                    0 => {
                        self.push_tok(Token::And);
                        self.gen_expr(rng, depth - 1);
                        self.gen_expr(rng, depth - 1);
                    }

                    1 => {
                        self.push_tok(Token::Or);
                        self.gen_expr(rng, depth - 1);
                        self.gen_expr(rng, depth - 1);
                    }

                    2 => {
                        self.push_tok(Token::Impl);
                        self.gen_expr(rng, depth - 1);
                        self.gen_expr(rng, depth - 1);
                    }

                    3 => {
                        self.push_tok(Token::BiImpl);
                        self.gen_expr(rng, depth - 1);
                        self.gen_expr(rng, depth - 1);
                    }

                    4 => {
                        self.push_tok(Token::Not);
                        self.gen_expr(rng, depth - 1);
                    }

                    5 => {
                        self.push_tok(Token::Var(rng.gen_range(0..self.num_of_var)));
                    }

                    _ => panic!("invalid random number"),
                }
            } else {
                self.push_tok(Token::Var(rng.gen_range(0..self.num_of_var)));
            }
        }

        fn push_tok(&mut self, token: Token) {
            self.token_vec.push(token);
        }


        pub fn truth_values(&self) {
                        
        }

        pub fn new(num_of_var: u32) -> Expression {
            Expression {
                num_of_var,
                token_vec: Vec::<Token>::new(),
                expr_string: String::new(),
            }
        }
    }
}

use logic::Expression;
// use logic::Token;
// use logic::Type;

fn main() {
    // let token: Token = Token::new_oper(Type::Or);

    // let variable = match token.get_variable() {
    //     Ok(variable) => variable,
    //     Err(error) => panic!("{:?}", error),
    // };

    // println!("{}", variable);

    let mut expr = Expression::new(3);
    expr.gen_expr(&mut rand::thread_rng(), 3);
    println!("{}", expr);
}
