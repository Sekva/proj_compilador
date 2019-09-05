use std::string::String;
use std::vec::Vec;
use std::fs::File;
use std::io::Read;

use std::collections::HashMap;

use crate::analisador_lexico::token::*;
use crate::analisador_lexico::tipo_token::*;

fn carregar_fonte(caminho : String) -> Vec<u8> {
    let mut buffer = Vec::new();
    let mut file = File::open(caminho).expect("Codigo fonte não encontrado");
    file.read_to_end(&mut buffer).expect("Incapaz de ler arquivo");

    buffer
}

fn alpha_numerico_underline(c : char) -> bool {
    return alpha_underline(c) || c.is_numeric();
}

fn alpha_underline(c : char) -> bool {
    return (c >= 'a' && c <= 'z') ||
           (c >= 'A' && c <= 'Z') ||
            c == '_';
}


pub struct Lexer {
    comeco : usize,
    char_atual : usize,
    linha : usize,
    fonte : Vec<u8>,
    reservadas : HashMap<String, Tipo_Token>,
    lista_tokens : Vec<Token>,
}

impl Lexer {


    pub fn novo(caminho : String) -> Lexer {

        let mut mapa = HashMap::new();
        mapa.insert("if".to_string(), Tipo_Token::IF);
        mapa.insert("else".to_string(), Tipo_Token::ELSE);
        mapa.insert("while".to_string(), Tipo_Token::WHILE);
        mapa.insert("returns".to_string(), Tipo_Token::RETURNS);
        mapa.insert("return".to_string(), Tipo_Token::RETURN);
        mapa.insert("as".to_string(), Tipo_Token::AS);
        mapa.insert("func".to_string(), Tipo_Token::FUNC);
        mapa.insert("bool".to_string(), Tipo_Token::ID_BOOL);
        mapa.insert("char".to_string(), Tipo_Token::ID_CHAR);
        mapa.insert("float".to_string(), Tipo_Token::ID_FLOAT);
        mapa.insert("int".to_string(), Tipo_Token::ID_INT);
        mapa.insert("str".to_string(), Tipo_Token::ID_STR);
        mapa.insert("void".to_string(), Tipo_Token::ID_VOID);
        mapa.insert("break".to_string(), Tipo_Token::BREAK);
        mapa.insert("continue".to_string(), Tipo_Token::CONTINUE);
        mapa.insert("printk".to_string(), Tipo_Token::PRINTK);
        mapa.insert("true".to_string(), Tipo_Token::TRUE);
        mapa.insert("false".to_string(), Tipo_Token::FALSE);

        Lexer {
            comeco : 0,
            char_atual : 0,
            linha : 1,
            fonte : carregar_fonte(caminho),
            reservadas : mapa,
            lista_tokens : Vec::new(),
        }
    }

    pub fn tokens(&self) -> &Vec<Token> { &self.lista_tokens }

    pub fn analisar(&mut self) {
        while !(self.char_atual >= self.fonte.len()) {
            self.comeco = self.char_atual;
            self.scan_token();
        }
        self.lista_tokens.push(Token::novo(Tipo_Token::EOF, "".into(), "".into(), self.linha));
    }

