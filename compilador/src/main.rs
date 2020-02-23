mod analisador_lexico;
use crate::analisador_lexico::*;

mod analisador_sintatico;
use crate::analisador_sintatico::*;

mod tabela_simbolos;

mod otimizador;

#[macro_use]
extern crate prettytable;

use prettytable::Table;
use std::env;

mod gerador_de_codigo;
use crate::gerador_de_codigo::Codigo;

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
            let mut table = Table::new();
            table.set_titles(row!["COD", "cod op", "tipo"]);
            for i in programa.clone() {
                if i.0 != "" {
                    table.add_row(row![i.0, i.1, i.2]);
                    //println!("{} --- {:?} --- {}\n", i.0, i.1, i.2);
                }
            }
            table.printstd();
        }
    }

    let mut gerador = Codigo::novo();
    let saida = gerador.gerar(programa.clone());

    println!("{}", saida);
}
