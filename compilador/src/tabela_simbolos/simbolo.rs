use crate::analisador_lexico::token::*;
use crate::tabela_simbolos::parametro::*;

#[derive(Copy,Clone,PartialEq)]
pub enum TipoSimbolo {
    Func,
    Var,
}


#[derive(Clone,PartialEq)]
pub struct Simbolo {

    nome : String,
    tipo : TipoSimbolo,
    atributo : String,
    valor : Option<Token>,
    parametros : Option<Parametros>,
    indice : u64,

}

impl Simbolo {

    pub fn nome      (&self) -> String             { self.nome.to_string() }
    pub fn tipo      (&self) -> TipoSimbolo        { self.tipo }
    pub fn atributo  (&self) -> String             { self.atributo.to_string() }
    pub fn parametros(&self) -> Option<Parametros> { self.parametros.clone() }
    pub fn indice    (&self) -> u64                { self.indice }

}


