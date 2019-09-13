use crate::tabela_simbolos::simbolo::*;


#[derive(Clone,PartialEq)]
pub struct Escopo {

    entradas : Vec<Simbolo>,
    subescopos : Vec<Escopo>,
    escopo_pai : Option<*mut Escopo>,

}

impl Escopo {

    pub fn novo() -> Escopo {
        Escopo {
            entradas : Vec::new(),
            subescopos : Vec::new(),
            escopo_pai : None
        }
    }

    pub fn add_entrada(&mut self, t : Simbolo) {
        self.entradas.push(t);
    }

    pub fn add_escopo(&mut self, mut e : Escopo) {
        e.escopo_pai = Some(self);
        self.subescopos.push(e);
    }

    pub fn pegar(&self, indice : u64) -> Option<Simbolo> {
        for i in self.entradas.clone() {
            if i.indice() == indice {
                return Some(i);
            }
        }

        for i in self.subescopos.clone() {
            match i.pegar(indice) {
                Some(s) => { return Some(s); }
                None => {}
            }
        }

        None
    }

    pub fn atualizar(&mut self, indice : u64, simbolo : Simbolo) -> bool {

        for i in 0..self.entradas.len() {
            if self.entradas[i].indice() == indice {
                self.entradas[i] = simbolo;
                return true;
            }
        }

        for i in 0..self.subescopos.len() {
            if self.subescopos[i].atualizar(indice, simbolo.clone()) == true {
                return true;
            }
        }

        false
    }
}

