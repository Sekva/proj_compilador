use crate::analisador_lexico::tipo_token::*;

pub struct Token {
    token : Tipo_Token,
    lexema : String,
    literal : String,
    linha : usize,
}

impl Token {

    pub fn novo(tipo : Tipo_Token, lexema : String, literal : String, linha : usize) -> Token {
        Token {
            token : tipo,
            lexema : lexema,
            literal : literal,
            linha : linha
        }
    }

    pub fn lexema(&self) -> String {
        self.lexema.to_string()
    }
    pub fn literal(&self) -> String {
        self.literal.to_string()
    }
    pub fn token(&self) -> Tipo_Token {
        self.token
    }
    pub fn linha(&self) -> usize {
        self.linha
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.token == other.token && self.lexema == other.lexema
    }
}

