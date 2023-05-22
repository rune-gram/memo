use super::token::Tokens;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    T(Tokens), // tokens
    C(u8), // chars
    CS(u8), // char string
    CN(u8), // char number
}

#[derive(Debug, Clone, PartialEq)]
pub struct Scanner {
    source: Vec<u8>,
    tokens : Vec<Token>,
    word: Vec<u8>,
    wordlast: u8,
    stringguard: bool,
    charguard: bool,
    start: usize,
    current: usize,
    line: usize,
}

// to determine operation for grouping the Char String/Number
const OPERATORS:[u8;5] = [
    61,126,60,62,59
];

impl Scanner {
    pub fn new(source: Vec<u8>) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            word: Vec::new(),
            wordlast: 00,
            stringguard: false,
            charguard: false,
            start: 0,
            current: 0,
            line: 1,
        }
    }
    
    pub fn advance(&mut self) -> u8 {
        self.current += 1;
        self.source[self.current-1]
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_token_tmp(&mut self, token: Tokens) {
        self.tokens.push(Token::T(token));
    }

    fn add_token_char(&mut self, c: u8) {
        self.tokens.push(Token::C(c));
    }

    fn forward_peek(&mut self) -> Option<u8> {
        // get next array, if not exist return None
        if self.current < self.source.len() - 1 {
            Some(self.source[self.current + 1])
        } else {
            None
        }
    }

    pub fn scan_token(&mut self){
        let c = self.advance();
        self.wordlast = c;
        match (c,self.stringguard) {
            (b'"',false|true) => {
                self.stringguard=!self.stringguard;
                self.add_token_tmp(Tokens::Petik);
            },
            (b'\'',false|true) => {
                self.stringguard=!self.stringguard;
                self.add_token_tmp(Tokens::Petik);
            },
            (b'<',false) => {
                let next_char = self.forward_peek();
                match next_char {
                    Some(b'=') => {
                        self.advance();
                        self.add_token_tmp(Tokens::KuranSama);
                    },
                    _ => {
                        self.add_token_tmp(Tokens::Kurang);
                    }
                }
            },
            (b'>',false) => {
                let next_char = self.forward_peek();
                match next_char {
                    Some(b'=') => {
                        self.advance();
                        self.add_token_tmp(Tokens::LebihSama);
                    },
                    _ => {
                        self.add_token_tmp(Tokens::Lebih);
                    }
                }
            },
            (b'=',false) => {
                let next_char = self.forward_peek();
                match next_char {
                    Some(b'=') => {
                        self.advance();
                        self.add_token_tmp(Tokens::Sama);
                    },
                    _ => {
                        self.add_token_tmp(Tokens::SamaDengan);
                    }
                }
            },
            (b'/', false) => {
                let next_char = self.forward_peek();
                match next_char {
                    Some(32) => { //< this is slash! idk but b'/' seems not working, but number works.
                        self.advance();
                        // problem :  this still make one line at the end of the comment
                        // visible to next scanner, this only occured when there is comment
                        // if there is no comment, everything fine.
                        while self.forward_peek() != Some(10) && !self.is_at_end() {
                            self.advance();
                        }
                    },
                    _ => {
                        self.add_token_tmp(Tokens::Bagi);
                    }
                }
            }
            (b'{',false) => {
                self.add_token_tmp(Tokens::KurawalBuka);
            }
            (b'}',false) => {
                self.add_token_tmp(Tokens::KurawalTutup);
            },
            (b'(',false) => {
                self.add_token_tmp(Tokens::KurungBuka);
            }
            (b')',false) => {
                self.add_token_tmp(Tokens::KurungTutup);
            }
            (b'~',false) => {
                self.add_token_tmp(Tokens::Pindah);
            },
            (b'!',false) => {
                self.add_token_tmp(Tokens::Seru);
            },
            (b'*',false) => {
                self.add_token_tmp(Tokens::Bintang);
            },
            (b'0'..=b'9',false) => {
                self.tokens.push(Token::CN(c));
            },
            (b' ',false) => {
                self.add_token_tmp(Tokens::Eow);
            },
            (b';',false) => {
                self.add_token_tmp(Tokens::Eol);
            },
            (10,false)=>{}, // new line
            _ => {
                if self.stringguard {
                    self.tokens.push(Token::CS(c));
                }else{
                    self.word.push(c);
                    self.add_token_char(c);
                }
            }
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.clone()
    }
}