    fn scan_token(&mut self) {
        let c = self.proximo();

        match c {
            '(' => self.lista_tokens.push(self.montar_token(Tipo_Token::PARENTESE_ESQUERDO)),
            ')' => self.lista_tokens.push(self.montar_token(Tipo_Token::PARENTESE_DIREITO)),
            '{' => self.lista_tokens.push(self.montar_token(Tipo_Token::CHAVE_ESQUERDA)),
            '}' => self.lista_tokens.push(self.montar_token(Tipo_Token::CHAVE_DIREITA)),
            ',' => self.lista_tokens.push(self.montar_token(Tipo_Token::VIRGULA)),
            ';' => self.lista_tokens.push(self.montar_token(Tipo_Token::PONTO_VIRGULA)),
            '=' => {
                if self.v_prox('=') {
                    self.proximo();
                    self.lista_tokens.push(self.montar_token(Tipo_Token::SIMBOLO_D_IGUAL))
                } else {
                    self.lista_tokens.push(self.montar_token(Tipo_Token::SIMBOLO_IGUAL))
                }
            },
            '|' => {
                if self.v_prox('|') {
                    self.proximo();
                    self.lista_tokens.push(self.montar_token(Tipo_Token::SIMBOLO_D_OR))
                } else {
                    self.lista_tokens.push(self.montar_token(Tipo_Token::SIMBOLO_OR))
                }
            },
            '&' => {
                if self.v_prox('&') {
                    self.proximo();
                    self.lista_tokens.push(self.montar_token(Tipo_Token::SIMBOLO_D_AND));
                } else {
                    self.lista_tokens.push(self.montar_token(Tipo_Token::SIMBOLO_AND));
                }
            },
            '!' => {
                if self.v_prox('=') {
                    self.proximo();
                    self.lista_tokens.push(self.montar_token(Tipo_Token::SIMBOLO_D_DIFERENTE))
                } else {
                    self.lista_tokens.push(self.montar_token(Tipo_Token::SIMBOLO_NOT))
                }
            },
            '<' => {
                if self.v_prox('=') {
                    self.proximo();
                    self.lista_tokens.push(self.montar_token(Tipo_Token::SIMBOLO_MENOR_IGUAL_Q))
                } else {
                    self.lista_tokens.push(self.montar_token(Tipo_Token::SIMBOLO_MENOR_Q))
                }
            },
            '>' => {
                if self.v_prox('=') {
                    self.proximo();
                    self.lista_tokens.push(self.montar_token(Tipo_Token::SIMBOLO_MAIOR_IGUAL_Q))
                } else {
                    self.lista_tokens.push(self.montar_token(Tipo_Token::SIMBOLO_MAIOR_Q))
                }
            },
            '+' => self.lista_tokens.push(self.montar_token(Tipo_Token::SIMBOLO_MAIS)),
            '-' => self.lista_tokens.push(self.montar_token(Tipo_Token::SIMBOLO_MENOS)),
            '*' => self.lista_tokens.push(self.montar_token(Tipo_Token::SIMBOLO_MULTI)),
            '/' => self.lista_tokens.push(self.montar_token(Tipo_Token::SIMBOLO_DIV)),
            '%' => self.lista_tokens.push(self.montar_token(Tipo_Token::SIMBOLO_MOD)),
            '~' => self.lista_tokens.push(self.montar_token(Tipo_Token::SIMBOLO_BIT_NOT)),
            ' ' => {},
            '\n' => self.linha += 1,
            '\r' => {},
            '\t' => {},
            '"' => {
                let t = self.pegar_string();
                self.lista_tokens.push(t);
            },
            '\'' => {
                let t = self.pegar_char();
                self.lista_tokens.push(t)
            },
            _ => {
                if c.is_numeric() {
                    let t = self.pegar_numero();
                    self.lista_tokens.push(t);
                } else if alpha_numerico_underline(c) {
                    let t = self.pegar_id();
                    self.lista_tokens.push(t);
                } else {
                    panic!("?????????????");
                }
            },
        }
    }

    fn proximo(&mut self) -> char {
        self.char_atual += 1;
        self.fonte[self.char_atual - 1] as char
    }

    fn v_prox(&self, c : char) -> bool {
        self.peek() == c
    }

    fn peek(&self) -> char {
        if self.char_atual >= self.fonte.len() { return '\0' }
        return self.fonte[self.char_atual] as char;
    }

    fn peek_prox(&self) -> char {
        if (self.char_atual + 1) >= self.fonte.len() { return '\0' }
        return self.fonte[self.char_atual + 1] as char;
    }

    fn montar_token(&self, tipo : Tipo_Token) -> Token {
        self.montar_token_2(tipo, "".into())
    }

    fn montar_token_2(&self, tipo : Tipo_Token, literal : String) -> Token {
        Token::novo(tipo, String::from_utf8(self.fonte[self.comeco..self.char_atual].to_vec()).unwrap(), literal, self.linha)
    }

