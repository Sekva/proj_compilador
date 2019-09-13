mod analisador_lexico;
use crate::analisador_lexico::*;

mod tabela_simbolos;
use crate::tabela_simbolos::*;

fn main() {

    let mut lexer = lexer::Lexer::novo("teste.asd".into());
    lexer.analisar();
    let tokens = lexer.tokens();

    for i in tokens {
        println!("{}", i);
    }

}
