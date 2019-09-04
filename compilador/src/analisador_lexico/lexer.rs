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

struct Lexer {
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

    pub fn analisar() -> Vec<Token> {
    }

}


static mut COMECO : usize = 0;
static mut CHAR_ATUAL : usize = 0;
static mut LINHA: usize = 1;


lazy_static! {
    static ref RESERVADAS : HashMap<String, Tipo_Token> = {
        let mut reservadas = HashMap::new();
        reservadas.insert("if".to_string(), Tipo_Token::IF);
        reservadas.insert("else".to_string(), Tipo_Token::ELSE);
        reservadas.insert("while".to_string(), Tipo_Token::WHILE);
        reservadas.insert("returns".to_string(), Tipo_Token::RETURNS);
        reservadas.insert("return".to_string(), Tipo_Token::RETURN);
        reservadas.insert("as".to_string(), Tipo_Token::AS);
        reservadas.insert("func".to_string(), Tipo_Token::FUNC);
        reservadas.insert("bool".to_string(), Tipo_Token::ID_BOOL);
        reservadas.insert("char".to_string(), Tipo_Token::ID_CHAR);
        reservadas.insert("float".to_string(), Tipo_Token::ID_FLOAT);
        reservadas.insert("int".to_string(), Tipo_Token::ID_INT);
        reservadas.insert("str".to_string(), Tipo_Token::ID_STR);
        reservadas.insert("void".to_string(), Tipo_Token::ID_VOID);
        reservadas.insert("break".to_string(), Tipo_Token::BREAK);
        reservadas.insert("continue".to_string(), Tipo_Token::CONTINUE);
        reservadas.insert("printk".to_string(), Tipo_Token::PRINTK);
        reservadas.insert("true".to_string(), Tipo_Token::TRUE);
        reservadas.insert("false".to_string(), Tipo_Token::FALSE);
        return reservadas;
    };
}



pub fn analisar(caminho : String) -> Vec<Token> {
    let codigo_fonte = carregar_fonte(caminho);

    let mut lista_tokens = Vec::new();

    unsafe {

        while !(CHAR_ATUAL >= codigo_fonte.len()) {
            COMECO = CHAR_ATUAL;
            scan_token(& mut lista_tokens, &codigo_fonte);
        }

    lista_tokens.push(Token::novo(Tipo_Token::EOF, "".into(), "".into(), LINHA));

    }
    lista_tokens
}

fn scan_token(lista_tokens : &mut Vec<Token>, codigo_fonte : &Vec<u8>) {

    let c = proximo(codigo_fonte);

    match c {
        '(' => lista_tokens.push(montar_token(Tipo_Token::PARENTESE_ESQUERDO, codigo_fonte)),
        ')' => lista_tokens.push(montar_token(Tipo_Token::PARENTESE_DIREITO, codigo_fonte)),
        '{' => lista_tokens.push(montar_token(Tipo_Token::CHAVE_ESQUERDA, codigo_fonte)),
        '}' => lista_tokens.push(montar_token(Tipo_Token::CHAVE_DIREITA, codigo_fonte)),
        ',' => lista_tokens.push(montar_token(Tipo_Token::VIRGULA, codigo_fonte)),
        ';' => lista_tokens.push(montar_token(Tipo_Token::PONTO_VIRGULA, codigo_fonte)),
        '=' => {
            if v_prox('=', codigo_fonte) {
                proximo(codigo_fonte);
                lista_tokens.push(montar_token(Tipo_Token::SIMBOLO_D_IGUAL, codigo_fonte))
            } else {
                lista_tokens.push(montar_token(Tipo_Token::SIMBOLO_IGUAL, codigo_fonte))
            }
        },
        '|' => {
            if v_prox('|', codigo_fonte) {
                proximo(codigo_fonte);
                lista_tokens.push(montar_token(Tipo_Token::SIMBOLO_D_OR, codigo_fonte))
            } else {
                lista_tokens.push(montar_token(Tipo_Token::SIMBOLO_OR, codigo_fonte))
            }
        },
        '&' => {
            if v_prox('&', codigo_fonte) {
                proximo(codigo_fonte);
                lista_tokens.push(montar_token(Tipo_Token::SIMBOLO_D_AND, codigo_fonte))
            } else {
                lista_tokens.push(montar_token(Tipo_Token::SIMBOLO_AND, codigo_fonte))
            }
        },
        '!' => {
            if v_prox('=', codigo_fonte) {
                proximo(codigo_fonte);
                lista_tokens.push(montar_token(Tipo_Token::SIMBOLO_D_DIFERENTE, codigo_fonte))
            } else {
                lista_tokens.push(montar_token(Tipo_Token::SIMBOLO_NOT, codigo_fonte))
            }
        },
        '<' => {
            if v_prox('=', codigo_fonte) {
                proximo(codigo_fonte);
                lista_tokens.push(montar_token(Tipo_Token::SIMBOLO_MENOR_IGUAL_Q, codigo_fonte))
            } else {
                lista_tokens.push(montar_token(Tipo_Token::SIMBOLO_MENOR_Q, codigo_fonte))
            }
        },
        '>' => {
            if v_prox('=', codigo_fonte) {
                proximo(codigo_fonte);
                lista_tokens.push(montar_token(Tipo_Token::SIMBOLO_MAIOR_IGUAL_Q, codigo_fonte))
            } else {
                lista_tokens.push(montar_token(Tipo_Token::SIMBOLO_MAIOR_Q, codigo_fonte))
            }
        },
        '+' => lista_tokens.push(montar_token(Tipo_Token::SIMBOLO_MAIS, codigo_fonte)),
        '-' => lista_tokens.push(montar_token(Tipo_Token::SIMBOLO_MENOS, codigo_fonte)),
        '*' => lista_tokens.push(montar_token(Tipo_Token::SIMBOLO_MULTI, codigo_fonte)),
        '/' => lista_tokens.push(montar_token(Tipo_Token::SIMBOLO_DIV, codigo_fonte)),
        '%' => lista_tokens.push(montar_token(Tipo_Token::SIMBOLO_MOD, codigo_fonte)),
        '~' => lista_tokens.push(montar_token(Tipo_Token::SIMBOLO_BIT_NOT, codigo_fonte)),
        ' ' => {},
        '\n' => unsafe { LINHA += 1 },
        '\r' => {},
        '\t' => {},
        '"' => lista_tokens.push(pegar_string(codigo_fonte)),
        '\'' => lista_tokens.push(pegar_char(codigo_fonte)),
        _ => {
            if c.is_numeric() {
                lista_tokens.push(pegar_numero(codigo_fonte));
            } else if alpha_numerico_underline(c) {
                lista_tokens.push(pegar_id(codigo_fonte));
            } else {
                panic!("?????????????");
            }
        },
    }

}

