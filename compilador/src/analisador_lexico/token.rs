use crate::analisador_lexico::tipo_token::*;
use std::fmt;

#[derive(Clone)]
pub struct Token {
    pub token: Tipo_Token,
    pub lexema: String,
    pub literal: String,
    pub linha: usize,
    pub valor_bool: Option<bool>,
    pub valor_char: Option<char>,
    pub valor_str: Option<String>,
    pub valor_int: Option<i128>,
    pub valor_float: Option<f64>,
    pub symtab: u64,
}

impl Token {
    pub fn novo(tipo: Tipo_Token, lexema: String, literal: String, linha: usize) -> Token {
        let mut opt_bool: Option<bool> = None;
        let mut opt_char: Option<char> = None;
        let mut opt_str: Option<String> = None;
        let mut opt_int: Option<i128> = None;
        let mut opt_float: Option<f64> = None;

        {
            let temp_literal = literal.to_string();

            match tipo {
                Tipo_Token::TRUE => opt_bool = Some(true),
                Tipo_Token::FALSE => opt_bool = Some(false),
                Tipo_Token::CHAR => opt_char = Some(temp_literal.chars().next().unwrap()),
                Tipo_Token::STR => opt_str = Some(temp_literal),
                Tipo_Token::INT => opt_int = Some(temp_literal.parse::<i128>().unwrap()),
                Tipo_Token::FLOAT => opt_float = Some(temp_literal.parse::<f64>().unwrap()),

                Tipo_Token::HEX => {
                    opt_int = Some(
                        i128::from_str_radix(temp_literal.trim_start_matches("0x"), 16).unwrap(),
                    )
                }
                Tipo_Token::OCTAL => {
                    opt_int = Some(
                        i128::from_str_radix(temp_literal.trim_start_matches("0x"), 16).unwrap(),
                    )
                }

                _ => {}
            }
        }

        Token {
            token: tipo,
            lexema: lexema,
            literal: literal,
            linha: linha,
            valor_bool: opt_bool,
            valor_char: opt_char,
            valor_str: opt_str,
            valor_int: opt_int,
            valor_float: opt_float,
            symtab: 0,
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

    pub fn valor_bool(&self) -> Option<bool> {
        self.valor_bool
    }
    pub fn valor_char(&self) -> Option<char> {
        self.valor_char
    }
    pub fn valor_str(&self) -> Option<String> {
        self.valor_str.clone()
    }
    pub fn valor_int(&self) -> Option<i128> {
        self.valor_int
    }
    pub fn valor_float(&self) -> Option<f64> {
        self.valor_float
    }

    //TODO: dar uso ou apagar
    pub fn _symtab(&self) -> u64 {
        self.symtab
    }
    pub fn set_symtab(&mut self, entrada: u64) {
        self.symtab = entrada;
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.token == other.token
            && self.lexema == other.lexema
            && self.literal == other.literal
            && self.linha == other.linha
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Tipo : {}; Lexema: {}; Literal: {}; Linha: {}",
            self.token(),
            self.lexema(),
            self.literal(),
            self.linha()
        )
    }
}
