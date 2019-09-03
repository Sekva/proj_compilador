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
}
