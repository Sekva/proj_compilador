use std::cmp::Ordering::*;
use std::ops::Index;

use crate::analisador_lexico::token::*;
use crate::tabela_simbolos::escopo::*;

pub struct TabelaSimbolos {
    indice : u64,
    global : Escopo,

}

impl TabelaSimbolos {

    pub fn nova() -> TabelaSimbolos {
        TabelaSimbolos {
            indice : 0,
            global : Escopo::novo(),
        }
    }

    pub fn preencher(&mut self, tokens : Vec<Token>) {

    }

}
