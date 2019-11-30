mod analisador_lexico;
use crate::analisador_lexico::*;

mod analisador_sintatico;
use crate::analisador_sintatico::*;

mod tabela_simbolos;

mod otimizador;

#[macro_use]
extern crate prettytable;

use std::env;

fn main() {
    println!("\n\n");

    let mut lexer = lexer::Lexer::novo("teste.asd".into());
    lexer.analisar();
    let tokens = lexer.tokens();

    // pra lembrar que fiz 5 parsers
    //let mut parser = parser5::Parser::novo(tokens.clone());

    let mut parser = parser::Parser::novo(tokens.clone());
    parser.iniciar_analise();

    for arg in env::args() {
        if arg == "-e" {
            parser.tabela_de_simbolos().printar();
        }
    }

    let mut otimim = 0;
    for arg in env::args() {
        if arg == "-O0" {
            otimim = 0;
        } else if arg == "-O1" {
            otimim = 1;
        } else if arg == "-O2" {
            otimim = 2;
        } else if arg == "-O3" {
            otimim = 3;
        }
    }

    let mut otimizador = otimizador::otimizador::Otimizador::novo(otimim, parser.programa());

    let programa = otimizador.otimizar();

    for arg in env::args() {
        if arg == "-p" {
            for i in programa.clone() {
                println!("{}", i);
            }
        }
    }
}
