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
        self.global.add_simbolo_no_indice(0, s);
        return self.indice_entrada - 1;
    }

    pub fn add_simbolo(&mut self, s: Simbolo) -> usize {

        self.indice_entrada += 1;

        // TODO: verificar em escopos pai

        self.global.add_simbolo_no_indice(self.escopo_atual, s);

        return self.indice_entrada - 1;
    }

    pub fn printar(&self) {
        self.global.printar();
    }
}
