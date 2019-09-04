#[macro_use]
extern crate lazy_static;

mod analisador_lexico;

fn main() {
    let tokens = analisador_lexico::lexer::analisar("teste.asd".into());

    for i in tokens {
        println!("tipo: {}, lexema: {}, literal: {}, linha: {}",i.token(), i.lexema(),  i.literal(), i.linha());
    }
}
