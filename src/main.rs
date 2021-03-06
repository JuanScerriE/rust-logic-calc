mod logic {
    use rand::{rngs::ThreadRng, Rng};
    use std::fmt;

    #[derive(PartialEq, PartialOrd, Eq, Debug, Copy, Clone)]
    pub enum Token {
        And,
        Or,
        Impls,
        BiImpls,
        Not,
        Var(u32),
        Bool(bool),
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
                Token::And => "^".to_string(),
                Token::Or => "v".to_string(),
                Token::Impls => "=>".to_string(),
                Token::BiImpls => "<=>".to_string(),
                Token::Not => "!".to_string(),

                Token::Var(index) => match index {
                    0 => "P".to_string(),
                    1 => "Q".to_string(),
                    2 => "R".to_string(),
                    3 => "S".to_string(),
                    4 => "T".to_string(),
                    index => format!("A{}", index),
                },

                Token::Bool(value) => value.to_string(),
            }
        }
    }

    pub struct Expression {
        var_count: u32,
        token_vec: Vec<Token>,
        expr_string: String,
    }

    impl fmt::Display for Expression {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self.token_vec)
        }
    }

    impl Expression {
        pub fn random(&mut self, rng: &mut ThreadRng, depth: u32) {
            let mut count: u32 = 1;
            let mut var_count: u32 = 0;
            let mut current_depth: u32 = 1;
            let mut depth_stack: Vec<u32> = Vec::new();

            while count != 0 {
                if current_depth == depth {
                    count -= 1;
                    current_depth = depth_stack.pop().unwrap_or(current_depth);

                    self.push_tok(Token::Var(var_count));

                    var_count += 1;
                } else {
                    match rng.gen_range(0..6) {
                        value if (value >= 0 && value <= 3) => {
                            // And, Or, Implies, Bi-implies
                            count += 1;
                            current_depth += 1;
                            depth_stack.push(current_depth);

                            match value {
                                0 => self.push_tok(Token::And),
                                1 => self.push_tok(Token::Or),
                                2 => self.push_tok(Token::Impls),
                                3 => self.push_tok(Token::BiImpls),
                                _ => (),
                            }
                        }

                        4 => {
                            // Not
                            count += 0;
                            current_depth += 1;

                            self.push_tok(Token::Not);
                        }

                        5 => {
                            // Variable
                            count -= 1;
                            current_depth = depth_stack.pop().unwrap_or(current_depth);

                            self.push_tok(Token::Var(var_count));

                            var_count += 1;
                        }

                        _ => (),
                    }
                }
            }

            self.var_count = var_count;
            self.token_vec.reverse();
        }

        pub fn gen_expr(&mut self, rng: &mut ThreadRng, depth: u32) {
            if depth == 0 {
                panic!("depth cannot be 0");
            }

            if depth != 1 {
                match rng.gen_range(0..6) {
                    0 => {
                        self.gen_expr(rng, depth - 1);
                        self.gen_expr(rng, depth - 1);
                        self.push_tok(Token::And);
                    }

                    1 => {
                        self.gen_expr(rng, depth - 1);
                        self.gen_expr(rng, depth - 1);
                        self.push_tok(Token::Or);
                    }

                    2 => {
                        self.gen_expr(rng, depth - 1);
                        self.gen_expr(rng, depth - 1);
                        self.push_tok(Token::Impls);
                    }

                    3 => {
                        self.gen_expr(rng, depth - 1);
                        self.gen_expr(rng, depth - 1);
                        self.push_tok(Token::BiImpls);
                    }

                    4 => {
                        self.gen_expr(rng, depth - 1);
                        self.push_tok(Token::Not);
                    }

                    5 => {
                        self.push_tok(Token::Var(rng.gen_range(0..self.var_count)));
                        // self.push_tok(Token::Var(depth));
                    }

                    _ => (),
                }
            } else {
                self.push_tok(Token::Var(rng.gen_range(0..self.var_count)));
                // self.push_tok(Token::Var(depth));
            }
        }

        fn push_tok(&mut self, token: Token) {
            self.token_vec.push(token);
        }

        pub fn truth_table(&mut self) {
            let mut result = Vec::new();

            let mut interpretation = vec![false; self.var_count as usize];

            let mut value_stack = Vec::new();

            for i in 0..(2_usize.pow(self.var_count)) {
                let mut token_vec = self.token_vec.clone();

                for j in (0..self.var_count).rev() {
                    interpretation[(self.var_count - j - 1) as usize] =
                        (i % 2_usize.pow(j + 1)) / 2_usize.pow(j) == 0;
                }

                for j in 0..self.token_vec.len() {
                    token_vec[j] = match self.token_vec[j] {
                        Token::Var(index) => Token::Bool(interpretation[index as usize]),
                        oper => oper, // assuming token is an operator
                    }
                }

                for token in token_vec {
                    let value: bool;

                    match token {
                        Token::Bool(boolean) => value = boolean,

                        Token::And => value = Expression::and(value_stack.pop(), value_stack.pop()),

                        Token::Or => value = Expression::or(value_stack.pop(), value_stack.pop()),

                        Token::Impls => {
                            value = Expression::impls(value_stack.pop(), value_stack.pop())
                        }

                        Token::BiImpls => {
                            value = Expression::bi_impls(value_stack.pop(), value_stack.pop());
                            // value_stack.push(Expression::bi_impls(value_stack.pop(), value_stack.pop()));
                        }

                        Token::Not => value = Expression::not(value_stack.pop()),

                        Token::Var(_) => panic!("variable is not allowed"),
                    }

                    value_stack.push(value);
                }

                result.push(match value_stack.pop() {
                    Some(value) => value,
                    None => panic!("no last element"),
                });
            }

            println!("{:#?}", result);
        }

        fn and(a: Option<bool>, b: Option<bool>) -> bool {
            match a {
                Some(a) => match b {
                    Some(b) => a & b,
                    None => panic!("invalid option"),
                },

                None => panic!("invalid option"),
            }
        }

        fn or(a: Option<bool>, b: Option<bool>) -> bool {
            match a {
                Some(a) => match b {
                    Some(b) => a | b,
                    None => panic!("invalid option"),
                },

                None => panic!("invalid option"),
            }
        }

        fn impls(a: Option<bool>, b: Option<bool>) -> bool {
            match a {
                Some(a) => match b {
                    Some(b) => !a | b,
                    None => panic!("invalid option"),
                },

                None => panic!("invalid option"),
            }
        }

        fn bi_impls(a: Option<bool>, b: Option<bool>) -> bool {
            match a {
                Some(a) => match b {
                    Some(b) => !(a ^ b),
                    None => panic!("invalid option"),
                },

                None => panic!("invalid option"),
            }
        }

        fn not(a: Option<bool>) -> bool {
            match a {
                Some(a) => !a,
                None => panic!("invalid option"),
            }
        }

        pub fn new() -> Expression {
            Expression {
                var_count: 0,
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

    let mut expr = Expression::new();
    // expr.gen_expr(&mut rand::thread_rng(), 3);
    expr.random(&mut rand::thread_rng(), 6);
    println!("{}", expr);
    expr.truth_table();
    println!("{}", expr);
}
