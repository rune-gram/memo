use super::token::Tokens;
use logos::Logos;

#[derive(Debug, Clone)]
pub struct Tval {
    token: Tokens,
    value: String,
}

pub struct Scans {
    pub content: String,
}

impl Scans {
    pub fn new(content: String) -> Scans {
        Scans { content: content }
    }

    pub fn start(&self) {
        let mut lex = Tokens::lexer(&self.content);
        let mut two_d_vector = Vec::new();
        let mut vector_inside = Vec::new();
        loop {
            let token = lex.next();
            if token == None {
                break;
            }
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
                                }
                                _ => {
                                    let vec_val = Tval {
                                        token: data,
                                        value: lex.slice().to_string(),
                                    };
                                    vector_inside.push(vec_val);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        let mut final_d_vector = Vec::new();
        // processing the variable
        // to combine multiple things into one.
        for vector in two_d_vector {
            let vtok_length = vector.len() - 1;
            let mut cloned_vec = vector.clone();
            let mut vtok_var = Vec::new();
            let mut var_position_first = 0usize;
            for (i, vtok) in vector.iter().enumerate() {
                // seek check next does it have a variable?
                let next_index = {
                    if i == vtok_length {
                        vtok_length
                    } else {
                        i + 1
                    }
                };
                if vtok.token == Tokens::Variable {
                    if vtok.token == Tokens::Variable {
                        if var_position_first == 0 {
                            var_position_first = i;
                        } else {
                            cloned_vec.remove(var_position_first+1);
                        }
                    }
                    vtok_var.push(vtok.value.clone());
                }
                if vector[next_index].token != Tokens::Variable {
                    break;
                }
            }
            // replace value on index
            cloned_vec[var_position_first].value = vtok_var.join("");
            // bulk remove from vec
            println!("\n\n");
            final_d_vector.push(cloned_vec);
        }

        println!("{:?}", final_d_vector);
    }
}
