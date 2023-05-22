use std::collections::BTreeMap;

use crate::parser::token::Tokens;

pub struct Mapper{
    f: BTreeMap<&'static str, Function>,
    v: BTreeMap<&'static str, Global>
}

pub struct Global {
    v: BTreeMap<&'static str, Vec<Tokens>>
}

pub struct Function {
    args: BTreeMap<&'static str, Vec<Tokens>>,
    opts: Option<Vec<Tokens>>,
}