    fn pegar_string(&mut self) -> Token {
        while (self.peek() != '"') && !(self.char_atual >= self.fonte.len()) {
            if self.peek() == '\n' { self.linha += 1; }
            self.proximo();
        }
        if self.char_atual >= self.fonte.len() { panic!("string incompleta na linha {}", self.linha); }
        self.proximo();

        self.montar_token_2(Tipo_Token::STR, String::from_utf8(self.fonte[(self.comeco + 1)..(self.char_atual - 1)].to_vec()).unwrap())
    }

    fn pegar_char(&mut self) -> Token {

        if self.char_atual >= self.fonte.len() { panic!("string incompleta na linha {}", self.linha); }
        self.proximo();
        if self.char_atual >= self.fonte.len() { panic!("string incompleta na linha {}", self.linha); }
        self.proximo();

        self.montar_token_2(Tipo_Token::CHAR, String::from_utf8(self.fonte[(self.comeco + 1)..(self.char_atual - 1)].to_vec()).unwrap())

    }

    fn pegar_numero(&mut self) -> Token {

        let mut tipo = Tipo_Token::INT;
        let mut is_float = false;
        let mut is_hex = false;

        while (self.peek().is_numeric()) && !(self.char_atual >= self.fonte.len()) { self.proximo(); }
        if self.peek() == '.' && self.peek_prox().is_numeric() {
            self.proximo();
            while (self.peek().is_numeric()) && !(self.char_atual >= self.fonte.len()) { self.proximo(); }
            is_float = true;
            tipo = Tipo_Token::FLOAT;
        }

        if self.peek() == 'x' && self.peek_prox().is_ascii_hexdigit() {
            self.proximo();
            while (self.peek().is_numeric()) && !(self.char_atual >= self.fonte.len()) { self.proximo(); }
            tipo = Tipo_Token::HEX;
            is_hex = true;
        }

        if (self.fonte[self.comeco] as char == '0') && (!is_float) && (!is_hex) {
            tipo = Tipo_Token::OCTAL;
        }

        self.montar_token_2(tipo, String::from_utf8(self.fonte[self.comeco..self.char_atual].to_vec()).unwrap())
    }

    fn pegar_id(&mut self) -> Token {

        while alpha_numerico_underline(self.peek()) { self.proximo(); }
        let id = String::from_utf8(self.fonte[self.comeco..self.char_atual].to_vec()).unwrap();
        let mut tipo = Tipo_Token::ID;

        match self.reservadas.get(&id) {
            Some(t) => {
                println!("{} : {}", &id, t);
                match t {
                    Tipo_Token::IF => tipo = Tipo_Token::IF,
                    Tipo_Token::ELSE => tipo = Tipo_Token::ELSE,
                    Tipo_Token::WHILE => tipo = Tipo_Token::WHILE,
                    Tipo_Token::RETURNS => tipo = Tipo_Token::RETURNS,
                    Tipo_Token::RETURN => tipo = Tipo_Token::RETURN,
                    Tipo_Token::AS => tipo = Tipo_Token::AS,
                    Tipo_Token::FUNC => tipo = Tipo_Token::FUNC,
                    Tipo_Token::ID_BOOL => tipo = Tipo_Token::ID_BOOL,
                    Tipo_Token::ID_CHAR => tipo = Tipo_Token::ID_CHAR,
                    Tipo_Token::ID_FLOAT => tipo = Tipo_Token::ID_FLOAT,
                    Tipo_Token::ID_INT => tipo = Tipo_Token::ID_INT,
                    Tipo_Token::ID_STR => tipo = Tipo_Token::ID_STR,
                    Tipo_Token::ID_VOID => tipo = Tipo_Token::ID_VOID,
                    Tipo_Token::BREAK => tipo = Tipo_Token::BREAK,
                    Tipo_Token::CONTINUE => tipo = Tipo_Token::CONTINUE,
                    Tipo_Token::PRINTK => tipo = Tipo_Token::PRINTK,
                    Tipo_Token::TRUE => tipo = Tipo_Token::TRUE,
                    Tipo_Token::FALSE => tipo = Tipo_Token::FALSE,
                    _ => {}
                }
            },
            None => {}
        }

        self.montar_token(tipo)

    }

}
