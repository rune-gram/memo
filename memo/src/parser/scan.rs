use super::token::Tokens;
use logos::Logos;

pub struct Scans {
    pub content: String,
}

impl Scans {
    pub fn new(content: String) -> Scans {
        Scans {
            content: content,
        }
    }

    fn seek_next_token(){}

    pub fn start(&self) {
        let mut lex = Tokens::lexer(&self.content);
        let mut two_d_vector  = Vec::new();
        let mut vector_inside = Vec::new();
        loop {
            let token = lex.next();
            if token == None {break;}
            match token {
                Some(data) => {
                    match data {
                        Ok(data) => {
                            // check if tokens is EOL
                            // println!("{}", lex.slice());
                            match data {
                                Tokens::EOL => {
                                    two_d_vector.push(vector_inside);
                                    vector_inside = Vec::new();
                                    continue;
                                },
                                _ => {
                                    vector_inside.push(vec![data.to_string(), lex.slice().to_string()]);
                                }
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }
        // destroying vector_inside
        vector_inside = Vec::new();
        
        // processing the variable
        

        println!("{:?}", two_d_vector);
    }
}