fn proximo(codigo_fonte : &Vec<u8>) -> char {
    unsafe {
        CHAR_ATUAL += 1;

        codigo_fonte[CHAR_ATUAL - 1] as char
    }
}

fn montar_token(tipo : Tipo_Token, fonte : &Vec<u8>) -> Token {
    montar_token_2(tipo, "".into(), fonte)
}

fn montar_token_2(tipo : Tipo_Token, literal : String, fonte : &Vec<u8>) -> Token {
    unsafe {
        Token::novo(tipo, String::from_utf8(fonte[COMECO..CHAR_ATUAL].to_vec()).unwrap(), literal, LINHA)
    }
}

fn v_prox(c : char, fonte : &Vec<u8>) -> bool {

    peek(fonte) == c

}

fn peek(fonte : &Vec<u8>) -> char {
    unsafe {
        if CHAR_ATUAL >= fonte.len() { return '\0' }
        return fonte[CHAR_ATUAL] as char;
    }
}

fn peek_prox(fonte : &Vec<u8>) -> char {
    unsafe {
        if (CHAR_ATUAL + 1) >= fonte.len() { return '\0' }
        return fonte[CHAR_ATUAL + 1] as char;
    }
}

fn pegar_string(fonte : &Vec<u8>) -> Token {
    unsafe {
        while (peek(fonte) != '"') && !(CHAR_ATUAL >= fonte.len()) {
            if peek(fonte) == '\n' { LINHA += 1; }
            proximo(fonte);
        }
        if CHAR_ATUAL >= fonte.len() { panic!("string incompleta na linha {}", LINHA); }
        proximo(fonte);
        montar_token_2(Tipo_Token::STR, String::from_utf8(fonte[(COMECO + 1)..(CHAR_ATUAL - 1)].to_vec()).unwrap(), fonte)
    }
}

fn pegar_char(fonte : &Vec<u8>) -> Token {
    unsafe {
        if CHAR_ATUAL >= fonte.len() { panic!("string incompleta na linha {}", LINHA); }
        proximo(fonte);
        if CHAR_ATUAL >= fonte.len() { panic!("string incompleta na linha {}", LINHA); }
        proximo(fonte);

        montar_token_2(Tipo_Token::CHAR, String::from_utf8(fonte[(COMECO + 1)..(CHAR_ATUAL - 1)].to_vec()).unwrap(), fonte)
    }
}

fn alpha_numerico_underline(c : char) -> bool {
    return alpha_underline(c) || c.is_numeric();
}

fn alpha_underline(c : char) -> bool {
    return (c >= 'a' && c <= 'z') ||
           (c >= 'A' && c <= 'Z') ||
            c == '_';
}

fn pegar_id(fonte : &Vec<u8>) -> Token {
    unsafe {
        while alpha_numerico_underline(peek(fonte)) { proximo(fonte); }
        let id = String::from_utf8(fonte[COMECO..CHAR_ATUAL].to_vec()).unwrap();

        let mut tipo = Tipo_Token::ID;

        match RESERVADAS.get(&id) {
            Some(t) => {
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
                    _ => {}
                }
            },
            None => {}
        }


        montar_token(tipo, fonte)
    }
}

fn pegar_numero(fonte : &Vec<u8>) -> Token {
    unsafe {

        let mut tipo = Tipo_Token::INT;
        let mut is_float = false;
        let mut is_hex = false;

        while (peek(fonte).is_numeric()) && !(CHAR_ATUAL >= fonte.len()) { proximo(fonte); }
        if peek(fonte) == '.' && peek_prox(fonte).is_numeric() {
            proximo(fonte);
            while (peek(fonte).is_numeric()) && !(CHAR_ATUAL >= fonte.len()) { proximo(fonte); }
            is_float = true;
            tipo = Tipo_Token::FLOAT;
        }

        if peek(fonte) == 'x' && peek_prox(fonte).is_ascii_hexdigit() {
            proximo(fonte);
            while (peek(fonte).is_numeric()) && !(CHAR_ATUAL >= fonte.len()) { proximo(fonte); }
            tipo = Tipo_Token::HEX;
            is_hex = true;
        }

        if (fonte[COMECO] as char == '0') && (!is_float) && (!is_hex) {
            tipo = Tipo_Token::OCTAL;
        }

        montar_token_2(tipo, String::from_utf8(fonte[COMECO..CHAR_ATUAL].to_vec()).unwrap(), fonte)
    }
}
