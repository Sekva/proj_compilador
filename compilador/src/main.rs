mod analisador_lexico;
use crate::analisador_lexico::*;

mod analisador_sintatico;
use crate::analisador_sintatico::*;


mod tabela_simbolos;
use crate::tabela_simbolos::*;

fn main() {

    let mut lexer = lexer::Lexer::novo("teste.asd".into());
    lexer.analisar();
    let tokens = lexer.tokens();

    for i in tokens {
        println!("{}", i);
    }

    // pra lembrar que fiz 5 parsers
    //let mut parser = parser5::Parser::novo(tokens.clone());


    let mut parser = parser::Parser::novo(tokens.clone());
    parser.iniciar_analise();

}
