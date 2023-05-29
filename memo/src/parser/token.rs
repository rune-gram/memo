use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Tokens {
    // operation
    #[token("\"", priority = 1)]
    Quote,
    #[token("'", priority = 1)]
    Char,
    #[token("=", priority = 1)]
    Assign,
    #[token("+", priority = 1)]
    Add,
    #[token("-", priority = 1)]
    Subtract,
    #[token("*", priority = 1)]
    Multiply,
    #[token("/", priority = 1)]
    Divide,
    #[token("!=", priority = 1)]
    BangEqual,
    #[token("==", priority = 1)]
    Equals,

    // types
    #[token("catat", priority = 1)]
    VarStr,
    #[token("angka", priority = 1)]
    VarNum,

    // control flow
    #[token("tulisn", priority = 1)]
    PrintInline,

    // value
    #[regex(r#"[a-zA-Z_]"#, priority = 1)]
    Variable,
    #[regex(r#""(\\[\\"]|[^"])*""#, priority = 2)]
    String,
    #[regex(r"[0-9]+", priority = 1)]
    Number,

    // #[regex(r#"=\s*(betul)\s*;"#, priority = 3)]
    // BoolTrue,
    // #[token(r#"=\s*(salah)\s*;"#, priority = 3)]
    // BoolFalse,

    // line or file management
    #[token(";", priority = 1)]
    EOL
}

impl ToString for Tokens {
    fn to_string(&self) -> String {
        match self {
            Tokens::Quote => "Quote".to_string(),
            Tokens::Char => "Char".to_string(),
            Tokens::Assign => "Assign".to_string(),
            Tokens::Add => "Add".to_string(),
            Tokens::Subtract => "Subtract".to_string(),
            Tokens::Multiply => "Multiply".to_string(),
            Tokens::Divide => "Divide".to_string(),
            Tokens::BangEqual => "BangEqual".to_string(),
            Tokens::Equals => "Equals".to_string(),
            Tokens::VarStr => "TypeStr".to_string(),
            Tokens::VarNum => "TypeNum".to_string(),
            Tokens::PrintInline => "PrintInline".to_string(),
            Tokens::Variable => "Variable".to_string(),
            Tokens::String => "String".to_string(),
            Tokens::Number => "Number".to_string(),
            // Tokens::BoolTrue => "True".to_string(),
            // Tokens::BoolFalse => "False".to_string(),
            Tokens::EOL => "EOL".to_string()
        }
    }
}