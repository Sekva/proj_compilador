use crate::analisador_lexico::tipo_token::Tipo_Token;

#[derive(Clone, PartialEq)]
pub enum Simbolo {
    // Nome, tipo, linha declarada, escopo, entrada
    Var(String, Tipo_Token, usize, String, usize),

    //Nome, tipo retorno, num de parametros, lista de tipos dos parametros, linha declarada,
    //entrada
    Func(String, Tipo_Token, usize, Vec<Tipo_Token>, usize, usize),
}
