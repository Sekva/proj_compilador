#[macro_use]
extern crate lazy_static;

mod analisador_lexico;
use analisador_lexico::*;


fn main() {
    let tokens = analisador_lexico::lexer::analisar("/home/sekva/dados/BCC/6P/compiladores/proj_compilador/compilador/teste.asd".into());

    for i in tokens {
        println!("tipo: {}, lexema: {}, literal: {}, linha: {}",i.token(), i.lexema(),  i.literal(), i.linha());
    }
}
