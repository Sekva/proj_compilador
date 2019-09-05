#[macro_use]
extern crate lazy_static;

mod analisador_lexico;
use crate::analisador_lexico::*;

fn main() {
    let mut lexer = lexer::Lexer::novo("teste.asd".into());
    lexer.analisar();
    let tokens2 = lexer.tokens();
}
