mod analisador_lexico;
use crate::analisador_lexico::*;

fn main() {

    let mut lexer = lexer::Lexer::novo("teste.asd".into());
    lexer.analisar();
    let tokens = lexer.tokens();

    for i in tokens {
        println!("{}", i);
    }

}
