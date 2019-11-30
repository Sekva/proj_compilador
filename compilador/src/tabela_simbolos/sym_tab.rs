use crate::analisador_lexico::tipo_token::*;
use crate::tabela_simbolos::escopo::*;

use crate::tabela_simbolos::simbolo::Simbolo;

#[derive(Clone)]
pub struct TabelaSimbolos {
    global: ListaEscopo,
    indice_entrada: usize,
    indice_escopo: usize,
    escopo_atual: usize,
}

impl TabelaSimbolos {
    pub fn nova() -> TabelaSimbolos {
        TabelaSimbolos {
            global: ListaEscopo::nova(),
            indice_entrada: 1,
            indice_escopo: 1,
            escopo_atual: 0,
        }
    }

    pub fn abrir_escopo(&mut self) {
        let mut novo = Escopo::novo();
        novo.add_como_pai(self.escopo_atual);
        novo.set_escopo_num(self.indice_escopo);
        self.global.add_escopo(self.escopo_atual, novo);
        self.escopo_atual = self.indice_escopo;
        self.indice_escopo += 1;
    }

    pub fn fechar_escopo(&mut self) {
        self.escopo_atual = self.global.pai_de(self.escopo_atual);
    }

    pub fn add_simbolo_escopo_global(&mut self, s: Simbolo) -> usize {
        self.indice_entrada += 1;

        let novo_s: Simbolo;

        match s {
            Simbolo::Var(_, _, _, _, _) => {
                panic!("variavel como funcção indo pra tabela de simbolos");
            }
            Simbolo::Func(a, b, c, d, e, _f) => {
                novo_s = Simbolo::Func(a, b, c, d, e, self.indice_entrada - 1)
            }
        }

        self.global.add_simbolo_no_indice(0, novo_s);

        return self.indice_entrada - 1;
    }

    pub fn add_simbolo(&mut self, s: Simbolo) -> usize {
        if self.global.existe(s.clone(), self.escopo_atual) {
            panic!("simbolo já existe")
        }

        self.indice_entrada += 1;

        let novo_s: Simbolo;

        match s {
            Simbolo::Var(a, b, c, d, _e) => {
                novo_s = Simbolo::Var(a, b, c, d, self.indice_entrada - 1)
            }
            Simbolo::Func(a, b, c, d, e, _f) => {
                novo_s = Simbolo::Func(a, b, c, d, e, self.indice_entrada - 1)
            }
        }

        self.global.add_simbolo_no_indice(self.escopo_atual, novo_s);

        return self.indice_entrada - 1;
    }

    pub fn printar(&self) {
        self.global.printar();
    }

    pub fn lookup(&self, lexema: String) -> Option<Tipo_Token> {
        self.global.lookup(lexema, self.escopo_atual)
    }

    pub fn lista_params(&self, lexema: String) -> Option<Vec<Tipo_Token>> {
        self.global.lista_params(lexema)
    }
}
